#![allow(clippy::unused_async)]
use std::collections::HashMap;

use axum::{extract::Query, http::StatusCode};
use chrono::Utc;
use loco_rs::{controller::ErrorDetail, prelude::*};
use sea_orm::{
    prelude::Decimal, ColumnTrait, ConnectionTrait, QueryFilter, Statement, TransactionTrait,
};
use serde_json::json;

use crate::{
    models::{
        _entities::{prelude::*, *},
        transaction_event_type::{self, INCOME, PAYMENT, TE_TYPE_RECOVERY},
    },
    views::{
        params_error,
        response::ModelResp,
        transaction::{
            RecoveryInExecute, TransItem, TransItemBatchReq, TransactionDetailResp,
            TransactionsResp,
        },
    },
};

pub async fn api_exec_recovery(
    State(ctx): State<AppContext>,
    Json(params): Json<RecoveryInExecute>,
) -> Result<Json<ModelResp<TransactionsResp>>> {
    let param = params.convert_to_trans_item();
    let event_id = transation_process(&ctx.db, param, params.trace_id).await?;
    format::json(ModelResp::success(TransactionsResp::new(vec![event_id])))
}

pub async fn api_exec_batch_trans(
    State(ctx): State<AppContext>,
    Json(params): Json<TransItemBatchReq>,
) -> Result<Json<ModelResp<TransactionsResp>>> {
    let trace_id = params.trace_id;
    let mut res: Vec<String> = Vec::new();
    for ele in params.trans {
        let event_id = transation_process(&ctx.db, ele, trace_id.clone()).await?;
        res.push(event_id);
    }
    format::json(ModelResp::success(TransactionsResp::new(res)))
}

pub async fn api_query_event(
    State(ctx): State<AppContext>,
    Query(map): Query<HashMap<String, String>>,
) -> Result<Json<ModelResp<Vec<TransactionDetailResp>>>> {
    let event_id = map
        .get("event_id")
        .ok_or_else(|| params_error("event_id is empty".to_string()))?
        .clone();
    let models = transaction_events::Entity::find()
        .filter(transaction_events::Column::TraceId.eq(&event_id))
        .all(&ctx.db)
        .await?;
    let res = models
        .iter()
        .map(|model| TransactionDetailResp::new(model))
        .collect::<Vec<TransactionDetailResp>>();
    format::json(ModelResp::success(res))
}

pub async fn api_query_event_by_trace_id(
    State(ctx): State<AppContext>,
    Query(map): Query<HashMap<String, String>>,
) -> Result<Json<ModelResp<Vec<TransactionDetailResp>>>> {
    let trace_id = map
        .get("trace_id")
        .ok_or_else(|| params_error("trace_id is empty".to_string()))?
        .clone();
    let models = transaction_events::Entity::find()
        .filter(transaction_events::Column::TraceId.eq(&trace_id))
        .all(&ctx.db)
        .await?;
    let res = models
        .iter()
        .map(|model| TransactionDetailResp::new(model))
        .collect::<Vec<TransactionDetailResp>>();
    format::json(ModelResp::success(res))
}

/*
发起交易请求：
1.获取两个钱包的信息。
2.判断两个钱包余额是否支持交易，不支持，交易失败，记录事件交易信息。交易结束。
3.钱包进行计算逻辑。
4.开始事务：更新钱包余额条件加上原有钱包余额，两个钱包更新余额成功，则提交事务，否则事务回滚，进行交易重试，从1开始
5.事件交易信息，两个钱包的账单信息存储。
6.交易结束。
*/
async fn transation_process(
    db: &sea_orm::prelude::DatabaseConnection,
    params: TransItem,
    trace_id: String,
) -> Result<String> {
    tracing::info!("发起交易 params: {:?}", &params);
    let params_clone = params.clone();
    let from_addr = params_clone.from_addr;
    let to_addr = params_clone.to_addr;
    let mut amount = params_clone.amount;
    let events_type = transaction_event_type::split_complex_event(&params_clone.event_type);
    let event_id = get_uuid();
    tracing::info!(
        "交易开始 event_id: {} events_type: {:?}",
        &event_id,
        &events_type
    );
    for ele in events_type {
        let direction = transaction_event_type::get_direction(ele);

        loop {
            // 1.获取两个钱包的信息。
            // let tran = params.new(&event_id);
            let from_wallet = Wallets::find()
                .filter(wallets::Column::Addr.eq(&from_addr))
                .one(db)
                .await?;

            let to_wallet = Wallets::find()
                .filter(wallets::Column::Addr.eq(&to_addr))
                .one(db)
                .await?;

            // 2.判断两个钱包余额是否支持交易，不支持，交易失败，记录事件交易信息。交易结束。
            let mut transaction_active: transaction_events::ActiveModel = build_transaction(
                &from_wallet,
                &to_wallet,
                &params,
                direction,
                trace_id.clone(),
            );
            // 金额回收 逻辑
            if ele == TE_TYPE_RECOVERY {
                amount = to_wallet
                    .as_ref()
                    .ok_or_else(|| Error::NotFound)?
                    .balance
                    .clone();
                transaction_active.amount = Set(amount.clone());
            }
            transaction_active.event_id = Set(event_id.clone());
            let state_value = transaction_active.clone().state.as_ref().clone();

            if state_value < 0 {
                let msg: String = transaction_active
                    .clone()
                    .status_msg
                    .as_ref()
                    .clone()
                    .ok_or_else(|| Error::NotFound)?;
                transaction_active.insert(db).await?;
                return Err(Error::CustomError(
                    StatusCode::BAD_REQUEST,
                    ErrorDetail::new("bad_request", &msg),
                ));
            }

            // 3.钱包进行计算逻辑。
            let from_wallet_model = from_wallet.ok_or_else(|| Error::NotFound)?;
            let mut from_balance = from_wallet_model.balance.clone();

            let to_wallet_model = to_wallet.ok_or_else(|| Error::NotFound)?;
            let mut to_balance = to_wallet_model.balance.clone();

            if direction == PAYMENT {
                from_balance -= amount;
                to_balance += amount;
            } else if direction == INCOME {
                from_balance += amount;
                to_balance -= amount;
            }
            let now = Utc::now();
            // 4.开始事务：更新钱包余额条件加上原有钱包余额，两个钱包更新余额成功，则提交事务，否则事务回滚，进行交易重试，从1开始
            let txn = db.begin().await?;
            let from_res = &txn
                .execute(Statement::from_sql_and_values(
                    sea_orm::DatabaseBackend::MySql,
                    "update wallets set balance = ?, updated_at=? where addr = ? and balance = ?",
                    [
                        from_balance.into(),
                        now.into(),
                        from_addr.clone().into(),
                        from_wallet_model.balance.into(),
                    ],
                ))
                .await?;
            let to_res = &txn
                .execute(Statement::from_sql_and_values(
                    sea_orm::DatabaseBackend::MySql,
                    "update wallets set balance = ?,  updated_at=?  where addr = ? and balance = ?",
                    [
                        to_balance.into(),
                        now.into(),
                        to_addr.clone().into(),
                        to_wallet_model.balance.into(),
                    ],
                ))
                .await?;
            if from_res.rows_affected().eq(&1) && to_res.rows_affected().eq(&1) {
                tracing::info!("钱包积分转账成功 event_id: {}", &event_id);
                txn.commit().await?;
                // 5.事件交易信息，两个钱包的账单信息存储。
                transaction_active.state = Set(10);
                transaction_active.status_msg = Set(Some("success".to_string()));
                transaction_active.insert(db).await?;
                let bill_actives = build_bill_actives(
                    &event_id,
                    &from_addr,
                    from_wallet_model.balance.clone(),
                    &to_addr,
                    to_wallet_model.balance.clone(),
                    amount,
                    ele,
                    json!({}),
                );
                Bills::insert_many(bill_actives).exec(db).await?;
                // 6.交易结束。
                tracing::info!("交易成功 event_id: {}", &event_id);
                break;
            } else {
                tracing::info!("交易失败回滚 event_id: {}", &event_id);
                txn.rollback().await?
            }
        }
    }
    tracing::info!("交易结束 event_id: {}", &event_id);

    Ok(event_id)
}

fn get_uuid() -> String {
    let uuid = uuid::Uuid::new_v4();
    uuid.to_string()
}

fn build_bill_actives(
    event_id: &str,
    from_addr: &str,
    from_addr_amount: Decimal,
    to_addr: &str,
    to_addr_amount: Decimal,
    amount: Decimal,
    event_type: &str,
    info: serde_json::Value,
) -> Vec<bill::ActiveModel> {
    let mut bill_actives = vec![];
    let from_bill = bill::ActiveModel {
        event_id: Set(event_id.to_string()),
        from_addr: Set(Some(from_addr.to_string())),
        to_addr: Set(Some(to_addr.to_string())),
        amount: Set(amount.clone()),
        current_balance: Set(from_addr_amount.clone()),
        event_type: Set(event_type.to_string()),
        direction: Set(transaction_event_type::get_direction(event_type)),
        info: Set(Some(info.clone())),
        ..Default::default()
    };
    let to_bill = bill::ActiveModel {
        event_id: Set(event_id.to_string()),
        from_addr: Set(Some(to_addr.to_string())),
        to_addr: Set(Some(from_addr.to_string())),
        amount: Set(amount.clone()),
        current_balance: Set(to_addr_amount.clone()),
        event_type: Set(event_type.to_string()),
        direction: Set(transaction_event_type::get_opposite_direction(
            transaction_event_type::get_direction(event_type),
        )),
        info: Set(Some(info.clone())),
        ..Default::default()
    };
    bill_actives.push(from_bill);
    bill_actives.push(to_bill);
    bill_actives
}

fn build_transaction(
    from_wallet: &Option<wallets::Model>,
    to_wallet: &Option<wallets::Model>,
    param: &TransItem,
    direction: i8,
    trace_id: String,
) -> transaction_events::ActiveModel {
    let mut tran_mode_active: transaction_events::ActiveModel = param.new(trace_id);
    tran_mode_active.direction = Set(direction);
    tran_mode_active.state = Set(0);

    let from_addr = &param.from_addr;
    let to_addr = &param.to_addr;

    // 校验钱包是否为同一个
    if from_addr == to_addr {
        tran_mode_active.state = Set(-1);
        tran_mode_active.status_msg = Set(Some("钱包不能相同".to_string()));
        return tran_mode_active;
    }
    // 校验from_addr 钱包可用性
    if let Some(v) = from_wallet {
        if v.status == -1 {
            tran_mode_active.state = Set(-1);
            tran_mode_active.status_msg = Set(Some(format!("钱包: {} 不可用", from_addr)));
            return tran_mode_active;
        }
        if direction == -1 {
            if v.balance < param.amount {
                tran_mode_active.state = Set(-1);
                tran_mode_active.status_msg = Set(Some(format!("钱包: {} 余额不足", from_addr)));
                return tran_mode_active;
            }
        }
    } else {
        tran_mode_active.state = Set(-1);
        tran_mode_active.status_msg = Set(Some(format!("钱包: {} 不存在", from_addr.clone())));
        return tran_mode_active;
    }

    // 校验to_addr 钱包可用性
    if let Some(v) = to_wallet {
        if v.status == -1 {
            tran_mode_active.state = Set(-1);
            tran_mode_active.status_msg = Set(Some(format!("钱包: {} 不可用", to_addr)));
            return tran_mode_active;
        }
        if direction == 1 {
            if v.balance < param.amount {
                tran_mode_active.state = Set(-1);
                tran_mode_active.status_msg = Set(Some(format!("钱包: {} 余额不足", to_addr)));
                return tran_mode_active;
            }
        }
    } else {
        tran_mode_active.state = Set(-1);
        tran_mode_active.status_msg = Set(Some(format!("钱包: {} 不存在", to_addr)));
        return tran_mode_active;
    }

    tran_mode_active
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("trans")
        .add("/", post(api_exec_batch_trans))
        .add("/", get(api_query_event))
        .add("/trace", get(api_query_event_by_trace_id))
        .add("/recovery", post(api_exec_recovery))
}
