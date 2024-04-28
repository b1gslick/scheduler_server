#![warn(clippy::all)]
pub use handle_errors;
use tokio::sync::{oneshot, oneshot::Sender};
use tracing_subscriber::fmt::format::FmtSpan;
use warp::{http::Method, Filter, Reply};

pub mod config;
pub mod routes;
pub mod store;
mod tests;
mod types;

async fn build_routes(store: store::Store) -> impl Filter<Extract = impl Reply> + Clone {
    let store_filter = warp::any().map(move || store.clone());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let get_activities = warp::get()
        .and(warp::path("activities"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(routes::activities::get_activities);

    let add_activity = warp::post()
        .and(warp::path("activities"))
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::activities::add_activity);

    let update_activities = warp::put()
        .and(warp::path("activities"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::activities::update_activities);

    let add_time_spent = warp::post()
        .and(warp::path("time_spent"))
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::time_spent::add_time_spent);

    let get_time_spent = warp::get()
        .and(warp::path("time_spent"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and_then(routes::time_spent::get_time_spent_by_id);

    let deleted_activities = warp::delete()
        .and(warp::path("activities"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and_then(routes::activities::deleted_activities);

    let registration = warp::post()
        .and(warp::path("registration"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::authentication::register);

    let login = warp::post()
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::authentication::login);

    get_activities
        .or(add_activity)
        .or(update_activities)
        .or(add_time_spent)
        .or(deleted_activities)
        .or(get_time_spent)
        .or(registration)
        .or(login)
        .with(cors)
        .with(warp::trace::request())
        .recover(handle_errors::return_error)
}

pub async fn setup_store(config: &config::Config) -> Result<store::Store, handle_errors::Error> {
    let store = store::Store::new(&format!(
        "postgres://{}:{}@{}:{}/{}",
        config.database_user,
        config.database_password,
        config.database_host,
        config.database_port,
        config.database_name
    ))
    .await
    .map_err(handle_errors::Error::DatabaseQueryError)?;

    sqlx::migrate!()
        .run(&store.clone().connection)
        .await
        .expect("Cannot run migration");

    let log_filter = std::env::var("RUST_LOG").unwrap_or_else(|_| {
        format!(
            "handle_errors={},activities-scheduler-server={},warp={}",
            config.log_level, config.log_level, config.log_level
        )
    });
    tracing_subscriber::fmt()
        .with_env_filter(log_filter)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    Ok(store)
}
pub async fn run(config: config::Config, store: store::Store) {
    let routes = build_routes(store).await;
    warp::serve(routes).run(([0, 0, 0, 0], config.port)).await;
}

pub struct OneshotHandler {
    pub sender: Sender<i32>,
}

pub async fn oneshot(store: store::Store) -> OneshotHandler {
    let routes = build_routes(store).await;
    let (tx, rx) = oneshot::channel::<i32>();

    let socket: std::net::SocketAddr = "127.0.0.1:3030"
        .to_string()
        .parse()
        .expect("Not a valid address");

    let (_, server) = warp::serve(routes).bind_with_graceful_shutdown(socket, async {
        rx.await.ok();
    });

    tokio::task::spawn(server);

    OneshotHandler { sender: tx }
}
