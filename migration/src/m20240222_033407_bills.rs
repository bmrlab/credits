use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(Bills::Table)
                    .col(pk_auto(Bills::Id))
                    .col(string(Bills::EventId))
                    .col(string_null(Bills::FromAddr))
                    .col(string_null(Bills::ToAddr))
                    .col(string(Bills::EventType).comment(
                        "- 支付事件：payment
                    - 转账事件：transfer
                    - 发放奖励：distribute
                    - 正常扣款：deduction
                    - 罚款：fine",
                    ))
                    .col(tiny_integer(Bills::Direction))
                    .col(decimal_len(Bills::Amount, 22, 2))
                    .col(decimal_len(Bills::CurrentBalance, 22, 2))
                    .col(string_null(Bills::ExtDesc))
                    .col(json_null(Bills::Info))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Bills::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Bills {
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
