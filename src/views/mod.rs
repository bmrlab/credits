use axum::http::StatusCode;
use loco_rs::{controller::ErrorDetail, prelude::Error};

pub mod wallets;

pub mod response;

pub fn params_error(desc: String) -> Error {
    Error::CustomError(
        StatusCode::BAD_REQUEST,
        ErrorDetail::new("bad_request", &desc),
    )
}
