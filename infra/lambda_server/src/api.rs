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
    use rstest::*;
    use axum::http::StatusCode;
    use axum_test_helper::TestClient;
    use diesel::r2d2::ConnectionManager;
    use r2d2;

    use app_context::AppContext;
    use diesel_repository::UserRepositoryImpl;

    #[fixture]
    fn app_client() -> TestClient {
        let database_url = std::env::var("DATABASE_URL").expect("failed to read the env var DATABASE_URL.");
        let manager = ConnectionManager::new(database_url);
        let pool = r2d2::Pool::new(manager).expect("failed to create the connection pool.");

        // build context
        let user_repository = UserRepositoryImpl::new(pool);

        // dependency injection
        let context = AppContext { user_repository };
 
        let app = app(context);

        TestClient::new(app)
    }

    #[rstest]
    #[tokio::test]
    async fn check_if_returns_hello_world(app_client: TestClient) {
        let res = app_client.get("/").send().await;

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(res.text().await, "Hello, World!");
    }

    #[rstest]
    #[tokio::test]
    async fn check_if_hey(app_client: TestClient) {
        let res = app_client.get("/hey").send().await;

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(res.text().await, "hey");
    }
}
