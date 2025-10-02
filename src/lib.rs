use crate::swagger::serve_swagger;
use crate::swagger::ApiDoc;

use std::sync::Arc;
use tracing::info;

use tracing_subscriber::fmt::format::FmtSpan;
use warp::{http::Method, http::StatusCode, Filter, Rejection, Reply};

use utoipa::OpenApi;
use utoipa_swagger_ui::Config as SwaggerConfig;

pub use handle_errors;
pub mod cache;
pub mod config;
pub mod routes;
pub mod store;
pub mod swagger;
pub mod tests;
mod types;

const VERSION: &str = "v1";

async fn build_routes(
    store: store::Store,
    cache: cache::CacheStore,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let store_filter = warp::any().map(move || store.clone());
    let cache_filter = warp::any().map(move || cache.clone());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let health_check = warp::get()
        .and(warp::path(VERSION))
        .and(warp::path("healthz"))
        .and(warp::path::end())
        .and_then(routes::health::healthz);

    let get_activities = warp::get()
        .and(warp::path(VERSION))
        .and(warp::path("activity"))
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(warp::query::<types::pagination::Pagination>())
        .and(store_filter.clone())
        .and_then(routes::activities::get_activities);

    let get_activity_by_id = warp::get()
        .and(warp::path(VERSION))
        .and(warp::path("activity"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and_then(routes::activities::get_activity_by_id);

    let add_activity = warp::post()
        .and(warp::path(VERSION))
        .and(warp::path("activity"))
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::activities::add_activity);

    let update_activities = warp::put()
        .and(warp::path(VERSION))
        .and(warp::path("activity"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::activities::update_activities);

    let start_timer = warp::post()
        .and(warp::path(VERSION))
        .and(warp::path("timer"))
        .and(warp::path("start"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and(cache_filter.clone())
        .and_then(routes::timer::start);

    let stop_timer = warp::post()
        .and(warp::path(VERSION))
        .and(warp::path("timer"))
        .and(warp::path("stop"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and(cache_filter.clone())
        .and_then(routes::timer::stop);

    let deleted_activities = warp::delete()
        .and(warp::path(VERSION))
        .and(warp::path("activity"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and_then(routes::activities::deleted_activities);

    let registration = warp::post()
        .and(warp::path(VERSION))
        .and(warp::path("registration"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::authentication::register);

    let login = warp::post()
        .and(warp::path(VERSION))
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::authentication::login);

    get_activities
        .or(get_activity_by_id)
        .or(health_check)
        .or(add_activity)
        .or(update_activities)
        .or(deleted_activities)
        .or(start_timer)
        .or(stop_timer)
        .or(registration)
        .or(login)
        .with(cors)
        .with(warp::trace::request())
        .recover(handle_errors::return_error)
}

#[utoipa::path(
        get,
        path = "healthz",
        responses(
            (status = 200, description = "OK"),
            (status = 404, description = "Not found")
        ),
    )]
pub async fn healthz() -> Result<impl warp::Reply, warp::Rejection> {
    info!("healthz");

    Ok(warp::reply::with_status("OK", StatusCode::OK))
}

pub async fn setup_cache(
    config: &config::Config,
) -> Result<cache::CacheStore, handle_errors::Error> {
    let cache = cache::CacheStore::new(&format!(
        "redis://{}:{}/0",
        config.cache_host, config.cache_port
    ))
    .await
    .unwrap();
    Ok(cache)
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

pub async fn run(config: config::Config, store: store::Store, cache: cache::CacheStore) {
    let swagger_config = Arc::new(SwaggerConfig::from("/api-doc.json"));

    let api_doc = warp::path("api-doc.json")
        .and(warp::get())
        .map(|| warp::reply::json(&ApiDoc::openapi()));

    let swagger_ui = warp::path("docs")
        .and(warp::get())
        .and(warp::path::full())
        .and(warp::path::tail())
        .and(warp::any().map(move || swagger_config.clone()))
        .and_then(serve_swagger);

    let routes = build_routes(store, cache).await;

    warp::serve(api_doc.or(swagger_ui).or(routes))
        .run(([0, 0, 0, 0], config.port))
        .await
}

#[cfg(test)]
mod test_scheduler {

    use std::env;

    use testcontainers::clients::Cli;

    use crate::{
        build_routes,
        config::Config,
        setup_store,
        tests::helpers::{
            convert_to_string, create_postgres, create_redis, prepare_cache, prepare_store,
        },
        types::{
            account::TokenAnswer,
            activities::{Activity, NewActivity},
        },
        VERSION,
    };

    #[tokio::test]
    async fn medium_test_configure_store() {
        let docker = Cli::default();
        let node = docker.run(create_postgres());

        let config = Config {
            log_level: "DEBUG".to_string(),
            port: 8080,
            database_user: "postgres".to_string(),
            database_password: "postgres".to_string(),
            database_host: "127.0.0.1".to_string(),
            database_port: node.get_host_port_ipv4(5432),
            database_name: "postgres".to_string(),
            cache_port: 6379,
            cache_host: "localhost".to_string(),
        };
        let result = setup_store(&config).await;
        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn medium_test_add_activity() {
        env::set_var("PASETO_KEY", "RANDOM WORDS WINTER MACINTOSH PC");
        let docker = Cli::default();
        let node = docker.run(create_postgres());
        let redis = docker.run(create_redis());
        let store = prepare_store(node.get_host_port_ipv4(5432)).await.unwrap();
        let cache = prepare_cache(redis.get_host_port_ipv4(6379)).await.unwrap();

        let filter = build_routes(store, cache).await;

        let register = format!("/{}/registration", VERSION);
        let login = format!("/{}/login", VERSION);
        let body = &serde_json::json!({"email": "test@test.iv", "password": "AbcD1x!#"});

        let reg_req = warp::test::request()
            .method("POST")
            .path(&register)
            .json(body)
            .reply(&filter)
            .await;

        assert_eq!(reg_req.status(), 201);

        let login_req = warp::test::request()
            .method("POST")
            .path(&login)
            .json(body)
            .reply(&filter)
            .await;

        let token = convert_to_string(login_req.body()).await.unwrap();

        let t: TokenAnswer = serde_json::from_str(&token).unwrap();

        let add_body = &serde_json::json!(
        {
            "title": "awesome title",
            "content": "awesome content",
            "time": 60
        });

        let add_path = format!("/{}/activity", VERSION);

        let add_req = warp::test::request()
            .method("POST")
            .header("Authorization", t.token.clone())
            .path(&add_path)
            .json(add_body)
            .reply(&filter)
            .await;

        let raw_act = convert_to_string(add_req.body()).await.unwrap();
        let new_act: NewActivity = serde_json::from_str(&raw_act).unwrap();

        assert_eq!(new_act.time, 3600);
        assert_eq!(new_act.title, "awesome title");
        assert_eq!(new_act.content, "awesome content");
        assert_eq!(add_req.status(), 201);

        let path = format!("/{}/activity?limit=1&offset=0", VERSION);

        let res = warp::test::request()
            .method("GET")
            .header("Authorization", t.token)
            .path(&path)
            .reply(&filter)
            .await;

        let raw_get = convert_to_string(res.body()).await.unwrap();
        let new_get: Vec<Activity> = serde_json::from_str(&raw_get).unwrap();

        assert_eq!(new_get[0].id.0, 1);
        assert_eq!(new_get[0].time, 3600);
    }
}
