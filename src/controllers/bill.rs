#![allow(clippy::unused_async)]
use axum::http::StatusCode;
use loco_rs::{controller::ErrorDetail, prelude::*};
use sea_orm::{ColumnTrait, Condition, QueryFilter};

use crate::{
    models::{
        _entities::{bill, prelude::Bill},
        time_util::string_to_date_time,
    },
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

    let sql = Bill::find().filter(
        Condition::all()
            .add(bill::Column::FromAddr.eq(&params.addr))
            .add(bill::Column::CreatedAt.gte(string_to_date_time(&start_time)))
            .add(bill::Column::CreatedAt.lte(string_to_date_time(&end_time))),
    );
    // tracing::info!("sql = {}", &sql.build(DbBackend::MySql).to_string());
    let bill_res = sql.all(&ctx.db).await?;
    let mut res = Vec::new();
    tracing::info!("bill_res={:?}", &bill_res);
    bill_res.iter().for_each(|ele| {
        res.push(BillResponse::new(ele));
    });

    format::json(ModelResp::success(res))
}

pub fn routes() -> Routes {
    Routes::new().prefix("bill").add("/history", post(history))
}
