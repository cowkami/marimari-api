use anyhow;
use async_trait::async_trait;
use axum::{
    body::Body,
    extract::{Extension, FromRequest, Json, RequestParts},
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use axum_macros::debug_handler;
use derive_getters::Getters;
use derive_new::new;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use app_context::AppContext;
use domain::User;
use tower::ServiceBuilder;
use tracing_subscriber::layer::Context;
use usecase::CreateUserCommand;

async fn root() -> &'static str {
    "Hello, World!"
}

async fn hey() -> &'static str {
    "hey"
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    name: String,
}

impl TryFrom<CreateUserRequest> for CreateUserCommand {
    type Error = anyhow::Error;

    fn try_from(request: CreateUserRequest) -> anyhow::Result<CreateUserCommand> {
        let CreateUserRequest { name } = request;
        let cmd = CreateUserCommand::builder()
            .name(name.try_into().unwrap())
            .build();
        Ok(cmd)
    }
}

#[derive(Serialize)]
pub struct CreateUserResponse {
    name: String,
    id: String,
}

impl From<User> for CreateUserResponse {
    fn from(user: User) -> Self {
        Self {
            name: user.name().to_string(),
            id: user.id().to_string(),
        }
    }
}

// #[derive(new, Getters)]
// pub struct UserServiceHandler {
// ctx: AppContext,
// }

// impl UserServiceHandler {
// #[debug_handler]
async fn create_user(
    Json(payload): Json<CreateUserRequest>,
    Extension(ctx): Extension<AppContext>,
) -> anyhow::Result<(StatusCode, Json<CreateUserResponse>), StatusCode> {
    let cmd = match payload.try_into() {
        Ok(cmd) => cmd,
        E => {
            return Err(StatusCode::BAD_REQUEST);
        }
    };
    let user = match usecase::create_user(&ctx, cmd).await {
        Ok(user) => user,
        E => {
            return Err(StatusCode::EXPECTATION_FAILED);
        }
    };
    Ok((StatusCode::CREATED, Json(user.into())))
}

#[allow(dead_code)]
pub fn app(ctx: AppContext) -> Router {
    // let handler = UserServiceHandler { ctx };
    // let ctx = Arc::new(ctx);

    Router::new()
        .route("/", get(root))
        .route("/hey", get(hey))
        .route("/users", get(|| async { "user dazoon" }).post(create_user))
        .layer(Extension(ctx))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum_test_helper::TestClient;

    // #[tokio::test]
    // async fn check_if_returns_hello_world() {
    //     let app = app();
    //     let client = TestClient::new(app);
    //     let res = client.get("/").send().await;

    //     assert_eq!(res.status(), StatusCode::OK);
    //     assert_eq!(res.text().await, "Hello, World!");
    // }

    // #[tokio::test]
    // async fn check_if_hey() {
    //     let app = app();
    //     let client = TestClient::new(app);
    //     let res = client.get("/hey").send().await;

    //     assert_eq!(res.status(), StatusCode::OK);
    //     assert_eq!(res.text().await, "hey");
    // }
}
