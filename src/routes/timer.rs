use crate::cache::CacheStore;
use crate::store::Store;
use crate::types::account::Session;
use chrono::prelude::*;
use tracing::info;
use warp::http::StatusCode;
use warp::reply::json;

#[utoipa::path(
        post,
        path = "timer/start/{activity_id}",
        responses(
            (status = 200, description = "Timer started"),
            (status = 404, description = "Not found")
        ),
        params(
            ("activity_id" = i32, Path, description = "acctivity id")
        ),
        security(
            ("Authorization" = [])
        )
    )]
pub async fn start(
    id: i32,
    session: Session,
    store: Store,
    cache: CacheStore,
) -> Result<impl warp::Reply, warp::Rejection> {
    let time = Utc::now();
    info!("start timer for: {}", id);
    let account_id = session.account_id;
    if store
        .clone()
        .get_activity_by_id(account_id, id)
        .await
        .is_err()
    {
        return Err(warp::reject::custom(
            handle_errors::Error::MissingParameters,
        ));
    }

    if let Ok(get_value) = cache.clone().get_value(id.to_string()).await {
        return Ok(warp::reply::with_status(
            json(&(get_value / 1_000_000_000)),
            StatusCode::OK,
        ));
    }

    match cache.clone().set_value(id.to_string(), time).await {
        Ok(res) => Ok(warp::reply::with_status(json(&res), StatusCode::OK)),
        Err(_) => Err(warp::reject::not_found()),
    }
}

#[utoipa::path(
        post,
        path = "timer/stop/{activity_id}",
        params(
            ("activity_id" = i32, Path, description = "acctivity id")
        ),
        responses(
            (status = 200, description = "Timer stop"),
            (status = 404, description = "Not found"),
        ),
        security(
            ("Authorization" = [])
        )
    )]
pub async fn stop(
    id: i32,
    session: Session,
    store: Store,
    cache: CacheStore,
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("stop timer for: {}", id);
    let account_id = session.account_id;

    let time_now = Utc::now();
    let mut activity = store
        .clone()
        .get_activity_by_id(account_id.clone(), id)
        .await
        .unwrap();

    match cache.clone().get_value(id.to_string()).await {
        Ok(time) => {
            let diff = ((time_now.timestamp_nanos_opt().unwrap() - time) / 1_000_000_000) as i32;
            activity.time -= diff;
            match store
                .clone()
                .update_activity(activity, id, account_id.clone())
                .await
            {
                Ok(res) => res,
                Err(e) => return Err(warp::reject::custom(e)),
            };
        }
        Err(_) => return Err(warp::reject::not_found()),
    };

    match cache.clone().delete_value(id.to_string()).await {
        Ok(res) => Ok(warp::reply::with_status(json(&res), StatusCode::OK)),
        Err(_) => Err(warp::reject::not_found()),
    }
}

#[cfg(test)]
mod test_timers {
    use crate::{routes::timer::start, routes::timer::stop};
    use testcontainers::clients::Cli;
    use warp::reply::Reply;

    use crate::tests::helpers::{
        create_postgres, create_redis, get_session, prepare_cache, prepare_store,
    };

    #[tokio::test]
    async fn medium_test_user_can_start_timer() {
        let docker = Cli::default();
        let node = docker.run(create_postgres());
        let redis = docker.run(create_redis());
        let store = prepare_store(node.get_host_port_ipv4(5432)).await.unwrap();
        let cache = prepare_cache(redis.get_host_port_ipv4(6379)).await.unwrap();
        store.clone().add_test_account(1).await;
        store.clone().add_test_acctivities().await;
        let result = start(1, get_session(1), store.clone(), cache.clone())
            .await
            .unwrap()
            .into_response();
        assert_eq!(result.status(), 200);
    }

    #[tokio::test]
    async fn medium_test_user_can_stop_timer() {
        let docker = Cli::default();
        let node = docker.run(create_postgres());
        let redis = docker.run(create_redis());
        let store = prepare_store(node.get_host_port_ipv4(5432)).await.unwrap();
        let cache = prepare_cache(redis.get_host_port_ipv4(6379)).await.unwrap();
        store.clone().add_test_account(1).await;
        store.clone().add_test_acctivities().await;
        let result = start(1, get_session(1), store.clone(), cache.clone())
            .await
            .unwrap()
            .into_response();
        assert_eq!(result.status(), 200);
        let result = stop(1, get_session(1), store.clone(), cache.clone())
            .await
            .unwrap()
            .into_response();
        assert_eq!(result.status(), 200);
    }

    #[tokio::test]
    async fn medium_test_if_user_press_start_after_started_return_key_value() {
        let docker = Cli::default();
        let node = docker.run(create_postgres());
        let redis = docker.run(create_redis());
        let store = prepare_store(node.get_host_port_ipv4(5432)).await.unwrap();
        let cache = prepare_cache(redis.get_host_port_ipv4(6379)).await.unwrap();
        store.clone().add_test_account(1).await;
        store.clone().add_test_acctivities().await;
        let result = start(1, get_session(1), store.clone(), cache.clone())
            .await
            .unwrap()
            .into_response();
        assert_eq!(result.status(), 200);
        let result = start(1, get_session(1), store.clone(), cache.clone())
            .await
            .unwrap()
            .into_response();
        assert_eq!(result.status(), 200);
    }
}
