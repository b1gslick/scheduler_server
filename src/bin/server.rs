use activities_scheduler_server::{config, run, setup_store};

#[tokio::main]
async fn main() -> Result<(), handle_errors::Error> {
    dotenvy::dotenv().ok();

    let config = config::Config::new().expect("Config can't be set");
    let store = setup_store(&config).await?;

    run(config, store).await;

    Ok(())
}
