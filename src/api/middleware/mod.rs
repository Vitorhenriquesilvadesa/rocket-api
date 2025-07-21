use std::fmt::Debug;

use rocket::async_trait;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

#[async_trait]
pub trait Middleware: Sized {
    type Error: Debug;

    async fn from_request(request: &Request<'_>) -> Result<Self, (Status, Self::Error)>;
}

pub struct MiddlewareGuard<T: Middleware>(pub T);

#[rocket::async_trait]
impl<'r, T: Middleware> FromRequest<'r> for MiddlewareGuard<T> {
    type Error = (Status, T::Error);

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match T::from_request(request).await {
            Ok(inner) => Outcome::Success(MiddlewareGuard(inner)),
            Err((status, err)) => Outcome::Error((status, (status, err))),
        }
    }
}

#[async_trait]
pub trait Authentication: Middleware {}
