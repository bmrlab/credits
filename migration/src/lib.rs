#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

pub struct Migrator;
mod m20240221_062832_wallets;
mod m20240221_064740_index_wallet_addr;
mod m20240222_033407_bills;
mod m20240222_033749_transaction_events;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240221_062832_wallets::Migration),
            Box::new(m20240221_064740_index_wallet_addr::Migration),
            Box::new(m20240222_033407_bills::Migration),
            Box::new(m20240222_033749_transaction_events::Migration),
        ]
    }
}
