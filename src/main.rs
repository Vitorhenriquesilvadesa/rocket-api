#![allow(clippy::result_large_err)]
use api::{app::build_app, telemetry};

extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenvy::dotenv().expect("Failed to load .env file");
    let guard = telemetry::init();

    build_app()
        .await
        .map_err(|e| panic!("{}", e))
        .unwrap()
        .launch()
        .await?;

    drop(guard);

    Ok(())
}
