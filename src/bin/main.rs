use loco_rs::cli;
use migration::Migrator;
use sakaloka_universe_be::app::App;

#[tokio::main]
async fn main() -> loco_rs::Result<()> {
    dotenvy::dotenv().ok();
    cli::main::<App, Migrator>().await
}
