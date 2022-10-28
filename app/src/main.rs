use builder::build_app;
use lambda_web::{is_running_on_lambda, run_hyper_on_lambda};
use std::net::SocketAddr;

mod builder;

#[tokio::main]
async fn main() -> anyhow::Result<(), lambda_runtime::Error> {
    let app = build_app()?;

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
