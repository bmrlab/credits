#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unused_async)]
use loco_rs::Result;
use sea_orm::{prelude::Decimal, Set};
use serde::{Deserialize, Serialize};

use crate::models::_entities::wallets;

use super::params_error;

#[derive(Debug, Deserialize, Serialize)]
pub struct WalletResponse {
    pub addr: String,
    pub balance: Decimal,
    pub status: i8,
}
impl WalletResponse {
    pub fn new(model: &wallets::Model) -> Self {
        Self {
            addr: model.addr.clone(),
            balance: model.balance.clone(),
            status: model.status,
        }
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateWallet {
    pub addr: String,
    pub balance: Option<Decimal>,
    pub status: Option<i8>,
}

impl UpdateWallet {
    pub fn update_balance(&self, model: &mut wallets::ActiveModel) -> Result<()> {
        model.balance = Set(self
            .balance
            .ok_or_else(|| params_error("balance is null".to_string()))?
            .clone());
        Ok(())
    }

    pub fn update_state(&self, model: &mut wallets::ActiveModel) -> Result<()> {
        model.status = Set(self
            .status
            .ok_or_else(|| params_error("status is null".to_string()))?
            .clone());
        Ok(())
    }
}
