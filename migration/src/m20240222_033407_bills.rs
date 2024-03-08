use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(Bill::Table)
                    .col(pk_auto(Bill::Id))
                    .col(string(Bill::EventId))
                    .col(string_null(Bill::FromAddr))
                    .col(string_null(Bill::ToAddr))
                    .col(string(Bill::EventType).comment(
                        "- 支付事件：payment
                    - 转账事件：transfer
                    - 发放奖励：distribute
                    - 正常扣款：deduction
                    - 罚款：fine",
                    ))
                    .col(tiny_integer(Bill::Direction))
                    .col(decimal_len(Bill::Amount, 22, 2))
                    .col(decimal_len(Bill::CurrentBalance, 22, 2))
                    .col(string_null(Bill::ExtDesc))
                    .col(json_null(Bill::Info))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Bill::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Bill {
    Table,
    Id,
    EventId,
    FromAddr,
    ToAddr,
    EventType,
    Direction,
    Amount,
    CurrentBalance,
    ExtDesc,
    Info,
}
