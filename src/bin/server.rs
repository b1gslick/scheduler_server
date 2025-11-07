use activities_scheduler_server::{config, run, setup_cache, setup_store};

#[tokio::main]
async fn main() -> Result<(), handle_errors::Error> {
    dotenvy::dotenv().ok();

    let config = config::Config::new().expect("Config can't be set");
    let store = setup_store(&config).await?;
    let cache = setup_cache(&config).await?;

    run(config, store, cache).await;

    Ok(())
}
