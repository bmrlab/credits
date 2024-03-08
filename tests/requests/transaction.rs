use credits::{
    app::App,
    models::_entities::{prelude::*, *},
};
use loco_rs::testing;
use sea_orm::{prelude::Decimal, ColumnTrait, EntityTrait, QueryFilter};
use serial_test::serial;

#[tokio::test]
#[serial]
async fn can_get_transaction() {
    testing::request::<App, _, _>(|request, ctx| async move {
        let from_addr = "0xf81dd4f427a6d74e73bfe1235a5202abb706b825";
        let to_addr = "0xd43f66c452ffabff8aea9cd5d2d9e8c3577363f5";
        let from_add_active = Wallets::find()
            .filter(wallets::Column::Addr.eq(from_addr))
            .one(&ctx.db)
            .await
            .expect("find wallet failed")
            .expect("wallet not found");

        let to_add_active = Wallets::find()
            .filter(wallets::Column::Addr.eq(to_addr))
            .one(&ctx.db)
            .await
            .expect("find wallet failed")
            .expect("wallet not found");

        let payload = serde_json::json!({
            "from_addr": "0xf81dd4f427a6d74e73bfe1235a5202abb706b825",
            "to_addr": "0xd43f66c452ffabff8aea9cd5d2d9e8c3577363f5",
            "amount": 1000,
            "event_type": "payment",
            "info": "{}"
        });
        let res = request.post("/api/transaction").json(&payload).await;

        let from_add_active_after = Wallets::find()
            .filter(wallets::Column::Addr.eq(from_addr))
            .one(&ctx.db)
            .await
            .expect("find wallet failed")
            .expect("wallet not found");

        let to_add_active_after = Wallets::find()
            .filter(wallets::Column::Addr.eq(to_addr))
            .one(&ctx.db)
            .await
            .expect("find wallet failed")
            .expect("wallet not found");

        assert_eq!(res.status_code(), 200);
        assert_eq!(
            from_add_active.balance - Decimal::new(1000, 0),
            from_add_active_after.balance
        );
        assert_eq!(
            to_add_active.balance + Decimal::new(1000, 0),
            to_add_active_after.balance
        );
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_recovery() {
    testing::request::<App, _, _>(|request, ctx| async move {
        let from_addr = "0xf81dd4f427a6d74e73bfe1235a5202abb706b825";
        let to_addr = "0xd43f66c452ffabff8aea9cd5d2d9e8c3577363f5";
        let from_add_active = Wallets::find()
            .filter(wallets::Column::Addr.eq(from_addr))
            .one(&ctx.db)
            .await
            .expect("find wallet failed")
            .expect("wallet not found");

        let to_add_active = Wallets::find()
            .filter(wallets::Column::Addr.eq(to_addr))
            .one(&ctx.db)
            .await
            .expect("find wallet failed")
            .expect("wallet not found");

        let payload = serde_json::json!({
            "from_addr": "0xf81dd4f427a6d74e73bfe1235a5202abb706b825",
            "to_addr": "0xd43f66c452ffabff8aea9cd5d2d9e8c3577363f5",
            "info": "{}"
        });
        let res = request
            .post("/api/transaction/recovery")
            .json(&payload)
            .await;
        assert_eq!(res.status_code(), 200);
        let from_add_active_after = Wallets::find()
            .filter(wallets::Column::Addr.eq(from_addr))
            .one(&ctx.db)
            .await
            .expect("find wallet failed")
            .expect("wallet not found");

        let to_add_active_after = Wallets::find()
            .filter(wallets::Column::Addr.eq(to_addr))
            .one(&ctx.db)
            .await
            .expect("find wallet failed")
            .expect("wallet not found");

        assert_eq!(
            from_add_active.balance + to_add_active.balance,
            from_add_active_after.balance
        );
        assert_eq!(Decimal::new(0, 0), to_add_active_after.balance);
    })
    .await;
}
