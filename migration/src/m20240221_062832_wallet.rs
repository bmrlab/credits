use loco_rs::schema::bool;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(Wallet::Table)
                    .col(pk_auto(Wallet::Id))
                    .col(string(Wallet::Addr))
                    .col(decimal_len(Wallet::Balance, 22, 2))
                    .col(bool(Wallet::State))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Wallet::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Wallet {
    Table,
    Id,
    Addr,
    Balance,
    State,
}
