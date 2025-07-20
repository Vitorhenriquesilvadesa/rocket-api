use api::app::build_app;

extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    match build_app().await {
        Ok(app) => app.launch().await?,
        Err(e) => {
            panic!("{}", e);
        }
    };

    Ok(())
}
