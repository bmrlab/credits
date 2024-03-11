use serde::{Deserialize, Serialize};

use crate::models::_entities::transaction_event;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CallbackRequest {
    // success fail
    pub status: String,
    pub msg: String,
    pub events: Vec<transaction_event::Model>,
}

impl CallbackRequest {
    pub fn new(status: String, msg: String, events: Vec<transaction_event::Model>) -> Self {
        Self {
            status,
            msg,
            events,
        }
    }
}
