pub mod time_s {

    use crate::store::store::Store;
    use crate::types::activities::ActivityId;
    use crate::types::time_spent::{NewTimeSpent, TimeSpent, TimeSpentId};
    use handle_errors::Error;
    use tracing::info;
    use warp::http::StatusCode;

    pub async fn get_tine_spen_by_id(
        id: u32,
        store: Store,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        info!("get tine spen by id {}", id);
        let timespent: Vec<TimeSpent> = store
            .time_spent
            .read()
            .await
            .values()
            .cloned()
            .filter(|s| s.id.0.parse::<u32>().unwrap() == id)
            .collect();
        if !timespent.is_empty() {
            Ok(warp::reply::with_status(
                format!("activity was found {:?}", timespent.clone()),
                StatusCode::OK,
            ))
        } else {
            Err(warp::reject::custom(Error::TimeSpentNotFound))
        }
    }

    pub async fn add_time_spent(
        store: Store,
        new_time_spent: NewTimeSpent,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        info!("find activity for time spent");
        match store.activities.read().await.get(&ActivityId(
            new_time_spent.activity_id.0.clone().to_string(),
        )) {
            Some(_) => {
                info!("add time spent");
                let timespent = TimeSpent {
                    id: TimeSpentId(get_next_time_spent_id(store.clone()).await.to_string()),
                    time: new_time_spent.time,
                    activity_id: ActivityId(new_time_spent.activity_id.0.to_string()),
                };

                store
                    .time_spent
                    .write()
                    .await
                    .insert(timespent.id.clone(), timespent.clone());
                Ok(warp::reply::with_status(
                    format!("Time added wit id {:?}", timespent.id.0.clone()),
                    StatusCode::OK,
                ))
            }
            None => return Err(warp::reject::custom(Error::ActivitiesNotFound)),
        }
    }

    async fn get_next_time_spent_id(store: Store) -> i32 {
        let res: Vec<TimeSpent> = store.time_spent.read().await.values().cloned().collect();
        if !res.is_empty() {
            res.last().clone().unwrap().id.0.parse::<i32>().unwrap() + 1
        } else {
            1
        }
    }
}
