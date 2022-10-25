mod app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    app::build_app()?
        .serve()
        .await
        .expect("failed to run lambda server.");

    Ok(())
}
