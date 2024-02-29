use crate::models::{_entities::transaction_events, transaction_event_type};
use sea_orm::{prelude::Decimal, Set};
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
#[derive(Debug, Deserialize, Serialize)]
pub struct TransItem {
    pub from_addr: String,
    pub to_addr: String,
    pub amount: Decimal,
    pub event_type: String,
    pub info: Json,
}

impl TransItem {
    pub fn new(&self, event_id: &String) -> transaction_events::ActiveModel {
        let event_type = self.event_type.clone();
        transaction_event_type::check_event_type(&event_type);
        transaction_events::ActiveModel {
            from_addr: Set(Some(self.from_addr.clone())),
            to_addr: Set(Some(self.to_addr.clone())),
            event_id: Set(event_id.clone()),
            amount: Set(self.amount.clone()),
            event_type: Set(event_type),
            info: Set(Some(json!(self.info.clone()))),
            ..Default::default()
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TransactionInExecute {
    pub trans: Vec<TransItem>,
    pub callback_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TransactionResp {
    pub event_ids: Vec<String>,
}

impl TransactionResp {
    pub fn new(event_ids: Vec<String>) -> Self {
        Self { event_ids }
    }
}
