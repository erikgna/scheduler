use rocket::http::Status;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub(crate) cause: &'static str,
}

// common errors
pub const ERROR_UNKNOWN_STATUS: Status = Status::InternalServerError;
pub const UNKNOWN_JSON: ErrorResponse = ErrorResponse {
    cause: "Internal Server Error",
};

pub const ERROR_WRONG_REQUEST_STATUS: Status = Status::BadRequest;
pub const WRONG_REQUEST_JSON: ErrorResponse = ErrorResponse {
    cause: "Wrong request",
};

pub const ERROR_UNAUTHORIZED_STATUS: Status = Status::Unauthorized;
pub const UNAUTHORIZED_JSON: ErrorResponse = ErrorResponse {
    cause: "Unauthorized",
};

pub const ERROR_NOT_FOUND_STATUS: Status = Status::NotFound;
pub const NOT_FOUND_JSON: ErrorResponse = ErrorResponse { cause: "Not found" };