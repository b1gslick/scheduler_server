use crate::routes::activities::activities::*;
use crate::routes::time_spent::time_spent::*;
use crate::store::store::Store;
use clap::Parser;
use config::Config;
use handle_errors::return_error;
use tracing_subscriber::fmt::format::FmtSpan;
use warp::{http::Method, Filter};

mod routes;
mod store;
mod types;

#[derive(Parser, Debug, Default, serde::Deserialize, PartialEq)]
struct Args {
    /// Log level for app
    log_level: String,
    /// Web server port
    port: u16,
}

#[tokio::main]
async fn main() {
    let config = Config::builder()
        .add_source(config::File::with_name("setup"))
        .build()
        .unwrap();
    let config = config.try_deserialize::<Args>().unwrap();

    let log_filter = std::env::var("RUST_LOG").unwrap_or_else(|_| {
        format!(
            "handle_errors={},activities-scheduler-server={},warp={}",
            config.log_level, config.log_level, config.log_level
        )
    });

    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    tracing_subscriber::fmt()
        .with_env_filter(log_filter)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let get_activities = warp::get()
        .and(warp::path("activities"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(get_activities)
        .with(warp::trace(|info| {
            tracing::info_span!(
                  "get_activities request",
                  method = %info.method(),
                  path = %info.path(),
                  id = %uuid::Uuid::new_v4(),
            )
        }));

    let add_activity = warp::post()
        .and(warp::path("activities"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(add_activity)
        .with(warp::trace(|info| {
            tracing::info_span!(
                  "add_activity request",
                  method = %info.method(),
                  path = %info.path(),
                  id = %uuid::Uuid::new_v4(),
            )
        }));

    let update_activities = warp::put()
        .and(warp::path("activities"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(update_activities)
        .with(warp::trace(|info| {
            tracing::info_span!(
                  "update_activities request",
                  method = %info.method(),
                  path = %info.path(),
                  id = %uuid::Uuid::new_v4(),
            )
        }));

    let add_time_spent = warp::post()
        .and(warp::path("time_spent"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(add_time_spent)
        .with(warp::trace(|info| {
            tracing::info_span!(
                  "add_time_spent request",
                  method = %info.method(),
                  path = %info.path(),
                  id = %uuid::Uuid::new_v4(),
            )
        }));
    let get_time_spent = warp::get()
        .and(warp::path("time_spent"))
        .and(warp::path::param::<u32>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_tine_spen_by_id)
        .with(warp::trace(|info| {
            tracing::info_span!(
                  "get_time_spent request",
                  method = %info.method(),
                  path = %info.path(),
                  id = %uuid::Uuid::new_v4(),
            )
        }));

    let deleted_activities = warp::delete()
        .and(warp::path("activities"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(deleted_activities)
        .with(warp::trace(|info| {
            tracing::info_span!(
                  "deleted_activities request",
                  method = %info.method(),
                  path = %info.path(),
                  id = %uuid::Uuid::new_v4(),
            )
        }));

    let routes = get_activities
        .or(add_activity)
        .or(update_activities)
        .or(add_time_spent)
        .or(deleted_activities)
        .or(get_time_spent)
        .with(cors)
        .with(warp::trace::request())
        .recover(return_error);

    warp::serve(routes).run(([127, 0, 0, 1], config.port)).await;
}
