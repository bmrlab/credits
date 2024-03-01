use crate::models::{_entities::transaction_events, transaction_event_type};
use sea_orm::{prelude::Decimal, Set};
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TransItem {
    pub from_addr: String,
    pub to_addr: String,
    pub amount: Decimal,
    pub event_type: String,
    pub info: serde_json::Value,
}

impl TransItem {
    pub fn new(&self) -> transaction_events::ActiveModel {
        let event_type = self.event_type.clone();
        transaction_event_type::check_event_type(&event_type);
        transaction_events::ActiveModel {
            from_addr: Set(Some(self.from_addr.clone())),
            to_addr: Set(Some(self.to_addr.clone())),
            amount: Set(self.amount.clone()),
            state: Set(10),
            event_type: Set(event_type),
            info: Set(Some(self.info.clone())),
            ..Default::default()
        }
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct TransactionResp {
    pub event_id: String,
}

impl TransactionResp {
    pub fn new(event_id: String) -> Self {
        Self { event_id }
    }
}
