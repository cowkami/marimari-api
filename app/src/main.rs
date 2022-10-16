use anyhow;
use diesel::r2d2::ConnectionManager;

use app_context::AppContext;
use diesel_repository::UserRepositoryImpl;
use lambda_server::Server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // build DB connection
    let database_url =
        std::env::var("DATABASE_URL").expect("failed to read the env var DATABASE_URL.");
    let manager = ConnectionManager::new(database_url);
    let pool = r2d2::Pool::new(manager).expect("failed to create the connection pool.");

    // build context
    let user_repository = UserRepositoryImpl::new(pool);

    // dependency injection
    let context = AppContext { user_repository };
    let server = Server::new(context);

    // run server
    server.serve().await.expect("failed to run lambda server.");

    Ok(())
}
