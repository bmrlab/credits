#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unused_async)]
use std::collections::HashMap;

use crate::models::_entities::prelude::*;
use crate::models::_entities::wallet;
use crate::views::params_error;
use crate::views::response::ModelResp;
use crate::views::wallets::UpdateWallet;
use crate::views::wallets::WalletResponse;
use axum::extract::Query;
use loco_rs::prelude::*;
use sea_orm::prelude::Decimal;
use sea_orm::{ColumnTrait, QueryFilter};

// 获取钱包信息
pub async fn get_one(
    Query(map): Query<HashMap<String, String>>,
    State(ctx): State<AppContext>,
) -> Result<Json<ModelResp<WalletResponse>>> {
    println!("map = {:?}", &map);
    let addr = map
        .get("addr")
        .ok_or_else(|| params_error("addr is empty".to_string()))?
        .clone();
    let base = Wallets::find()
        .filter(wallet::Column::Addr.eq(&addr))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;
    format::json(success(&base))
}

// 创建钱包
pub async fn create_addr(State(ctx): State<AppContext>) -> Result<Json<ModelResp<WalletResponse>>> {
    let active_model = wallet::ActiveModel {
        addr: Set(get_addr()),
        balance: Set(Decimal::new(0, 2)),
        state: Set(1),
        ..Default::default()
    };
    let base = active_model.insert(&ctx.db).await?;
    format::json(success(&base))
}

// 更新钱包积分
pub async fn update_balance(
    State(ctx): State<AppContext>,
    Json(params): Json<UpdateWallet>,
) -> Result<Json<ModelResp<WalletResponse>>> {
    let mut base: wallet::Model = Wallets::find()
        .filter(wallet::Column::Addr.eq(&params.addr))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;
    let mut active_model = base.into_active_model();
    params.update_balance(&mut active_model)?;
    base = active_model.update(&ctx.db).await?;
    format::json(success(&base))
}

// 更钱包状态
pub async fn update_status(
    State(ctx): State<AppContext>,
    Json(params): Json<UpdateWallet>,
) -> Result<Json<ModelResp<WalletResponse>>> {
    let mut base: wallet::Model = Wallets::find()
        .filter(wallet::Column::Addr.eq(&params.addr))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;
    let mut active_model = base.into_active_model();
    params.update_state(&mut active_model)?;
    tracing::info!("active---- info {:?}", &active_model);
    base = active_model.update(&ctx.db).await?;
    format::json(success(&base))
}

fn success(base: &wallet::Model) -> ModelResp<WalletResponse> {
    ModelResp::success(WalletResponse::new(base))
}

/*
生成一个随机钱包地址
0x开头 长度为42位的字符串
示例: 0xed761902880a3ce647c472c8d32c5fda13c0d235
*/
fn get_addr() -> String {
    let mut addr = "0x".to_string();
    let chars = "abcdef0123456789";
    for _ in 0..40 {
        let idx = rand::random::<usize>() % 16;
        addr.push(chars.chars().nth(idx).unwrap());
    }
    tracing::info!("生成的地址: {}", addr);
    addr
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("wallet")
        .add("/update_balance", post(update_balance))
        .add("/update_status", post(update_status))
        .add("/", post(create_addr))
        .add("/", get(get_one))
}
