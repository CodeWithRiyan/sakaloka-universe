//! Generates the OpenAPI JSON specification for the Sakaloka API.
//!
//! Run with: `cargo run -p sakaloka-api --bin openapi`
//! Pipe to a file: `cargo run -p sakaloka-api --bin openapi > openapi.json`

use utoipa::OpenApi;

fn main() {
    match sakaloka_api::docs::ApiDoc::openapi().to_pretty_json() {
        Ok(spec) => println!("{spec}"),
        Err(e) => {
            eprintln!("Failed to generate OpenAPI JSON: {e}");
            std::process::exit(1);
        }
    }
}
