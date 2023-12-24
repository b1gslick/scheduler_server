pub mod activities {
    use crate::store::store::Store;
    use crate::types::activities::{Activity, ActivityId};
    use crate::types::pagination::extract_pagination;
    use handle_errors::Error;
    use std::collections::HashMap;
    use tracing::{info, instrument};
    use warp::http::StatusCode;

    #[instrument]
    pub async fn get_activities(
        params: HashMap<String, String>,
        store: Store,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        info!("quering activities");
        if !params.is_empty() {
            let pagination = extract_pagination(params)?;
            info!(pagination = true);
            let res: Vec<Activity> = store.activities.read().await.values().cloned().collect();
            let res = &res[pagination.start..pagination.end];
            Ok(warp::reply::json(&res))
        } else {
            info!(pagination = false);
            let res: Vec<Activity> = store.activities.read().await.values().cloned().collect();
            Ok(warp::reply::json(&res))
        }
    }

    pub async fn add_activity(
        store: Store,
        activity: Activity,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        info!("add activity");
        store
            .activities
            .write()
            .await
            .insert(activity.id.clone(), activity.clone());

        Ok(warp::reply::with_status(
            format!("Activity added: {:?}", activity.clone()),
            StatusCode::OK,
        ))
    }

    pub async fn update_activities(
        id: String,
        store: Store,
        activity: Activity,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        info!("update activities");
        match store.activities.write().await.get_mut(&ActivityId(id)) {
            Some(a) => *a = activity.clone(),
            None => return Err(warp::reject::custom(Error::ActivitiesNotFound)),
        }
        Ok(warp::reply::with_status(
            format!("Activity updated: {:?}", activity.clone()),
            StatusCode::OK,
        ))
    }

    pub async fn deleted_activities(
        id: String,
        store: Store,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        info!("delete activities");
        match store.activities.write().await.remove(&ActivityId(id)) {
            Some(_) => Ok(warp::reply::with_status("Activity deleted", StatusCode::OK)),
            None => Err(warp::reject::custom(Error::ActivitiesNotFound)),
        }
    }
}
