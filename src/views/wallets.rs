use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};

use crate::models::_entities::wallets;

#[derive(Debug, Deserialize, Serialize)]
pub struct WalletResponse {
    pub addr: String,
    pub balance: Decimal,
    pub status: i8,
}
impl WalletResponse {
    pub fn new(model: &wallets::Model) -> Self {
        WalletResponse {
            addr: model.addr.clone(),
            balance: model.balance.clone(),
            status: model.status,
        }
    }
}
