#[cfg(test)]
mod api_test {
    use axum::http::StatusCode;
    use axum_test_helper::TestClient;
    use marimari_api;
    use marimari_api::builder;
    use rstest::*;

    #[fixture]
    fn test_client() -> TestClient {
        let app = builder::build_app().expect("failed to build app");
        TestClient::new(app)
    }

    #[rstest]
    #[case("/", StatusCode::OK, "Hello, World!")]
    #[case("/hey", StatusCode::OK, "hey")]
    #[tokio::test]
    async fn ping(
        test_client: TestClient,
        #[case] uri: &str,
        #[case] status_code: StatusCode,
        #[case] message: &str,
    ) {
        let res = test_client.get(uri).send().await;
        assert_eq!(res.status(), status_code);
        assert_eq!(res.text().await, message);
    }
}
