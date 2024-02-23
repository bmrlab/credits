use loco_rs::schema::bool;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(Wallets::Table)
                    .col(pk_auto(Wallets::Id))
                    .col(string(Wallets::Addr))
                    .col(decimal_len(Wallets::Balance, 22, 2))
                    .col(bool(Wallets::Status))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Wallets::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Wallets {
    Table,
    Id,
    Addr,
    Balance,
    Status,
}
