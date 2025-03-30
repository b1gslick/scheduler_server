use crate::swagger::serve_swagger;
use crate::swagger::ApiDoc;

use std::sync::Arc;

use tracing_subscriber::fmt::format::FmtSpan;
use warp::{http::Method, Filter, Rejection, Reply};

use utoipa::OpenApi;
use utoipa_swagger_ui::Config as SwaggerConfig;

pub use handle_errors;
pub mod config;
pub mod routes;
pub mod store;
pub mod swagger;
pub mod tests;
mod types;

const VERSION: &str = "v1";

async fn build_routes(
    store: store::Store,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let store_filter = warp::any().map(move || store.clone());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let get_activities = warp::get()
        .and(warp::path(VERSION))
        .and(warp::path("activity"))
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(routes::activities::get_activities);

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

    let add_time_spent = warp::post()
        .and(warp::path(VERSION))
        .and(warp::path("time_spent"))
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::time_spent::add_time_spent);

    let get_time_spent = warp::get()
        .and(warp::path(VERSION))
        .and(warp::path("time_spent"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and_then(routes::time_spent::get_time_spent_by_id);

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

    let routes = build_routes(store).await;

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
        tests::helpers::{create_postgres, prepare_store},
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
        };
        let result = setup_store(&config).await;
        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn medium_test_get_empty_activities() {
        env::set_var("PASETO_KEY", "RANDOM WORDS WINTER MACINTOSH PC");
        let docker = Cli::default();
        let node = docker.run(create_postgres());
        let store = prepare_store(node.get_host_port_ipv4(5432)).await.unwrap();

        let filter = build_routes(store).await;

        let register = format!("/{}/registration", VERSION);
        let login = format!("/{}/login", VERSION);
        let body = &serde_json::json!({"email": "test@test.iv", "password": "AbcD1x!#"});

        let reg_req = warp::test::request()
            .method("POST")
            .path(&register)
            .json(body)
            .reply(&filter)
            .await;

        assert_eq!(reg_req.status(), 200);

        let login_req = warp::test::request()
            .method("POST")
            .path(&login)
            .json(body)
            .reply(&filter)
            .await;

        let token = convert_to_string(login_req.body())
            .await
            .unwrap()
            .replace("\"", "");
        println!("{:?}", token);

        let path = format!("/{}/activity?limit=1&offset=1", VERSION);
        let res = warp::test::request()
            .method("GET")
            .header("Authorization", token)
            .path(&path)
            .reply(&filter)
            .await;
        println!("{:?}", convert_to_string(res.body()).await.unwrap());
        assert_eq!(res.body().to_vec(), b"[]");
    }

    async fn convert_to_string(
        bytes: &warp::hyper::body::Bytes,
    ) -> Result<String, warp::Rejection> {
        String::from_utf8(bytes.to_vec()).map_err(|_| warp::reject())
    }
}
