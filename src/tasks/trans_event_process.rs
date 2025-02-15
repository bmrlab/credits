use std::collections::BTreeMap;

use chrono::{TimeZone, Utc};
use loco_rs::prelude::*;
use mongodb::{
    bson::{doc, Bson, Document},
    options::FindOptions,
    Client, Collection,
};
use sea_orm::prelude::Decimal;
use serde_json::Value as Json;

use crate::models::_entities::transaction_event;

pub struct TransEventProcess;
#[async_trait]
impl Task for TransEventProcess {
    fn task(&self) -> TaskInfo {
        TaskInfo {
            name: "trans_event_process".to_string(),
            detail: "Task generator".to_string(),
        }
    }
    async fn run(&self, ctx: &AppContext, vars: &BTreeMap<String, String>) -> Result<()> {
        println!("Task TransEventProcess generated vars={:?}", vars);

        let uri = vars.get("url").ok_or_else(|| Error::NotFound)?;

        println!("uri = {:?}", uri);
        let client = Client::with_uri_str(uri).await.unwrap();
        // Get a handle on the movies collection
        let database = client.database("muse-credits-billing");
        let options = FindOptions::builder().no_cursor_timeout(true).build();
        let my_coll: Collection<Document> = database.collection("transaction_event");
        let filter = doc! { "event_exec_id": { "$gt": 148384 } };
        let mut cursor = my_coll.find(filter, options).await.unwrap();
        while cursor.advance().await.unwrap() {
            let mut tran = transaction_event::ActiveModel {
                ..Default::default()
            };

            let doc = &cursor.deserialize_current().unwrap();

            println!("{:?}", doc);
            let mut id: u64 = 0;

            let event_exec_id_opt = doc.get("event_exec_id");
            if let Some(v) = event_exec_id_opt {
                if let Bson::Int32(r) = v {
                    id = r.clone() as u64;
                    tran.id = Set(r.to_owned() as u64);
                }
            }
            let temp = transaction_event::Entity::find_by_id(id)
                .one(&ctx.db)
                .await
                .unwrap();
            if let None = temp {
                let event_type_opt = doc.get("event_type");
                if let Some(event_type) = event_type_opt {
                    if let Bson::String(v) = event_type {
                        tran.event_type = Set(v.to_string());
                    }
                }

                let from_addr_opt = doc.get("from_addr");
                if let Some(from_addr) = from_addr_opt {
                    if let Bson::String(v) = from_addr {
                        tran.from_addr = Set(Some(v.to_string()));
                    }
                }

                let to_addr_opt = doc.get("to_addr");
                if let Some(to_addr) = to_addr_opt {
                    if let Bson::String(v) = to_addr {
                        tran.to_addr = Set(Some(v.to_string()));
                    }
                }

                let amount_opt = doc.get("amount");
                if let Some(amount) = amount_opt {
                    if let Bson::Double(v) = amount {
                        tran.amount = Set(Decimal::new((v * 100.0).round() as i64, 2));
                    }
                }

                let info_opt = doc.get("info");
                if let Some(info) = info_opt {
                    if let Bson::Document(v) = info {
                        tran.info = Set(Some(Json::from_iter(v.clone())));
                    }
                }

                let account_event_id_opt = doc.get("_id");
                if let Some(v) = account_event_id_opt {
                    if let Bson::String(r) = v {
                        tran.event_id = Set(r.to_string());
                    }
                }

                let trace_id_id_opt: Option<&Bson> = doc.get("trace_id");
                if let Some(v) = trace_id_id_opt {
                    if let Bson::String(r) = v {
                        tran.trace_id = Set(r.to_string());
                    }
                }

                let status_opt = doc.get("status");
                if let Some(v) = status_opt {
                    if let Bson::Int32(r) = v {
                        tran.state = Set(r.to_owned() as i16);
                    }
                }

                let callback_url_opt = doc.get("callback_url");
                if let Some(v) = callback_url_opt {
                    if let Bson::String(r) = v {
                        tran.callback_url = Set(Some(r.to_string()));
                    }
                }

                let status_msg_opt = doc.get("status_msg");
                if let Some(v) = status_msg_opt {
                    if let Bson::String(r) = v {
                        tran.status_msg = Set(Some(r.to_string()));
                    }
                }

                let created_at_opt = doc.get("created_at");
                if let Some(v) = created_at_opt {
                    if let Bson::DateTime(r) = v {
                        tran.created_at =
                            Set(Utc.timestamp_millis_opt(r.timestamp_millis()).unwrap());
                    }
                }
                let updated_at_opt = doc.get("updated_at");
                if let Some(v) = updated_at_opt {
                    if let Bson::DateTime(r) = v {
                        tran.updated_at =
                            Set(Utc.timestamp_millis_opt(r.timestamp_millis()).unwrap());
                    }
                }
                println!("需要存储的数据： {:?}", &tran);
                println!("开始转储------");
                tran.insert(&ctx.db).await.unwrap();
            }
        }
        println!("转储完成------");
        Ok(())
    }
}
