use ezsp::Error;
use rocket::Response;
use rocket::http::Status;
use rocket::response::Responder;
use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Debug)]
pub struct EzspJsonResponse<T> {
    result: Result<T, Error>,
}

impl<T> EzspJsonResponse<T> {
    #[must_use]
    pub fn new(result: Result<T, Error>) -> Self {
        Self { result }
    }
}

impl<T> From<Result<T, Error>> for EzspJsonResponse<T> {
    fn from(result: Result<T, Error>) -> Self {
        Self::new(result)
    }
}

impl<'r, 'o: 'r, T> Responder<'r, 'o> for EzspJsonResponse<T>
where
    T: Serialize,
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
