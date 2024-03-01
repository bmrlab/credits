#![allow(clippy::unused_async)]
use axum::http::StatusCode;
use loco_rs::{controller::ErrorDetail, prelude::*};
use sea_orm::{ColumnTrait, ConnectionTrait, QueryFilter, Statement, TransactionTrait};
use serde_json::json;

use crate::{
    models::{
        _entities::{prelude::*, *},
        transaction_event_type::{self, INCOME, PAYMENT},
    },
    views::transaction::{TransItem, TransactionResp},
};

pub async fn echo(req_body: String) -> String {
    req_body
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
pub async fn api_exec_trans(
    State(ctx): State<AppContext>,
    Json(params): Json<TransItem>,
) -> Result<Json<TransactionResp>> {
    let params_clone = params.clone();
    let from_addr = params_clone.from_addr;
    let to_addr = params_clone.to_addr;
    let amount = params_clone.amount;
    let direction = transaction_event_type::get_direction(&params_clone.event_type);
    let event_id = get_uuid();

    loop {
        // 1.获取两个钱包的信息。
        // let tran = params.new(&event_id);
        let from_wallet = Wallets::find()
            .filter(wallets::Column::Addr.eq(&from_addr))
            .one(&ctx.db)
            .await?;

        let to_wallet = Wallets::find()
            .filter(wallets::Column::Addr.eq(&to_addr))
            .one(&ctx.db)
            .await?;

        // 2.判断两个钱包余额是否支持交易，不支持，交易失败，记录事件交易信息。交易结束。
        let mut transaction_active: transaction_events::ActiveModel =
            build_transaction(&from_wallet, &to_wallet, &params, direction);
        transaction_active.event_id = Set(event_id.clone());
        let state_value = transaction_active.clone().state.as_ref().clone();

        let msg: String = transaction_active
            .clone()
            .status_msg
            .as_ref()
            .clone()
            .ok_or_else(|| Error::NotFound)?;
        if state_value < 0 {
            transaction_active.insert(&ctx.db).await?;
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

        // 4.开始事务：更新钱包余额条件加上原有钱包余额，两个钱包更新余额成功，则提交事务，否则事务回滚，进行交易重试，从1开始
        let txn = ctx.db.begin().await?;
        let from_res = &txn
            .execute(Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::MySql,
                "update wallets set balance = ? where addr = ? and balance = ?",
                [
                    from_balance.into(),
                    from_addr.clone().into(),
                    from_wallet_model.balance.into(),
                ],
            ))
            .await?;
        let to_res = &txn
            .execute(Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::MySql,
                "update wallets set balance = ? where addr = ? and balance = ?",
                [
                    to_balance.into(),
                    to_addr.clone().into(),
                    to_wallet_model.balance.into(),
                ],
            ))
            .await?;
        if from_res.rows_affected().eq(&1) && to_res.rows_affected().eq(&1) {
            txn.commit().await?;
            // 5.事件交易信息，两个钱包的账单信息存储。
            transaction_active.state = Set(10);
            let transaction_event_type = transaction_active.insert(&ctx.db).await?;
            println!("transaction_event_type {:?}", transaction_event_type);
            // 6.交易结束。
            break;
        } else {
            txn.rollback().await?
        }
    }

    // let mut event_ids: Vec<String> = vec![];

    // for item in &params.trans {
    //     let event_id = get_uuid();
    //     let active_mode = item.new(&event_id);
    //     event_ids.push(event_id.clone());
    //     active_mode.insert(&ctx.db).await?;
    // }

    // let pool = &ctx.redis.ok_or_else(|| Error::NotFound)?.clone();
    // let mut pool_coon = pool.get().await?;
    // let redis_coon = pool_coon.unnamespaced_borrow_mut();
    // let msg = json!(&event_ids).to_string();
    // tracing::info!("redis lpush msg={}", &msg);
    // redis_coon.lpush(QUEUE_KEY, msg).await?;

    format::json(TransactionResp::new(event_id))
}

fn get_uuid() -> String {
    let uuid = uuid::Uuid::new_v4();
    uuid.to_string()
}

fn build_transaction(
    from_wallet: &Option<wallets::Model>,
    to_wallet: &Option<wallets::Model>,
    param: &TransItem,
    direction: i8,
) -> transaction_events::ActiveModel {
    let mut tran_mode_active: transaction_events::ActiveModel = param.new();
    tran_mode_active.direction = Set(direction);

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

// pub async fn hello(State(ctx): State<AppContext>) -> Result<String> {
//     let pool = &ctx.redis.ok_or_else(|| Error::NotFound)?.clone();
//     let mut pool_coon = pool.get().await?;
//     let coon = pool_coon.unnamespaced_borrow_mut();

//     let kang: String = coon.set_ex("kang", "bbb", 1000).await?;
//     println!("kang = {}", &kang);
//     coon.lpush("aa", "1").await?;
//     coon.lpush("aa", "1").await?;

//     let len: i32 = coon.llen("aa").await?;
//     println!(" len {}", len);

//     loop {
//         let temp = pool_coon.unnamespaced_borrow_mut();
//         let value: Option<redis::Value> =
//             temp.rpop("aa", Some(NonZeroUsize::new(1).unwrap())).await?;

//         if let Some(redis::Value::Bulk(values)) = value {
//             println!("Bulk values:");
//             for v in values {
//                 match v {
//                     redis::Value::Data(data) => {
//                         println!("  Binary data: {}", String::from_utf8_lossy(&data))
//                     }
//                     _ => println!("  Other value: {:?}", v),
//                 }
//             }
//         } else {
//             println!("Option is None");
//         }

//         // if let Some(value) = cc {
//         //     println!("cc value={}", value);
//         // } else {
//         //     println!("cc is null");
//         // }
//         let next: i32 = temp.llen("aa").await?;
//         println!(" loop len ={}", next);
//         if next == 0 {
//             break;
//         }
//     }

//     // let cmd = coon.cmd_with_key("get", "kang".to_string());

//     // let res = cmd.query_async(coon.unnamespaced_borrow_mut());

//     // let res = coon
//     //     .set_nx_ex("kang".to_string(), "1".to_string(), 10)
//     //     .await;
//     // let value = res.unwrap();

//     // println!("value {:?}", value);

//     // do something with context (database, etc)
//     format::text("hello")
// }

pub fn routes() -> Routes {
    Routes::new()
        .prefix("transaction")
        .add("/", post(api_exec_trans))
        .add("/echo", post(echo))
}
