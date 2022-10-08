use domain::ProviderUserRepository;
use dynamodb_repository::UserRepositoryImpl;

pub struct AppContext {
    pub user_repository: UserRepositoryImpl,
}

impl ProviderUserRepository for AppContext {
    type Repository = UserRepositoryImpl;

    fn provide(&self) -> &UserRepositoryImpl {
        &self.user_repository
    }
}
