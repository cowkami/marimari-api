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

    // #[rstest]
    // #[tokio::test]
    // async fn create_record(test_client: TestClient) {
    //     use api_server::user::{CreateUserRequest, CreateUserResponse};

    //     // given
    //     let req = CreateUserRequest {
    //         name: "UserNameTest".to_string(),
    //     };

    //     // when
    //     let res = test_client.post("/users").json(&req).send().await;

    //     // then
    //     let status_code = res.status();
    //     let CreateUserResponse { name, id: _ } = res.json().await;
    //     assert_eq!(status_code, StatusCode::OK);
    //     assert_eq!(name, "test-username");
    // }
}
