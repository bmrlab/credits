// use std::num::NonZeroUsize;

// use axum::{async_trait, Router as AxumRouter};
// use chrono::Local;
// use loco_rs::{
//     app::{AppContext, Initializer},
//     Error, Result,
// };
// use redis::AsyncCommands;
// use sea_orm::{
//     prelude::Decimal, ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, IntoActiveModel,
//     QueryFilter, Set, TransactionTrait,
// };
// use serde_json::json;

// use crate::models::_entities::{prelude::*, wallets};
// use crate::{
//     controllers::{LOCK_KEY, LOCK_KEY_TTL, LOCK_TRANSACTION_KEY, QUEUE_KEY},
//     models::_entities::transaction_events,
// };

// pub struct ListenTranInitializer;

// #[async_trait]
// impl Initializer for ListenTranInitializer {
//     fn name(&self) -> String {
//         "listen-tran".to_string()
//     }

//     async fn after_routes(&self, router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
//         sub_transaction(ctx).await?;
//         Ok(router)
//     }
// }

// async fn sub_transaction(ctx: &AppContext) -> Result<()> {
//     let ctx_clone = ctx.clone();
//     tokio::spawn(tran(ctx_clone));
//     Ok(())
// }

// async fn tran(ctx: AppContext) -> Result<()> {
//     let pool = &ctx.redis.ok_or_else(|| Error::NotFound)?;
//     let db: &sea_orm::prelude::DatabaseConnection = &ctx.db;
//     let mut pool_coon = pool.get().await.unwrap();
//     let coon: &mut redis::aio::Connection = pool_coon.unnamespaced_borrow_mut();
//     println!("启动 redis queue : {} 监听", QUEUE_KEY);
//     loop {
//         let exist: bool = coon.exists(LOCK_KEY).await.unwrap();
//         if exist {
//             std::thread::sleep(std::time::Duration::from_millis(1000));
//         } else {
//             coon.set_ex(LOCK_KEY, 1, LOCK_KEY_TTL).await?;
//             println!("获取锁成功-------------");
//             // 先消耗当前缓存中的数据
//             let current: Option<redis::Value> = coon.get(LOCK_TRANSACTION_KEY).await.unwrap();

//             println!("current value {:?}", &current);

//             let value = current.or(coon
//                 .rpop(QUEUE_KEY, Some(NonZeroUsize::new(1).unwrap()))
//                 .await
//                 .unwrap());

//             println!("real value {:?}", &value);

//             if let Some(redis::Value::Bulk(values)) = value {
//                 println!("开始消费 values={:?}", &values);
//                 let start_time = Local::now();
//                 for v in values {
//                     if let redis::Value::Data(data) = v {
//                         let res: Vec<String> = serde_json::from_str(
//                             String::from_utf8_lossy(&data).to_string().as_str(),
//                         )
//                         .expect("Failed to parse JSON");

//                         coon.set(LOCK_TRANSACTION_KEY, json!(&res).to_string())
//                             .await?;

//                         // 数据处理
//                         transaction_detail(res, db).await.unwrap();
//                     }
//                 }

//                 println!(
//                     "结束消费 time {:?}",
//                     Local::now().signed_duration_since(start_time)
//                 );
//             }
//             coon.del(QUEUE_KEY).await?;
//             coon.del(LOCK_TRANSACTION_KEY).await?;
//             println!("释放锁成功------------------");
//         }
//     }
// }

// async fn transaction_detail(
//     event_ids: Vec<String>,
//     db: &sea_orm::prelude::DatabaseConnection,
// ) -> Result<()> {
//     println!("doing.........{:?}", event_ids);
//     let res = TransactionEvents::find()
//         .filter(transaction_events::Column::EventId.is_in(event_ids))
//         .all(db)
//         .await?;

//     for model in res {
//         let model_clone: transaction_events::Model = model.clone();
//         let from_wallet: Option<wallets::Model> = Wallets::find()
//             .filter(wallets::Column::Addr.eq(model.from_addr))
//             .one(db)
//             .await?;

//         let to_wallet: Option<wallets::Model> = Wallets::find()
//             .filter(wallets::Column::Addr.eq(model.to_addr))
//             .one(db)
//             .await?;

//         // 生成交易事件信息
//         let tran_mode_active =
//             build_transaction(from_wallet.clone(), to_wallet.clone(), model_clone)?;
//         let state_value = tran_mode_active.clone().state.unwrap();
//         if state_value == 10 {
//             // 生成入账账单信息
//             let mut wallets_active: Vec<wallets::ActiveModel> = vec![];
//             if model.direction == 1 {
//                 todo!();
//                 wallets_active.push(wallets::ActiveModel {
//                     addr: Set("adsfdas".to_string()),
//                     ..Default::default()
//                 });
//             } else {
//                 // 生成出账账单信息
//                 todo!();
//             }
//             // 账单交易信息
//             todo!();

//             db.transaction::<_, (), DbErr>(|txn| {
//                 Box::pin(async move {
//                     // 更新钱包
//                     for ele in wallets_active {
//                         ele.update(txn).await?;
//                     }

//                     // 更新交易事件
//                     tran_mode_active.update(txn).await?;
//                     Ok(())
//                 })
//             })
//             .await
//             .unwrap();
//         } else {
//             tran_mode_active.update(db).await?;
//         }
//     }

//     // let mut exist: bool = coon.exists(LOCK_KEY).await.unwrap();
//     // while exist {
//     //     println!("存在消费事务，尝试获取锁");
//     //     std::thread::sleep(std::time::Duration::from_millis(100));
//     //     exist = coon.exists(LOCK_KEY).await.unwrap();
//     // }
//     // coon.set_ex(LOCK_KEY, 1, LOCK_KEY_TTL).await?;
//     // coon.set(LOCK_TRANSACTION_KEY, json!(&event_ids).to_string())
//     // .await?;

//     Ok(())
// }

// fn build_wallet(
//     wallet: wallets::Model,
//     amount: Decimal,
//     direction: i8,
// ) -> Result<wallets::ActiveModel> {
//     if direction == 1 {
//         let wallet_clone = wallet.clone();
//         let mut active_model = wallet_clone.into_active_model();
//         active_model.balance = Set(wallet.balance - amount);
//         return Ok(active_model);
//     }

//     let wallet_clone = wallet.clone();
//     let mut active_model = wallet_clone.into_active_model();
//     active_model.balance = Set(wallet.balance + amount);
//     Ok(active_model)
// }

// fn build_transaction(
//     from_wallet: Option<wallets::Model>,
//     to_wallet: Option<wallets::Model>,
//     model: transaction_events::Model,
// ) -> Result<transaction_events::ActiveModel> {
//     let mut tran_mode_active: transaction_events::ActiveModel = model.clone().into_active_model();
//     let from_addr = model.from_addr.clone();
//     let to_addr = model.to_addr.clone();
//     tran_mode_active.state = Set(10);

//     // 校验钱包是否为同一个
//     if let (Some(from_value), Some(to_value)) = (model.from_addr.clone(), model.to_addr.clone()) {
//         if from_value == to_value {
//             tran_mode_active.state = Set(-1);
//             tran_mode_active.status_msg = Set(Some("钱包不能相同".to_string()));
//             return Ok(tran_mode_active);
//         }
//     }
//     // 校验钱包可用性
//     if let Some(v) = from_wallet {
//         if v.status == -1 {
//             tran_mode_active.state = Set(-1);
//             if let Some(value) = from_addr {
//                 tran_mode_active.status_msg = Set(Some(format!("钱包: {} 不可用", value.clone())));
//                 return Ok(tran_mode_active);
//             }
//         }
//         if model.direction == -1 {
//             if v.balance < model.amount {
//                 tran_mode_active.state = Set(-1);
//                 if let Some(value) = from_addr {
//                     tran_mode_active.status_msg =
//                         Set(Some(format!("钱包: {} 余额不足", value.clone())));
//                     return Ok(tran_mode_active);
//                 }
//             }
//         }
//     } else {
//         tran_mode_active.state = Set(-1);
//         if let Some(value) = from_addr {
//             tran_mode_active.status_msg = Set(Some(format!("钱包: {} 不存在", value.clone())));
//             return Ok(tran_mode_active);
//         }
//     }

//     if let Some(v) = to_wallet {
//         if v.status == -1 {
//             tran_mode_active.state = Set(-1);
//             if let Some(value) = to_addr {
//                 tran_mode_active.status_msg = Set(Some(format!("钱包: {} 不可用", value.clone())));
//                 return Ok(tran_mode_active);
//             }
//         }
//         if model.direction == 1 {
//             if v.balance < model.amount {
//                 tran_mode_active.state = Set(-1);
//                 if let Some(value) = from_addr {
//                     tran_mode_active.status_msg =
//                         Set(Some(format!("钱包: {} 余额不足", value.clone())));
//                     return Ok(tran_mode_active);
//                 }
//             }
//         }
//     } else {
//         tran_mode_active.state = Set(-1);
//         if let Some(value) = to_addr {
//             tran_mode_active.status_msg = Set(Some(format!("钱包: {} 不存在", value.clone())));
//             return Ok(tran_mode_active);
//         }
//     }

//     // 校验双方钱包余额是否充足

//     Ok(tran_mode_active)
// }
