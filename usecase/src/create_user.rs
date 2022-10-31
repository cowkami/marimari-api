use anyhow::Context;
use error::AppError;
use typed_builder::TypedBuilder;

use domain::{ProvideUserRepository, User, UserName, UserRepository};

#[derive(TypedBuilder)]
pub struct CreateUserCommand {
    name: UserName,
}

pub async fn create_user<T>(ctx: &T, cmd: CreateUserCommand) -> anyhow::Result<User>
where
    T: ProvideUserRepository,
{
    let user = User::new(cmd.name);
    let user_repository = ProvideUserRepository::provide(ctx);

    user_repository
        .save(&user)
        .await
        .with_context(|| AppError::Internal("failed to create user".to_string()))?;

    Ok(user)
}

#[cfg(test)]
mod test {
    use super::*;

    use domain::MockUserRepository;
    use mock_context::MockContext;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_create_user() {
        let mut user_repository = MockUserRepository::new();

        user_repository
            .expect_save()
            .withf(|user| user.name().to_string() == "TestUser")
            .returning(|_| Ok(()));

        let ctx = MockContext { user_repository };

        let cmd = CreateUserCommand::builder()
            .name("TestUser".to_string().try_into().unwrap())
            .build();

        create_user(&ctx, cmd).await.unwrap();
    }
}
