pub mod store {

    use handle_errors::Error;
    use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
    use sqlx::Row;

    use crate::types::{
        activities::{Activity, ActivityId, NewActivity},
        time_spent::{NewTimeSpent, TimeSpent, TimeSpentId},
    };
    use tracing::error;

    #[derive(Clone, Debug)]
    pub struct Store {
        pub connection: PgPool,
    }

    impl Store {
        pub async fn new(db_url: &str) -> Self {
            let db_pool = match PgPoolOptions::new()
                .max_connections(5)
                .connect(db_url)
                .await
            {
                Ok(pool) => pool,
                Err(e) => panic!("Couldn't establish DB connection:{}", e),
            };
            Store {
                connection: db_pool,
            }
        }
        pub async fn get_activities(
            self,
            limit: Option<i32>,
            offset: i32,
        ) -> Result<Vec<Activity>, Error> {
            match sqlx::query("SELECT * from activities LIMIT $1 OFFSET $2")
                .bind(limit)
                .bind(offset)
                .map(|row: PgRow| Activity {
                    id: ActivityId(row.get("id")),
                    title: row.get("title"),
                    content: row.get("content"),
                    time: row.get("time"),
                })
                .fetch_all(&self.connection)
                .await
            {
                Ok(activities) => Ok(activities),
                Err(e) => {
                    error!("Can't get activitues with {:?}", e);
                    Err(Error::DatabaseQueryError)
                }
            }
        }
        pub async fn add_activity(self, new_activity: NewActivity) -> Result<Activity, Error> {
            match sqlx::query("INSERT INTO activities (title, contenct, time) VALUES($1, $2, $3)")
                .bind(new_activity.title)
                .bind(new_activity.content)
                .bind(new_activity.time)
                .map(|row: PgRow| Activity {
                    id: ActivityId(row.get("id")),
                    title: row.get("title"),
                    content: row.get("content"),
                    time: row.get("time"),
                })
                .fetch_one(&self.connection)
                .await
            {
                Ok(activity) => Ok(activity),

                Err(e) => {
                    error!("Can't add activity with {:?}", e);
                    Err(Error::DatabaseQueryError)
                }
            }
        }
        pub async fn update_activity(
            self,
            activity: Activity,
            activity_id: i32,
        ) -> Result<Activity, Error> {
            match sqlx::query(
                "UPDATE activities
            SET title = $1, content = $2, time = $3
            WHERE id = $4
            RETURING id, title, content, tags",
            )
            .bind(activity.title)
            .bind(activity.content)
            .bind(activity.time)
            .bind(activity_id)
            .map(|row: PgRow| Activity {
                id: ActivityId(row.get("id")),
                title: row.get("title"),
                content: row.get("content"),
                time: row.get("time"),
            })
            .fetch_one(&self.connection)
            .await
            {
                Ok(activity) => Ok(activity),
                Err(e) => {
                    error!("Can't update activity with {:?}", e);
                    Err(Error::DatabaseQueryError)
                }
            }
        }
        pub async fn delete_activity(&self, activity_id: i32) -> Result<bool, Error> {
            match sqlx::query("DELETE FROM activities WHERE id = $1")
                .bind(activity_id)
                .execute(&self.connection)
                .await
            {
                Ok(_) => Ok(true),
                Err(e) => {
                    error!("Can't delete activity with {:?}", e);
                    Err(Error::DatabaseQueryError)
                }
            }
        }
        pub async fn add_time_spent(
            &self,
            new_time_spent: NewTimeSpent,
        ) -> Result<TimeSpent, Error> {
            match sqlx::query("INSERT INTO time_spent (time, activity_id) VALUES ($1, $2)")
                .bind(new_time_spent.time)
                .bind(new_time_spent.activity_id.0)
                .map(|row: PgRow| TimeSpent {
                    id: TimeSpentId(row.get("id")),
                    time: row.get("time"),
                    activity_id: ActivityId(row.get("activity_id")),
                })
                .fetch_one(&self.connection)
                .await
            {
                Ok(time_spent) => Ok(time_spent),

                Err(e) => {
                    error!("Can't add time spent with {:?}", e);
                    Err(Error::DatabaseQueryError)
                }
            }
        }

        pub async fn get_time_spent_by_id(&self, id: i32) -> Result<TimeSpent, Error> {
            match sqlx::query("SELECT * from time_spent WHERE id = $1")
                .bind(id)
                .map(|row: PgRow| TimeSpent {
                    id: TimeSpentId(row.get("id")),
                    time: row.get("time"),
                    activity_id: ActivityId(row.get("activity_id")),
                })
                .fetch_one(&self.connection)
                .await
            {
                Ok(time_spent) => Ok(time_spent),
                Err(e) => {
                    error!("Can't get time spent with {:?}", e);
                    Err(Error::DatabaseQueryError)
                }
            }
        }
    }
}
