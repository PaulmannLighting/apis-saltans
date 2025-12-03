use std::error::Error;

use rocket::Response;
use rocket::http::Status;
use rocket::response::Responder;
use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Debug)]
pub struct JsonResult<T, E> {
    result: Result<T, E>,
}

impl<T, E> JsonResult<T, E> {
    #[must_use]
    pub fn new(result: Result<T, E>) -> Self {
        Self { result }
    }
}

impl<T, E> From<Result<T, E>> for JsonResult<T, E> {
    fn from(result: Result<T, E>) -> Self {
        Self::new(result)
    }
}

impl<'r, 'o: 'r, T, E> Responder<'r, 'o> for JsonResult<T, E>
where
    T: Serialize,
    E: Error,
{
    fn respond_to(self, req: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        match self.result {
            Ok(data) => Json::from(data).respond_to(req),
            Err(err) => Response::build_from(Json::from(format!("Error: {err}")).respond_to(req)?)
                .status(Status::InternalServerError)
                .ok(),
        }
    }
}
