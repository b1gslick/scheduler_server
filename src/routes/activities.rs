use crate::store::Store;
use crate::types::account::Session;
use crate::types::activities::{Activity, ActivityId, NewActivity};
use crate::types::pagination::extract_pagination;
use crate::types::pagination::Pagination;
use std::collections::HashMap;
use tracing::{info, instrument};
use warp::http::StatusCode;

#[instrument]
#[utoipa::path(
        get,
        path = "activities",
        responses(
            (status = 200, description = "List activities", body = [Activity])
        ),
        security(
            ("Authorization" = [])
        )
    )]
pub async fn get_activities(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("quering activities");
    let mut pagination = Pagination::default();

    if !params.is_empty() {
        info!(pagination = true);
        pagination = extract_pagination(params)?;
    }

    let res: Vec<Activity> = match store
        .get_activities(pagination.limit, pagination.offset)
        .await
    {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    Ok(warp::reply::json(&res))
}

#[utoipa::path(
        post,
        path = "activities",
        request_body = NewActivity,
        responses(
            (status = 200, description = "activity added", body = Activity),
            (status = 409, description = "activity is already exists"),
            (status = 422, description = "can't add activities", body = Activity)
        ),
        security(
            ("Authorization" = [])
        )
    )]
pub async fn add_activity(
    session: Session,
    store: Store,
    new_activity: NewActivity,
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("add activity");
    let account_id = session.account_id;
    if let Err(e) = store.add_activity(new_activity.clone(), account_id).await {
        info!("Add activity not added{:?}", new_activity.clone());
        return Err(warp::reject::custom(e));
    }
    Ok(warp::reply::with_status(
        format!("Activity added: {:?}", new_activity),
        StatusCode::OK,
    ))
}

#[utoipa::path(
        put,
        path = "activities/{id}",
        request_body = NewActivity,
        params(
            ("id" = i32, Path, description = "Activity unique id")
        ),
        responses(
            (status = 200, description = "activity added", body = Activity),
            (status = 404, description = "activity not found"),
            (status = 422, description = "can't add activities", body = Activity)
        ),
        security(
            ("Authorization" = [])
        )
    )]
pub async fn update_activities(
    id: i32,
    session: Session,
    store: Store,
    new_activity: NewActivity,
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("update activities");
    let account_id = session.account_id;
    let activity = Activity {
        id: ActivityId(id),
        title: new_activity.title,
        content: new_activity.content,
        time: new_activity.time,
    };

    if store.is_activity_owner(id, &account_id).await? {
        let res = match store.update_activity(activity, id, account_id).await {
            Ok(res) => res,
            Err(e) => return Err(warp::reject::custom(e)),
        };
        info!("Update completed with {:?}", &res);
        Ok(warp::reply::json(&res))
    } else {
        Err(warp::reject::custom(handle_errors::Error::Unauthorized))
    }
}

#[utoipa::path(
        delete,
        path = "activities/{id}",
        params(
            ("id" = i32, Path, description = "Activity unique id")
        ),
        responses(
            (status = 200, description = "activity deleted", body = i32),
            (status = 401, description = "Unauthorized"),
            (status = 404, description = "activity not found"),
        ),
        security(
            ("Authorization" = [])
        )
    )]
pub async fn deleted_activities(
    id: i32,
    session: Session,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("delete activities");
    let account_id = session.account_id;
    if store.is_activity_owner(id, &account_id).await? {
        if let Err(e) = store.delete_activity(id, account_id).await {
            return Err(warp::reject::custom(e));
        }
        Ok(warp::reply::with_status(
            format!("Activity {} deleted", id),
            StatusCode::OK,
        ))
    } else {
        Err(warp::reject::custom(handle_errors::Error::Unauthorized))
    }
}

#[cfg(test)]
mod test_activities {
    use crate::routes::activities::{add_activity, deleted_activities, update_activities};
    use crate::tests::helpers::{create_postgres, get_session, prepare_store};
    use crate::types::activities::NewActivity;
    use testcontainers_modules::testcontainers::clients::Cli;
    use warp::reply::Reply;

    #[tokio::test]
    async fn medium_test_add_activities() {
        let docker = Cli::default();
        let node = docker.run(create_postgres());
        let store = prepare_store(node.get_host_port_ipv4(5432)).await.unwrap();
        let account_id = 1;
        store.clone().add_test_account(account_id).await;

        let record = NewActivity {
            title: "test".to_string(),
            content: "test".to_string(),
            time: 1,
        };
        let result = add_activity(get_session(account_id), store.clone(), record)
            .await
            .unwrap()
            .into_response();
        assert_eq!(result.status(), 200);
    }

    #[tokio::test]
    async fn medium_test_user_should_get_owned_activities_with_limit() {
        let docker = Cli::default();
        let node = docker.run(create_postgres());
        let store = prepare_store(node.get_host_port_ipv4(5432)).await.unwrap();
        let account_id = 1;
        let limit = 1;
        store.clone().add_test_account(account_id).await;
        store.clone().add_test_acctivities().await;
        store.clone().add_test_acctivities().await;
        let result = store.clone().get_activities(Some(limit), 0).await.unwrap();
        assert_eq!(result.len() as i32, limit);
    }

    #[tokio::test]
    async fn medium_test_user_should_get_owned_all_activities() {
        let docker = Cli::default();
        let node = docker.run(create_postgres());
        let store = prepare_store(node.get_host_port_ipv4(5432)).await.unwrap();
        let account_id = 1;
        let num_activities = 10;
        store.clone().add_test_account(account_id).await;
        for _ in 0..num_activities {
            store.clone().add_test_acctivities().await;
        }

        let result = store.clone().get_activities(None, 0).await.unwrap();
        assert_eq!(result.len() as i32, num_activities);
    }

    #[tokio::test]
    async fn medium_test_user_should_get_owned_activities_with_offset() {
        let docker = Cli::default();
        let node = docker.run(create_postgres());
        let store = prepare_store(node.get_host_port_ipv4(5432)).await.unwrap();
        let account_id = 1;
        let num_activities = 10;
        store.clone().add_test_account(account_id).await;
        for _ in 0..num_activities {
            store.clone().add_test_acctivities().await;
        }

        let result = store
            .clone()
            .get_activities(None, num_activities - 1)
            .await
            .unwrap();
        assert_eq!(result.len() as i32, num_activities - (num_activities - 1));
    }

    #[tokio::test]
    async fn medium_test_update_activities() {
        let docker = Cli::default();
        let node = docker.run(create_postgres());
        let store = prepare_store(node.get_host_port_ipv4(5432)).await.unwrap();
        let account_id = 1;
        let activity_id = 1;
        store.clone().add_test_account(account_id).await;
        store.clone().add_test_acctivities().await;
        let for_update = NewActivity {
            title: "updated".to_string(),
            content: "full_update".to_string(),
            time: 199999,
        };
        let result = update_activities(activity_id, get_session(account_id), store, for_update)
            .await
            .unwrap()
            .into_response();
        assert_eq!(result.status(), 200);
    }
    #[tokio::test]
    async fn medium_test_update_not_exist_activities() {
        let docker = Cli::default();
        let node = docker.run(create_postgres());
        let store = prepare_store(node.get_host_port_ipv4(5432)).await.unwrap();
        let account_id = 1;
        store.clone().add_test_account(account_id).await;
        let for_update = NewActivity {
            title: "updated".to_string(),
            content: "full_update".to_string(),
            time: 199999,
        };
        let result = update_activities(1, get_session(account_id), store, for_update).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn medium_test_user_should_delete_owned_activities() {
        let docker = Cli::default();
        let node = docker.run(create_postgres());
        let store = prepare_store(node.get_host_port_ipv4(5432)).await.unwrap();
        let account_id = 1;
        store.clone().add_test_account(account_id).await;
        store.clone().add_test_acctivities().await;
        let result = deleted_activities(1, get_session(account_id), store)
            .await
            .unwrap()
            .into_response();
        assert_eq!(result.status(), 200);
    }
    #[tokio::test]
    async fn medium_test_user_should_not_delete_not_owned_activities() {
        let docker = Cli::default();
        let node = docker.run(create_postgres());
        let store = prepare_store(node.get_host_port_ipv4(5432)).await.unwrap();
        let account_id = 1;
        store.clone().add_test_account(account_id).await;
        let result = deleted_activities(1, get_session(account_id), store).await;
        assert!(result.is_err());
    }
}
