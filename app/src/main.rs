mod api;

use anyhow;
use diesel::r2d2::ConnectionManager;
use lambda_web::{is_running_on_lambda, run_hyper_on_lambda, LambdaError};
use std::net::SocketAddr;

use api::app;
use app_context::AppContext;
use diesel_repository::UserRepositoryImpl;

#[tokio::main]
async fn main() -> anyhow::Result<(), LambdaError> {
    // build DB connection
    let database_url =
        std::env::var("DATABASE_URL").expect("failed to read the env var DATABASE_URL.");
    let manager = ConnectionManager::new(database_url);
    let pool = r2d2::Pool::new(manager).expect("failed to create the connection pool.");

    // dependency injection
    let user_repository = UserRepositoryImpl::new(pool);
    let context = AppContext { user_repository };

    let app = app(context);

    if is_running_on_lambda() {
        // Run app on AWS Lambda
        run_hyper_on_lambda(app).await?;
    } else {
        // Run app on local server
        let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await?;
    }

    Ok(())
}
