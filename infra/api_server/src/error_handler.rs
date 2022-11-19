use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use error::AppError;

#[derive(Debug)]
pub struct ErrorResponse {
    pub code: StatusCode,
    pub message: String,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        (self.code, self.message).into_response()
    }
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

    ErrorResponse { code, message }
}
