use axum::http::StatusCode;

use error::AppError;

pub fn handle_error(err: anyhow::Error) -> StatusCode {
    eprintln!("{err:?}");

    match err.downcast_ref::<AppError>() {
        Some(err) => match err {
            AppError::InvalidArgument(msg) => StatusCode::BAD_REQUEST,
            AppError::Internal(msg) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound(msg) => StatusCode::NOT_FOUND,
        },
        None => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
