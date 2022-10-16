use anyhow::{self, Context};
use axum::{
    extract::{Extension, Json},
    http::StatusCode,
    routing::get,
    Router,
};
use lambda_web::{is_running_on_lambda, run_hyper_on_lambda, LambdaError};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

use app_context::AppContext;
use domain::User;
use error::AppError;
use usecase::CreateUserCommand;

pub struct Server {
    context: AppContext,
}

impl Server {
    pub fn new(context: AppContext) -> Self {
        Self { context }
    }

    pub async fn serve(self) -> anyhow::Result<(), LambdaError> {
        let app = app(self.context);

        if is_running_on_lambda() {
            // Run app on AWS Lambda
            run_hyper_on_lambda(app).await?;
        } else {
            // Run app on local server
            let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
            axum::Server::bind(&addr)
                .serve(app.into_make_service())
                .await?;
        }

        Ok(())
    }
}

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

    fn try_from(
        CreateUserRequest { name }: CreateUserRequest,
    ) -> anyhow::Result<CreateUserCommand> {
        let cmd = CreateUserCommand::builder()
            .name(name.try_into().with_context(|| format!("invalid name"))?)
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

async fn create_user(
    Json(payload): Json<CreateUserRequest>,
    Extension(ctx): Extension<AppContext>,
) -> anyhow::Result<(StatusCode, Json<CreateUserResponse>), StatusCode> {
    let cmd = payload.try_into().map_err(|e| handle_error(e))?;

    let user = usecase::create_user(&ctx, cmd)
        .await
        .map_err(|e| handle_error(e))?;
    Ok((StatusCode::CREATED, Json(user.into())))
}

fn handle_error(err: anyhow::Error) -> StatusCode {
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

pub fn app(ctx: AppContext) -> Router {
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
