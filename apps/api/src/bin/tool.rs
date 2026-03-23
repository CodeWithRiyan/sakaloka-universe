//! Sakaloka API — CLI tool for database tasks, code generation, etc.

use loco_rs::cli;
use migration::Migrator;
use sakaloka_api::app::App;

#[tokio::main]
async fn main() -> loco_rs::Result<()> {
    cli::main::<App, Migrator>().await
}
