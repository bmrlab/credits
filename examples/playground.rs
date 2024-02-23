use eyre::Context;
#[allow(unused_imports)]
use loco_rs::{cli::playground, prelude::*};
use muse_integrator::{
    app::App,
    models::_entities::wallets::{self, ActiveModel},
};
use sea_orm::prelude::Decimal;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let ctx = playground::<App>().await.context("playground")?;

    let active_model = ActiveModel {
        addr: Set("0xadsfasdf".to_string()),
        balance: Set(Decimal::new(2000, 2)),
        status: Set(1),
        ..Default::default()
    };
    active_model.insert(&ctx.db).await.unwrap();

    let res = wallets::Entity::find().all(&ctx.db).await.unwrap();
    println!("{:?}", res);

    Ok(())
}
