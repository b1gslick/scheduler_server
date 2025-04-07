use crate::store::Store;
use crate::types::account::Session;
use crate::types::time_spent::{NewTimeSpent, TimeSpent};
use tracing::info;
use warp::http::StatusCode;
use warp::reply::json;

#[utoipa::path(
        get,
        path = "time_spent/{id}",
        responses(
            (status = 200, description = "Get time spent by id", body = TimeSpent),
            (status = 404, description = "Not found")
        ),
        params(
            ("id" = i32, Path, description = "time spent unique id")
        ),
        security(
            ("Authorization" = [])
        )
    )]
pub async fn get_time_spent_by_id(
    id: i32,
    session: Session,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("get time spen by id {}", id);
    let account_id = session.account_id;
    let res: TimeSpent = match store.get_time_spent_by_id(id, account_id).await {
        Ok(res) => res,
        Err(_) => return Err(warp::reject::not_found()),
    };

    Ok(warp::reply::with_status(json(&res), StatusCode::OK))
}

#[utoipa::path(
        post,
        path = "time_spent",
        request_body = NewTimeSpent,
        responses(
            (status = 200, description = "Get time spent by id", body = NewTimeSpent),
            (status = 404, description = "Not found"),
            (status = 422, description = "Can't parse body", body = NewTimeSpent)
        ),
        security(
            ("Authorization" = [])
        )
    )]
pub async fn add_time_spent(
    session: Session,
    store: Store,
    new_time_spent: NewTimeSpent,
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("find activity for time spent");
    if new_time_spent.time <= 0 {
        return Err(warp::reject::custom(
            handle_errors::Error::MissingParameters,
        ));
    }
    let account_id = session.account_id;
    match store
        .add_time_spent(new_time_spent.clone(), account_id)
        .await
    {
        Ok(_) => Ok(warp::reply::with_status(
            json(&new_time_spent),
            StatusCode::OK,
        )),
        Err(_) => Err(warp::reject::not_found()),
    }
}

#[cfg(test)]
mod test_activities {
    use crate::{
        routes::time_spent::add_time_spent, routes::time_spent::get_time_spent_by_id,
        types::time_spent::NewTimeSpent,
    };
    use testcontainers::clients::Cli;
    use warp::reply::Reply;

    use crate::tests::helpers::{create_postgres, get_session, prepare_store};

    #[tokio::test]
    async fn medium_test_user_can_add_time_spent_for_owned_activities() {
        let docker = Cli::default();
        let node = docker.run(create_postgres());
        let store = prepare_store(node.get_host_port_ipv4(5432)).await.unwrap();
        store.clone().add_test_account(1).await;
        store.clone().add_test_acctivities().await;
        let time_spent = NewTimeSpent {
            time: 1,
            activity_id: crate::types::activities::ActivityId(1),
        };
        let result = add_time_spent(get_session(1), store, time_spent)
            .await
            .unwrap()
            .into_response();
        assert_eq!(result.status(), 200);
    }

    #[tokio::test]
    async fn medium_test_user_can_get_owner_time_spent() {
        let docker = Cli::default();
        let node = docker.run(create_postgres());
        let store = prepare_store(node.get_host_port_ipv4(5432)).await.unwrap();
        store.clone().add_test_account(1).await;
        store.clone().add_test_acctivities().await;
        let time_spent = NewTimeSpent {
            time: 1,
            activity_id: crate::types::activities::ActivityId(1),
        };
        let _ = add_time_spent(get_session(1), store.clone(), time_spent).await;
        let result = get_time_spent_by_id(1, get_session(1), store)
            .await
            .unwrap()
            .into_response();
        assert_eq!(result.status(), 200);
    }
}
