use std::env;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use snafu::Snafu;
use uuid::Uuid;
use serde::Serialize;

#[derive(Snafu, Debug)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    // Non route errors
    #[snafu(display("Could not read environment variable: {source}"))]
    MissingEnvVar { source: env::VarError },

    // Route errors

}

#[derive(Serialize)]
struct ErrorResponse {
    code: &'static str,
    message: &'static str,
    reference: Uuid,
}

impl Error {
    fn as_error_code(&self) -> (&'static str, &'static str, StatusCode) {
        match self {
            // Route Errors go here
            _ => (
                "UNKNOWN_ERR",
                "An unknown error occurred",
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let reference = Uuid::new_v4();
        let (code, message, status) = self.as_error_code();

        log::error!("[{reference}] {self:?}");

        let body = Json(ErrorResponse {
            code,
            message,
            reference,
        });

        (status, body).into_response()
    }
}