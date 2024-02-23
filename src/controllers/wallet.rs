#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unused_async)]
use crate::models::_entities::prelude::*;
use crate::models::_entities::wallets;
use crate::views::wallets::WalletResponse;
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, QueryFilter};

// 获取钱包信息
pub async fn get_one(
    Path(addr): Path<String>,
    State(ctx): State<AppContext>,
) -> Result<Json<WalletResponse>> {
    let base = Wallets::find()
        .filter(wallets::Column::Addr.eq(&addr))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| Error::NotFound)?;
    format::json(WalletResponse::new(&base))
}

pub fn routes() -> Routes {
    Routes::new().prefix("wallet").add("/:addr", get(get_one))
}
