use anyhow::Context;
use domain::{ProviderUserRepository, User, UserName, UserRepository};

pub struct CreateUserCommand {
    name: UserName,
}

pub async fn create_user<T>(ctx: &T, cmd: CreateUserCommand) -> anyhow::Result<User>
where
    T: ProviderUserRepository,
{
    let user = User::new(cmd.name);
    let user_repository = ProviderUserRepository::provide(ctx);

    user_repository.save(&user).await;

    Ok(user)
}
