pub mod auth;
pub mod user;

use rocket::Route;

pub fn get_routes() -> Vec<Route> {
    let mut routes = Vec::new();
    routes.extend(user::routes());
    routes.extend(auth::routes());
    routes
}
