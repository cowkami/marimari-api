use anyhow::{self, Context};
use axum::extract::{Extension, Json};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

use app_context::AppContext;
use domain::{User, UserId};
use error::AppError;

use crate::error_handler::{handle_error, ErrorResponse};
use crate::extracter::QueryString;

#[derive(Deserialize)]
pub struct UserIdReq(String);

impl TryFrom<UserIdReq> for UserId {
    type Error = anyhow::Error;

    fn try_from(UserIdReq(id): UserIdReq) -> anyhow::Result<UserId> {
        UserId::try_from(id)
            .with_context(|| AppError::Internal("failed to cast UserIdReq to UserId".to_string()))
    }
}

#[derive(Deserialize)]
pub struct GetUsersByIdsRequest {
    user_ids: Vec<UserIdReq>,
}

#[derive(Serialize)]
pub struct UserRes {
    name: String,
    id: String,
}

impl From<User> for UserRes {
    fn from(user: User) -> Self {
        Self {
            name: user.name.to_string(),
            id: user.id.to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct GetUsersByIdsResponse {
    users: Vec<UserRes>,
}

pub async fn get_users_by_ids(
    QueryString(params): QueryString<GetUsersByIdsRequest>,
    Extension(ctx): Extension<AppContext>,
) -> anyhow::Result<(StatusCode, Json<GetUsersByIdsResponse>), ErrorResponse> {
    let user_ids = params
        .user_ids
        .into_iter()
        .map(|id| {
            UserId::try_from(id).with_context(|| AppError::Internal("invalid user id".to_string()))
        })
        .collect::<anyhow::Result<Vec<UserId>>>()
        .map_err(|e| handle_error(e))?;

    let users: Vec<UserRes> = usecase::get_users_by_ids(&ctx, user_ids)
        .await
        .map_err(|e| handle_error(e))?
        .into_iter()
        .map(|user| user.into())
        .collect();

    Ok((StatusCode::OK, Json(GetUsersByIdsResponse { users: users })))
}
