use loco_rs::cli;
use migration::Migrator;
use muse_integrator::app::App;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    cli::main::<App, Migrator>().await
}
