use anyhow;
use axum::Router;
use diesel::r2d2::ConnectionManager;

use api_server::api;
use app_context::AppContext;
use diesel_repository::UserRepositoryImpl;

pub fn build_app() -> anyhow::Result<Router> {
    // build DB connection
    let database_url =
        std::env::var("DATABASE_URL").expect("failed to read the env var DATABASE_URL.");
    let manager = ConnectionManager::new(database_url);
    let pool = r2d2::Pool::new(manager).expect("failed to create the connection pool.");

    // build context
    let user_repository = UserRepositoryImpl::new(pool);

    // dependency injection
    let context = AppContext { user_repository };

    Ok(api(context))
}
