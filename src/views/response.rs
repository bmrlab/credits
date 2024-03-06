use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ModelResp<T> {
    pub data: T,
    pub status: u8,
    pub msg: String,
}

impl<T> ModelResp<T> {
    pub fn success(data: T) -> Self {
        Self {
            data,
            msg: "success".to_string(),
            status: 200,
        }
    }
}
