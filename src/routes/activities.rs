use crate::store::Store;
use crate::types::account::Session;
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

#[cfg(test)]
mod test_activities {
    use crate::routes::activities::{add_activity, deleted_activities, update_activities};
    use crate::types::account::Account;
    use crate::types::activities::{Activity, ActivityId};
    use crate::{
        store::Store,
        types::{
            account::{AccountID, Session},
            activities::NewActivity,
        },
    };
    use chrono::Utc;
    use std::collections::HashMap;
    use testcontainers::RunnableImage;
    use testcontainers_modules::{postgres::Postgres, testcontainers::clients::Cli};
    use warp::reply::Reply;

    impl Store {
        async fn add_test_account(self, id: i32) -> bool {
            let account = Account {
                id: Some(AccountID(id)),
                email: "test@test.iv".to_string(),
                password: "tesstststs".to_string(),
            };
            match self.add_account(account).await {
                Ok(_) => true,
                Err(e) => panic!("{e:?}"),
            }
        }
        async fn add_test_acctivities(self) -> bool {
            let record = NewActivity {
                title: "test".to_string(),
                content: "test".to_string(),
                time: 1,
            };
            match self.add_activity(record, AccountID(1)).await {
                Ok(_) => true,
                Err(e) => panic!("{e:?}"),
            }
        }
        async fn add_tables(&self, name: &str) -> bool {
            let mut tables: HashMap<String, String> = HashMap::new();
            tables.insert(
                "activities".to_string(),
                "CREATE TABLE IF NOT EXISTS activities (
                id serial PRIMARY KEY,
                title VARCHAR (255) NOT NULL,
                content TEXT NOT NULL,
                time integer NOT NULL,
                account_id serial NOT NULL,
                created_on TIMESTAMP NOT NULL DEFAULT NOW()
            );"
                .to_string(),
            );
            tables.insert(
                "accounts".to_string(),
                "CREATE TABLE IF NOT EXISTS accounts (
                id serial NOT NULL,
                email VARCHAR(255) NOT NULL PRIMARY KEY,
                password VARCHAR(255) NOT NULL
                );"
                .to_string(),
            );
            match tables.get(name) {
                Some(insert) => sqlx::query(insert)
                    .fetch_all(&self.connection)
                    .await
                    .is_ok(),
                None => panic!(),
            }
        }
    }

    #[tokio::test]
    async fn test_add_activities() {
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
    async fn test_user_should_get_owned_activities_with_limit() {
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
    async fn test_user_should_get_owned_all_activities() {
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
    async fn test_user_should_get_owned_activities_with_ofset() {
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
    async fn test_update_activities() {
        let docker = Cli::default();
        let node = docker.run(create_postgres());
        let store = prepare_store(node.get_host_port_ipv4(5432)).await.unwrap();
        let account_id = 1;
        let activity_id = 1;
        store.clone().add_test_account(account_id).await;
        store.clone().add_test_acctivities().await;
        let for_update = Activity {
            id: ActivityId(activity_id),
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
    async fn test_update_not_exist_activities() {
        let docker = Cli::default();
        let node = docker.run(create_postgres());
        let store = prepare_store(node.get_host_port_ipv4(5432)).await.unwrap();
        let account_id = 1;
        store.clone().add_test_account(account_id).await;
        let for_update = Activity {
            id: ActivityId(1),
            title: "updated".to_string(),
            content: "full_update".to_string(),
            time: 199999,
        };
        let result = update_activities(1, get_session(account_id), store, for_update).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_user_should_delete_owned_activities() {
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
    async fn test_user_should_not_delete_not_owned_activities() {
        let docker = Cli::default();
        let node = docker.run(create_postgres());
        let store = prepare_store(node.get_host_port_ipv4(5432)).await.unwrap();
        let account_id = 1;
        store.clone().add_test_account(account_id).await;
        let result = deleted_activities(1, get_session(account_id), store).await;
        assert!(result.is_err());
    }

    fn create_postgres() -> RunnableImage<Postgres> {
        RunnableImage::from(Postgres::default()).with_tag("16.2-alpine3.18")
    }

    async fn prepare_store(port: u16) -> Result<Store, sqlx::Error> {
        let store = Store::new(&format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres",
            port
        ))
        .await
        .unwrap();

        store.add_tables("accounts").await;
        store.add_tables("activities").await;
        Ok(store)
    }
    fn get_session(id: i32) -> Session {
        let current_date_time = Utc::now();
        let dt = current_date_time + chrono::TimeDelta::try_days(1).unwrap();
        Session {
            exp: dt,
            account_id: AccountID(id),
            nbf: current_date_time,
        }
    }
}
