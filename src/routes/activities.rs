pub mod activities {
    use crate::store::store::Store;
    use crate::types::activities::{Activity, NewActivity};
    use crate::types::pagination::extract_pagination;
    use crate::types::pagination::Pagination;
    use std::collections::HashMap;
    use tracing::{info, instrument};
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
        store: Store,
        new_activity: NewActivity,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        info!("add activity");
        if let Err(e) = store.add_activity(new_activity.clone()).await {
            return Err(warp::reject::custom(e));
        }

        Ok(warp::reply::with_status(
            format!("Activity added: {:?}", new_activity),
            StatusCode::OK,
        ))
    }

    pub async fn update_activities(
        id: i32,
        store: Store,
        activity: Activity,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        info!("update activities");
        let res = match store.update_activity(activity, id).await {
            Ok(res) => res,
            Err(e) => return Err(warp::reject::custom(e)),
        };
        Ok(warp::reply::json(&res))
    }

    pub async fn deleted_activities(
        id: i32,
        store: Store,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        info!("delete activities");
        if let Err(e) = store.delete_activity(id).await {
            return Err(warp::reject::custom(e));
        }
        Ok(warp::reply::with_status(
            format!("Activity {} deleted", id),
            StatusCode::OK,
        ))
    }
}
