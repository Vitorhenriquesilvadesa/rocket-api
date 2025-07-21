use api::{app::build_app, telemetry};

extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenvy::dotenv().expect("Failed to load .env file");
    let guard = telemetry::init();

    match build_app().await {
        Ok(app) => app.launch().await?,
        Err(e) => {
            panic!("{}", e);
        }
    };

    drop(guard);

    Ok(())
}
