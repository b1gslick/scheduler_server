pub mod time_spent {

    use crate::routes::activities::activities::update_activities;
    use crate::store::store::Store;
    use crate::types::activities::ActivityId;
    use crate::types::time_spent::{NewTimeSpent, TimeSpent, TimeSpentId};
    use handle_errors::Error;
    use tracing::info;
    use warp::http::StatusCode;

    pub async fn add_time_spent(
        store: Store,
        new_time_spent: NewTimeSpent,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        info!("add time spent");
        let timespent = TimeSpent {
            id: TimeSpentId(get_next_time_spent_id(store.clone()).await.to_string()),
            time: new_time_spent.time,
            activity_id: ActivityId(new_time_spent.activity_id.0.to_string()),
        };

        match store
            .activities
            .write()
            .await
            .get_mut(&ActivityId(timespent.activity_id.0.clone()))
        {
            Some(a) => {
                let time = a.time - timespent.time;
                let mut new_value = a.clone();
                new_value.time = time;
                let _ = update_activities(a.id.0.clone(), store.clone(), new_value.clone()).await;
                info!("activity id was find");
                store
                    .time_spent
                    .write()
                    .await
                    .insert(timespent.id.clone(), timespent);
                Ok(warp::reply::with_status("Time added", StatusCode::OK))
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
