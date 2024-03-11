use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                table_auto(TransactionEvent::Table)
                    .col(pk_auto(TransactionEvent::Id))
                    .col(string(TransactionEvent::EventId))
                    .col(string(TransactionEvent::TraceId))
                    .col(string_null(TransactionEvent::FromAddr))
                    .col(string_null(TransactionEvent::ToAddr))
                    .col(decimal_len(TransactionEvent::Amount, 22, 2).default(0))
                    .col(string(TransactionEvent::EventType).comment(
                        "金额流向 发起方 --> 接收方
                    - 支付事件：payment
                    - 转账事件：transfer
                    - 发放奖励：distribute
                    金额流向 发起方 <-- 接收方
                    - 正常扣款：deduction
                    - 金额回收: recovery
                    - 罚款：fine
                    复合操作 金额流向 接收方 --> 发起方 --> 接收方
                    - 带金额回收的奖励发放: distribute_with_recovery",
                    ))
                    .col(
                        tiny_integer(TransactionEvent::Direction)
                            .default(1)
                            .comment("交易方向：1 收入；-1 支出"),
                    )
                    .col(json_null(TransactionEvent::Info))
                    .col(
                        small_integer(TransactionEvent::State)
                            .default(0)
                            .comment("交易事件状态：0 交易开始；10 交易成功；-1 交易失败"),
                    )
                    .col(string_null(TransactionEvent::StatusMsg))
                    .col(string_null(TransactionEvent::CallbackUrl))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TransactionEvent::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TransactionEvent {
    Table,
    Id,
    EventId,
    FromAddr,
    ToAddr,
    Amount,
    EventType,
    Direction,
    Info,
    State,
    StatusMsg,
    CallbackUrl,
    TraceId,
}
