use axum::{
    routing::{get, post},
    Json, Router,
};

async fn root() -> &'static str {
    "Hello, World!"
}

async fn hey() -> &'static str {
    "hey"
}

#[allow(dead_code)]
pub fn app() -> Router {
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
