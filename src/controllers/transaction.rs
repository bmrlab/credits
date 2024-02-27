#![allow(clippy::unused_async)]
use loco_rs::prelude::*;
use redis::AsyncCommands;
use serde_json::json;

use crate::views::transaction::{TransactionInExecute, TransactionResp};

use super::QUEUE_KEY;

pub async fn echo(req_body: String) -> String {
    req_body
}

pub async fn api_exec_trans(
    State(ctx): State<AppContext>,
    Json(params): Json<TransactionInExecute>,
) -> Result<Json<TransactionResp>> {
    let mut event_ids: Vec<String> = vec![];

    for item in &params.trans {
        let event_id = get_uuid();
        let active_mode = item.new(&event_id);
        event_ids.push(event_id.clone());
        active_mode.insert(&ctx.db).await?;
    }

    let pool = &ctx.redis.ok_or_else(|| Error::NotFound)?.clone();
    let mut pool_coon = pool.get().await?;
    let redis_coon = pool_coon.unnamespaced_borrow_mut();
    let msg = json!(&event_ids).to_string();
    tracing::info!("redis lpush msg={}", &msg);
    redis_coon.lpush(QUEUE_KEY, msg).await?;

    format::json(TransactionResp::new(event_ids))
}

fn get_uuid() -> String {
    let uuid = uuid::Uuid::new_v4();
    uuid.to_string()
}

// pub async fn hello(State(ctx): State<AppContext>) -> Result<String> {
//     let pool = &ctx.redis.ok_or_else(|| Error::NotFound)?.clone();
//     let mut pool_coon = pool.get().await?;
//     let coon = pool_coon.unnamespaced_borrow_mut();

//     let kang: String = coon.set_ex("kang", "bbb", 1000).await?;
//     println!("kang = {}", &kang);
//     coon.lpush("aa", "1").await?;
//     coon.lpush("aa", "1").await?;

//     let len: i32 = coon.llen("aa").await?;
//     println!(" len {}", len);

//     loop {
//         let temp = pool_coon.unnamespaced_borrow_mut();
//         let value: Option<redis::Value> =
//             temp.rpop("aa", Some(NonZeroUsize::new(1).unwrap())).await?;

//         if let Some(redis::Value::Bulk(values)) = value {
//             println!("Bulk values:");
//             for v in values {
//                 match v {
//                     redis::Value::Data(data) => {
//                         println!("  Binary data: {}", String::from_utf8_lossy(&data))
//                     }
//                     _ => println!("  Other value: {:?}", v),
//                 }
//             }
//         } else {
//             println!("Option is None");
//         }

//         // if let Some(value) = cc {
//         //     println!("cc value={}", value);
//         // } else {
//         //     println!("cc is null");
//         // }
//         let next: i32 = temp.llen("aa").await?;
//         println!(" loop len ={}", next);
//         if next == 0 {
//             break;
//         }
//     }

//     // let cmd = coon.cmd_with_key("get", "kang".to_string());

//     // let res = cmd.query_async(coon.unnamespaced_borrow_mut());

//     // let res = coon
//     //     .set_nx_ex("kang".to_string(), "1".to_string(), 10)
//     //     .await;
//     // let value = res.unwrap();

//     // println!("value {:?}", value);

//     // do something with context (database, etc)
//     format::text("hello")
// }

pub fn routes() -> Routes {
    Routes::new()
        .prefix("transaction")
        .add("/", post(api_exec_trans))
        .add("/echo", post(echo))
}
