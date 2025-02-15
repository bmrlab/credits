use crate::models::{
    _entities::transaction_event,
    transaction_event_type::{self, TE_TYPE_RECOVERY},
};
use sea_orm::{prelude::Decimal, Set};
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TransItem {
    pub from_addr: String,
    pub to_addr: String,
    pub amount: Decimal,
    pub event_type: String,
    pub info: serde_json::Value,
}

impl TransItem {
    pub fn new(
        &self,
        trace_id: String,
        callback_url: String,
        amount: Decimal,
    ) -> transaction_event::ActiveModel {
        let event_type = self.event_type.clone();
        transaction_event_type::check_event_type(&event_type);
        transaction_event::ActiveModel {
            from_addr: Set(Some(self.from_addr.clone())),
            to_addr: Set(Some(self.to_addr.clone())),
            amount: Set(amount),
            trace_id: Set(trace_id.clone()),
            state: Set(10),
            callback_url: Set(Some(callback_url.clone())),
            event_type: Set(event_type),
            info: Set(Some(self.info.clone())),
            ..Default::default()
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TransItemBatchReq {
    pub trans: Vec<TransItem>,
    pub trace_id: String,
    pub callback_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TransactionsResp {
    pub event_ids: Vec<String>,
}
impl TransactionsResp {
    pub fn new(event_ids: Vec<String>) -> Self {
        Self { event_ids }
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RecoveryInExecute {
    pub from_addr: String,
    pub to_addr: String,
    pub trace_id: String,
    pub info: serde_json::Value,
    pub callback_url: String,
}

impl RecoveryInExecute {
    pub fn convert_to_trans_item(&self) -> TransItem {
        TransItem {
            from_addr: self.from_addr.clone(),
            to_addr: self.to_addr.clone(),
            amount: Decimal::new(0, 0),
            event_type: TE_TYPE_RECOVERY.to_string(),
            info: self.info.clone(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TransactionDetailResp {
    pub created_at: String,
    pub id: u64,
    pub event_id: String,
    pub from_addr: String,
    pub to_addr: String,
    pub amount: Decimal,
    pub event_type: String,
    pub direction: i8,
    pub info: Json,
    pub trace_id: String,
    pub status_msg: String,
}

impl TransactionDetailResp {
    pub fn new(model: &transaction_event::Model) -> Self {
        let mut status_msg = "";
        if let Some(msg) = &model.status_msg {
            status_msg = msg;
        };
        Self {
            created_at: model.created_at.to_string(),
            id: model.id,
            event_id: model.event_id.clone(),
            from_addr: model.from_addr.clone().unwrap(),
            to_addr: model.to_addr.clone().unwrap(),
            amount: model.amount.clone(),
            event_type: model.event_type.clone(),
            direction: model.direction,
            info: model.info.clone().unwrap(),
            trace_id: model.trace_id.clone(),
            status_msg: status_msg.to_string(),
        }
    }
}
