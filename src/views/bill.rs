use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};

use crate::models::_entities::bills;

#[derive(Debug, Deserialize, Serialize)]
pub struct BillQueryParams {
    pub addr: String,
    pub start_time: i64,
    pub end_time: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BillResponse {
    pub event_id: String,
    pub from_addr: String,
    pub to_addr: String,
    pub amount: Decimal,
    pub event_type: String,
    pub info: serde_json::Value,
}

impl BillResponse {
    pub fn new(model: &bills::Model) -> Self {
        Self {
            event_id: model.event_id.clone(),
            from_addr: model.from_addr.clone().unwrap(),
            to_addr: model.to_addr.clone().unwrap(),
            amount: model.amount.clone(),
            event_type: model.event_type.clone(),
            info: model.info.clone().unwrap(),
        }
    }
}
