use loco_rs::app::AppContext;

pub mod transaction;
pub mod wallet;

// 队列key名称
pub const QUEUE_KEY: &str = "muse:credits-bill:transaction_queue";
// 当前锁key名称
pub const LOCK_KEY: &str = "muse:credits-bill:transaction_key";
// 当前锁key过期时间
pub const LOCK_KEY_TTL: i8 = 10;

// 当前交易信息key 防丢失
pub const LOCK_TRANSACTION_KEY: &str = "muse:credits-bill:transaction_key_lock";

// async fn sub_transaction(ctx: &AppContext) {
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
// }
