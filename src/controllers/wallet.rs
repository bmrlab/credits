#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unused_async)]
use crate::models::_entities::prelude::*;
use crate::models::_entities::wallets;
use crate::views::response::ModelResp;
use crate::views::wallets::UpdateWallet;
use crate::views::wallets::WalletResponse;
use loco_rs::prelude::*;
use sea_orm::prelude::Decimal;
use sea_orm::{ColumnTrait, QueryFilter};

// 获取钱包信息
pub async fn get_one(
    Path(addr): Path<String>,
    State(ctx): State<AppContext>,
) -> Result<Json<ModelResp<WalletResponse>>> {
    let base = Wallets::find()
        .filter(wallets::Column::Addr.eq(&addr))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;
    format::json(success(&base))
}

// 创建钱包
pub async fn create_addr(State(ctx): State<AppContext>) -> Result<Json<ModelResp<WalletResponse>>> {
    let active_model = wallets::ActiveModel {
        addr: Set(get_addr()),
        balance: Set(Decimal::new(0, 2)),
        status: Set(1),
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
    let mut base: wallets::Model = Wallets::find()
        .filter(wallets::Column::Addr.eq(&params.addr))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;
    let mut active_model = base.into_active_model();
    params.update_balance(&mut active_model);
    tracing::info!("active---- info {:?}", &active_model);
    base = active_model.update(&ctx.db).await?;
    format::json(success(&base))
}

fn success(base: &wallets::Model) -> ModelResp<WalletResponse> {
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
        .add("/", post(create_addr))
        .add("/:addr", get(get_one))
}
