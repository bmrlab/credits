use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Wallets {
    Table,
    Addr,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        //
        // add column
        //
        /*
        manager
            .alter_table(
                Table::alter()
                    .table(Movies::Table)
                    .add_column_if_not_exists(integer(Movies::Rating))
                    .to_owned(),
            )
            .await
        */

        //
        // delete column
        //
        /*
        manager
            .alter_table(
                Table::alter()
                    .table(Movies::Table)
                    .drop_column(Movies::Rating)
                    .to_owned(),
            )
            .await
        */

        //
        // create index
        //

        manager
            .create_index(
                Index::create()
                    .name("idx-addr")
                    .table(Wallets::Table)
                    .col(Wallets::Addr)
                    .to_owned(),
            )
            .await
    }
}
