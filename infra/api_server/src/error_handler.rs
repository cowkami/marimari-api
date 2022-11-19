use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use error::AppError;
use serde::Serialize;

#[derive(Debug)]
pub struct ErrorResponse {
    code: StatusCode,
    body: ErrorResponseBody,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        (self.code, Json(self.body)).into_response()
    }
}

#[derive(Debug, Serialize)]
struct ErrorResponseBody {
    code: String,
    message: String,
}

pub fn handle_error(err: anyhow::Error) -> ErrorResponse {
    eprintln!("{err:?}");

    let (code, message) = match err.downcast_ref::<AppError>() {
        Some(err) => match err {
            AppError::InvalidArgument(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
        },
        None => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "undefined internal error".to_string(),
        ),
    };

    let body = ErrorResponseBody {
        code: code.to_string(),
        message,
    };

    ErrorResponse { code, body }
}
