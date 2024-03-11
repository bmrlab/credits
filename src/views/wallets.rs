#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unused_async)]
use chrono::Utc;
use loco_rs::Result;
use sea_orm::{prelude::Decimal, Set};
use serde::{Deserialize, Serialize};

use crate::models::_entities::wallet;

use super::params_error;

#[derive(Debug, Deserialize, Serialize)]
pub struct WalletResponse {
    pub addr: String,
    pub balance: Decimal,
    pub state: i8,
}
impl WalletResponse {
    pub fn new(model: &wallet::Model) -> Self {
        Self {
            addr: model.addr.clone(),
            balance: model.balance.clone(),
            state: model.state,
        }
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateWallet {
    pub addr: String,
    pub balance: Option<Decimal>,
    pub state: Option<i8>,
}

impl UpdateWallet {
    pub fn update_balance(&self, model: &mut wallet::ActiveModel) -> Result<()> {
        model.balance = Set(self
            .balance
            .ok_or_else(|| params_error("balance is null".to_string()))?
            .clone());
        model.updated_at = Set(Utc::now());
        Ok(())
    }

    pub fn update_state(&self, model: &mut wallet::ActiveModel) -> Result<()> {
        model.state = Set(self
            .state
            .ok_or_else(|| params_error("status is null".to_string()))?
            .clone());
        model.updated_at = Set(Utc::now());
        Ok(())
    }
}
