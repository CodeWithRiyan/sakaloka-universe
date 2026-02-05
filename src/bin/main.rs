use loco_rs::cli;
use migration::Migrator;
use pos_damai_rust::app::App;

#[tokio::main]
async fn main() -> loco_rs::Result<()> {
    dotenvy::dotenv().ok();
    cli::main::<App, Migrator>().await
}
