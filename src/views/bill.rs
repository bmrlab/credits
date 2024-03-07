use sea_orm::prelude::{DateTimeUtc, Decimal};
use serde::{Deserialize, Serialize};

use crate::models::{_entities::bills, time_util};

#[derive(Debug, Deserialize, Serialize)]
pub struct BillQueryParams {
    pub addr: String,
    pub start_time: String,
    pub end_time: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BillResponse {
    pub id: i32,
    pub event_id: String,
    pub from_addr: String,
    pub to_addr: String,
    pub amount: Decimal,
    pub event_type: String,
    pub created_at: DateTimeUtc,
    pub direction: i8,
    pub info: serde_json::Value,
}

impl BillResponse {
    pub fn new(model: &bills::Model) -> Self {
        Self {
            id: model.id,
            event_id: model.event_id.clone(),
            from_addr: model.from_addr.clone().unwrap(),
            to_addr: model.to_addr.clone().unwrap(),
            amount: model.amount.clone(),
            direction: model.direction.clone(),
            event_type: model.event_type.clone(),
            info: model.info.clone().unwrap(),
            created_at: time_util::gtm_time(model.created_at.clone()),
        }
    }
}
