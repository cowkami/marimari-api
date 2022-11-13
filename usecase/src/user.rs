use anyhow::Context;
use error::AppError;
use typed_builder::TypedBuilder;

use domain::{ProvideUserRepository, User, UserId, UserName, UserRepository};

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

pub async fn get_users_by_ids<T>(ctx: &T, ids: Vec<UserId>) -> anyhow::Result<Vec<User>>
where
    T: ProvideUserRepository,
{
    let user_repository = ProvideUserRepository::provide(ctx);

    let users = user_repository
        .get_by_ids(&ids)
        .await
        .with_context(|| AppError::Internal("failed to get users".to_string()))?;

    Ok(users)
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    use domain::MockUserRepository;
    use mock_context::MockContext;

    #[rstest]
    #[tokio::test]
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

    #[rstest]
    #[case("0123456789abcdefffffffffffffffff")]
    #[should_panic]
    #[case("0123456789abcdef0000000000000000")]
    #[tokio::test]
    async fn test_get_by_ids(#[case] user_id: &str) {
        let mut user_repository = MockUserRepository::new();

        user_repository.expect_get_by_ids().returning(|user_ids| {
            let saved_user_id: UserId = "0123456789abcdefffffffffffffffff"
                .to_string()
                .try_into()
                .unwrap();

            if user_ids[0] == saved_user_id {
                Ok(vec![User {
                    name: "TestUser".to_string().try_into().unwrap(),
                    id: saved_user_id,
                }])
            } else {
                panic!()
            }
        });

        let ctx = MockContext { user_repository };

        get_users_by_ids(&ctx, vec![user_id.to_string().try_into().unwrap()])
            .await
            .unwrap();
    }
}
