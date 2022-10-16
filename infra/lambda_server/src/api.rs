use anyhow;
use axum::{extract::Extension, routing::get, Router};
use lambda_web::{is_running_on_lambda, run_hyper_on_lambda, LambdaError};
use std::net::SocketAddr;

use crate::user::create_user;
use app_context::AppContext;

pub struct AppServer {
    context: AppContext,
}

impl AppServer {
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

pub fn app(context: AppContext) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/hey", get(hey))
        .route("/users", get(|| async { "user dazoon" }).post(create_user))
        .layer(Extension(context))
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
