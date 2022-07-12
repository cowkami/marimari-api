use axum::{
    routing::{get, post},
    Json, Router,
};
use lambda_web::{is_running_on_lambda, run_hyper_on_lambda, LambdaError};
use std::net::SocketAddr;

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn hey() -> &'static str {
    "hey"
}

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    // build our application with a route
    let app = app();

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

#[allow(dead_code)]
fn app() -> Router {
    Router::new().route("/", get(root)).route("/hey", get(hey))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum_test_helper::TestClient;

    #[tokio::test]
    async fn check_if_returns_hello_world() {
        let app = app();
        let client = TestClient::new(app);
        let res = client.get("/").send().await;

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(res.text().await, "Hello, World!");
    }

    #[tokio::test]
    async fn check_if_hey() {
        let app = app();
        let client = TestClient::new(app);
        let res = client.get("/hey").send().await;

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(res.text().await, "hey");
    }
}
