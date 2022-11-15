use std::collections::HashSet;

use anyhow::{ensure, Context};
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

    let not_found_ids = diff_vec(ids, users.iter().map(|u| u.id().clone()).collect());
    ensure!(
        not_found_ids.len() == 0,
        AppError::InvalidArgument(format!("user_ids: {not_found_ids:?} is/are not found"))
    );

    Ok(users)
}

fn diff_vec<T>(vec1: Vec<T>, vec2: Vec<T>) -> Vec<T>
where
    T: Eq + std::hash::Hash + Clone,
{
    let vec2set = |v: Vec<T>| v.iter().map(|e| e.clone()).collect::<HashSet<T>>();
    let set1 = vec2set(vec1);
    let set2 = vec2set(vec2);
    set1.difference(&set2)
        .into_iter()
        .map(|e| e.clone())
        .collect()
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
    #[case(vec!["0123456789abcdefffffffffffffffff".to_string().try_into().unwrap()])]
    #[should_panic]
    #[case(vec!["0123456789abcdef0000000000000000".to_string().try_into().unwrap()])]
    #[tokio::test]
    async fn test_get_by_ids(#[case] request_user_ids: Vec<UserId>) {
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
                Ok(vec![])
            }
        });

        let ctx = MockContext { user_repository };

        get_users_by_ids(&ctx, request_user_ids).await.unwrap();
    }
}
