use anyhow::Context;
use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
};
use serde::de::DeserializeOwned;

use error::AppError;

use crate::error_handler::{handle_error, ErrorResponse};

pub struct QueryString<T>(pub T)
where
    T: Send;

#[async_trait]
impl<T, B> FromRequest<B> for QueryString<T>
where
    T: DeserializeOwned + Send,
    B: Send,
{
    type Rejection = ErrorResponse;

    async fn from_request(req: &mut RequestParts<B>) -> anyhow::Result<Self, Self::Rejection> {
        let query = req
            .uri()
            .query()
            .with_context(|| AppError::InvalidArgument("failed to parse url".to_string()))
            .map_err(|e| handle_error(e))?;

        let query: T = serde_qs::from_str(query)
            .with_context(|| AppError::InvalidArgument("failed to parse query string".to_string()))
            .map_err(|e| handle_error(e))?;

        Ok(Self(query))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Request;
    use rstest::rstest;
    use serde::Deserialize;
    use std::fmt::Debug;

    #[derive(Deserialize, Debug, PartialEq)]
    struct TestRequest {
        vec: Vec<String>,
    }

    async fn check<T: DeserializeOwned + PartialEq + Debug + Send>(uri: impl AsRef<str>, value: T) {
        let mut req = RequestParts::new(Request::builder().uri(uri.as_ref()).body(()).unwrap());
        assert_eq!(
            QueryString::<T>::from_request(&mut req).await.unwrap().0,
            value
        );
    }

    #[rstest]
    #[case(
        "http://example.com/test?vec[0]=1&vec[1]=2",
        TestRequest {vec: vec!["1".to_string(), "2".to_string()]},
    )]
    #[case(
        "http://example.com/test?vec[]=1&vec[]=2",
        TestRequest {vec: vec!["1".to_string(), "2".to_string()]},
    )]
    #[tokio::test]
    async fn test_query_string(#[case] uri: &str, #[case] expected: TestRequest) {
        check(uri, expected).await;
    }
}
