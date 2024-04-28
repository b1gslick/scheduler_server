use std::collections::HashMap;

use testcontainers::RunnableImage;
use testcontainers_modules::postgres::Postgres;

use crate::{
    routes::authentication::hash_password,
    types::account::{AccountID, Session},
};
use chrono::Utc;

use crate::{
    store::Store,
    types::{account::Account, activities::NewActivity},
};

#[allow(dead_code)]
impl Store {
    pub async fn add_test_account(self, id: i32) -> Option<Account> {
        let pass = hash_password("tesstststs".to_string().as_bytes());
        let mut account = Account {
            id: Some(crate::types::account::AccountID(id)),
            email: "test@test.iv".to_string(),
            password: pass,
        };
        match self.add_account(account.clone()).await {
            Ok(_) => {
                account.password = "tesstststs".to_string();
                Some(account)
            }
            Err(_) => None,
        }
    }
    pub async fn add_test_acctivities(self) -> bool {
        let record = NewActivity {
            title: "test".to_string(),
            content: "test".to_string(),
            time: 1,
        };
        match self
            .add_activity(record, crate::types::account::AccountID(1))
            .await
        {
            Ok(_) => true,
            Err(e) => panic!("{e:?}"),
        }
    }
    pub async fn add_tables(&self, name: &str) -> bool {
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
        tables.insert(
            "time_spent".to_string(),
            "CREATE TABLE IF NOT EXISTS time_spent (
            id serial PRIMARY KEY,
            time integer NOT NULL,
            account_id serial NOT NULL,
            created_on TIMESTAMP NOT NULL DEFAULT NOW(),
            activity_id integer REFERENCES activities
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

#[allow(dead_code)]
pub async fn prepare_store(port: u16) -> Result<Store, sqlx::Error> {
    let store = Store::new(&format!(
        "postgres://postgres:postgres@127.0.0.1:{}/postgres",
        port
    ))
    .await
    .unwrap();

    store.add_tables("accounts").await;
    store.add_tables("activities").await;
    store.add_tables("time_spent").await;
    Ok(store)
}

#[allow(dead_code)]
pub fn create_postgres() -> RunnableImage<Postgres> {
    RunnableImage::from(Postgres::default()).with_tag("16.2-alpine3.18")
}

#[allow(dead_code)]
pub fn get_session(id: i32) -> Session {
    let current_date_time = Utc::now();
    let dt = current_date_time + chrono::TimeDelta::try_days(1).unwrap();
    Session {
        exp: dt,
        account_id: AccountID(id),
        nbf: current_date_time,
    }
}
