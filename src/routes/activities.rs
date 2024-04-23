use crate::store::Store;
use crate::types::account::Session;
use crate::types::activities::{Activity, NewActivity};
use crate::types::pagination::extract_pagination;
use crate::types::pagination::Pagination;
use std::collections::HashMap;
use tracing::{error, info, instrument};
use warp::http::StatusCode;

#[instrument]
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

pub async fn update_activities(
    id: i32,
    session: Session,
    store: Store,
    activity: Activity,
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("update activities");
    let account_id = session.account_id;
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
