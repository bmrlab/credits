use std::num::NonZeroUsize;

use axum::{async_trait, Extension, Router as AxumRouter};
use chrono::{Local, TimeZone};
use loco_rs::{
    app::{AppContext, Initializer},
    Error, Result,
};
use redis::AsyncCommands;

use crate::controllers::QUEUE_KEY;

pub struct ListenTranInitializer;

#[async_trait]
impl Initializer for ListenTranInitializer {
    fn name(&self) -> String {
        "listen-tran".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
        sub_transaction(ctx).await?;
        Ok(router)
    }
}

async fn sub_transaction(ctx: &AppContext) -> Result<()> {
    let pool: loco_rs::prelude::worker::Pool<loco_rs::prelude::worker::RedisConnectionManager> =
        ctx.redis.as_ref().ok_or_else(|| Error::NotFound)?.clone();
    tokio::spawn(tran(pool));
    Ok(())
}

async fn tran(
    pool: loco_rs::prelude::worker::Pool<loco_rs::prelude::worker::RedisConnectionManager>,
) {
    let mut pool_coon = pool.get().await.unwrap();
    let coon: &mut redis::aio::Connection = pool_coon.unnamespaced_borrow_mut();
    loop {
        // 计算消费时间
        let start_time = Local::now();
        println!("开始消费");
        std::thread::sleep(std::time::Duration::from_secs(5));
        let value: Option<redis::Value> = coon
            .rpop(QUEUE_KEY, Some(NonZeroUsize::new(1).unwrap()))
            .await
            .unwrap();

        println!("value={:?}", &value);

        if let Some(redis::Value::Bulk(values)) = value {
            println!("Bulk values:");
            for v in values {
                match v {
                    redis::Value::Data(data) => {
                        println!("  Binary data: {}", String::from_utf8_lossy(&data))
                    }
                    _ => println!("  Other value: {:?}", v),
                }
            }
        } else {
            println!("Option is None");
        }

        println!(
            "开始结束 消耗时间 {:?}",
            Local::now().signed_duration_since(start_time)
        );
    }
}
