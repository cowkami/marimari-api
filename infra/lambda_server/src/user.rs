use anyhow::{self, Context};
use axum::extract::{Extension, Json};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

use app_context::AppContext;
use domain::User;
use usecase::CreateUserCommand;

use crate::error::handle_error;

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

pub async fn create_user(
    Json(payload): Json<CreateUserRequest>,
    Extension(ctx): Extension<AppContext>,
) -> anyhow::Result<(StatusCode, Json<CreateUserResponse>), StatusCode> {
    let cmd = payload.try_into().map_err(|e| handle_error(e))?;

    let user = usecase::create_user(&ctx, cmd)
        .await
        .map_err(|e| handle_error(e))?;

    Ok((StatusCode::CREATED, Json(user.into())))
}
