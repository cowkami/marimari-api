use axum::{extract::Extension, routing::get, Router};

use crate::users::create_user;
use app_context::AppContext;

async fn root() -> &'static str {
    "Hello, World!"
}

async fn hey() -> &'static str {
    "hey"
}

pub fn api(context: AppContext) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/hey", get(hey))
        .route("/users", get(|| async { "user dazoon" }).post(create_user))
        .layer(Extension(context))
}
