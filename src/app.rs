use std::path::Path;

use async_trait::async_trait;
use loco_rs::{
    app::{AppContext, Hooks},
    boot::{create_app, BootResult, StartMode},
    controller::AppRoutes,
    environment::Environment,
    task::Tasks,
    worker::Processor,
    Result,
};
use migration::Migrator;
use sea_orm::DatabaseConnection;

use crate::{controllers, tasks};

pub struct App;
#[async_trait]
impl Hooks for App {
    fn app_name() -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    fn app_version() -> String {
        format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_SHA")
                .or(option_env!("GITHUB_SHA"))
                .unwrap_or("dev")
        )
    }

    async fn boot(mode: StartMode, environment: &Environment) -> Result<BootResult> {
        create_app::<Self, Migrator>(mode, environment).await
    }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::with_default_routes()
            .add_route(controllers::bill::routes())
            .add_route(controllers::transaction::routes())
            .prefix("/api")
            .add_route(controllers::wallet::routes())
    }
    // async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
    //     Ok(vec![Box::new(
    //         initializers::listen_tran::ListenTranInitializer,
    //     )])
    // }

    fn connect_workers<'a>(_p: &'a mut Processor, _ctx: &'a AppContext) {
        todo!("connect workers")
    }

    fn register_tasks(tasks: &mut Tasks) {
        tasks.register(tasks::trans_event_process::TransEventProcess);
    }

    async fn truncate(_db: &DatabaseConnection) -> Result<()> {
        Ok(())
    }

    async fn seed(_db: &DatabaseConnection, _base: &Path) -> Result<()> {
        Ok(())
    }
}
