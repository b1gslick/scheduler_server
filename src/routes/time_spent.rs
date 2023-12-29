use crate::store::Store;
use crate::types::account::Session;
use crate::types::time_spent::{NewTimeSpent, TimeSpent};
use tracing::info;
use warp::http::StatusCode;

pub async fn get_tine_spen_by_id(
    id: i32,
    session: Session,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("get tine spen by id {}", id);
    let account_id = session.account_id;
    let res: TimeSpent = match store.get_time_spent_by_id(id, account_id).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    Ok(warp::reply::json(&res))
}

pub async fn add_time_spent(
    session: Session,
    store: Store,
    new_time_spent: NewTimeSpent,
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("find activity for time spent");
    let account_id = session.account_id;
    match store
        .add_time_spent(new_time_spent.clone(), account_id)
        .await
    {
        Ok(_) => Ok(warp::reply::with_status(
            format!("Time added wit id {:?}", new_time_spent),
            StatusCode::OK,
        )),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
