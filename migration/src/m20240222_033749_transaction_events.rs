use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(TransactionEvents::Table)
                    .col(pk_auto(TransactionEvents::Id))
                    .col(string(TransactionEvents::EventId))
                    .col(string(TransactionEvents::EventExecId))
                    .col(string_null(TransactionEvents::FromAddr))
                    .col(string_null(TransactionEvents::ToAddr))
                    .col(decimal_len(TransactionEvents::Amount, 22, 2))
                    .col(string(TransactionEvents::EventType))
                    .col(tiny_integer(TransactionEvents::Direction))
                    .col(json_null(TransactionEvents::Info))
                    .col(small_integer(TransactionEvents::State))
                    .col(string_null(TransactionEvents::StatusMsg))
                    .col(string_null(TransactionEvents::CallbackUrl))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TransactionEvents::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TransactionEvents {
    Table,
    Id,
    EventId,
    EventExecId,
    FromAddr,
    ToAddr,
    Amount,
    EventType,
    Direction,
    Info,
    State,
    StatusMsg,
    CallbackUrl,
}
