#![allow(clippy::unused_async)]
use axum::http::StatusCode;
use loco_rs::{controller::ErrorDetail, prelude::*};
use sea_orm::{prelude::DateTimeUtc, ColumnTrait, Condition, DbBackend, QueryFilter, QueryTrait};

use crate::{
    models::_entities::{bills, prelude::Bills},
    views::{
        bill::{BillQueryParams, BillResponse},
        response::ModelResp,
    },
};

pub async fn history(
    State(ctx): State<AppContext>,
    Json(params): Json<BillQueryParams>,
) -> Result<Json<ModelResp<Vec<BillResponse>>>> {
    let start_time = params.start_time;
    let end_time = params.end_time;
    if start_time > end_time {
        return Err(Error::CustomError(
            StatusCode::BAD_REQUEST,
            ErrorDetail::new("bad_request", "start_time > end_time"),
        ));
    };

    let sql = Bills::find().filter(
        Condition::all()
            .add(bills::Column::FromAddr.eq(&params.addr))
            .add(bills::Column::CreatedAt.gt(DateTimeUtc::from_timestamp_millis(start_time)))
            .add(bills::Column::CreatedAt.lt(DateTimeUtc::from_timestamp_millis(end_time))),
    );
    tracing::info!("sql = {}", &sql.build(DbBackend::MySql).to_string());
    let bill_res = sql.all(&ctx.db).await?;
    let mut res = Vec::new();
    bill_res.iter().for_each(|ele| {
        res.push(BillResponse::new(ele));
    });

    format::json(ModelResp::success(res))
}

pub fn routes() -> Routes {
    Routes::new().prefix("bill").add("/", post(history))
}
