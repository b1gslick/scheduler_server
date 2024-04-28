#![allow(dead_code)]
use argon2::{self, Config};
use chrono::prelude::*;
use rand::Rng;
use std::{env, future};
use warp::Filter;

use crate::store::Store;
use crate::types::account::{Account, AccountID, Session};

pub async fn register(store: Store, account: Account) -> Result<impl warp::Reply, warp::Rejection> {
    let hashed_password = hash_password(account.password.as_bytes());
    let account = Account {
        id: account.id,
        email: account.email,
        password: hashed_password,
    };
    match store.add_account(account).await {
        Ok(_) => Ok(warp::reply::json(&"Account added".to_string())),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub fn hash_password(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}

pub async fn login(store: Store, login: Account) -> Result<impl warp::Reply, warp::Rejection> {
    match store.get_account(login.email).await {
        Ok(account) => match verify_password(&account.password, login.password.as_bytes()) {
            Ok(verified) => {
                if verified {
                    Ok(warp::reply::json(&issue_token(
                        account.id.expect("id not found"),
                    )))
                } else {
                    Err(warp::reject::custom(handle_errors::Error::WrongPassword))
                }
            }
            Err(e) => Err(warp::reject::custom(
                handle_errors::Error::ArgonLibraryError(e),
            )),
        },
        Err(e) => Err(warp::reject::custom(e)),
    }
}
fn verify_password(hash: &str, password: &[u8]) -> Result<bool, argon2::Error> {
    argon2::verify_encoded(hash, password)
}

fn issue_token(account_id: AccountID) -> String {
    let current_date_time = Utc::now();
    let dt = current_date_time + chrono::TimeDelta::try_days(1).unwrap();
    let key = env::var("PASETO_KEY").unwrap();

    paseto::tokens::PasetoBuilder::new()
        .set_encryption_key(&Vec::from(key.as_bytes()))
        .set_expiration(&dt)
        .set_not_before(&Utc::now())
        .set_claim("account_id", serde_json::json!(account_id))
        .build()
        .expect("Failed to construct paseto token w/ builder!")
}
pub fn verify_token(token: String) -> Result<Session, handle_errors::Error> {
    let key = env::var("PASETO_KEY").unwrap();
    let token = paseto::tokens::validate_local_token(
        &token,
        None,
        key.as_bytes(),
        &paseto::tokens::TimeBackend::Chrono,
    )
    .map_err(|_| handle_errors::Error::CannotDecryptionToken)?;

    serde_json::from_value::<Session>(token)
        .map_err(|_| handle_errors::Error::CannotDecryptionToken)
}

pub fn auth() -> impl Filter<Extract = (Session,), Error = warp::Rejection> + Clone {
    warp::header::<String>("Authorization").and_then(|token: String| {
        let token = match verify_token(token) {
            Ok(t) => t,
            Err(_) => return future::ready(Err(warp::reject::reject())),
        };

        future::ready(Ok(token))
    })
}

#[cfg(test)]
mod authentication_tests {

    use crate::routes::authentication::{login, register};
    use testcontainers::clients::Cli;
    use warp::reply::Reply;

    use crate::{
        tests::helpers::{create_postgres, prepare_store},
        types::account::Account,
    };

    use super::{auth, env, issue_token, AccountID};

    #[tokio::test]
    async fn small_test_post_activities_auth() {
        env::set_var("PASETO_KEY", "RANDOM WORDS WINTER MACINTOSH PC");
        let token = issue_token(AccountID(3));

        let filter = auth();

        let res = warp::test::request()
            .header("Authorization", token)
            .filter(&filter);

        assert_eq!(res.await.unwrap().account_id, AccountID(3));
    }

    #[tokio::test]
    async fn medium_test_user_should_have_possibilities_for_registration() {
        let docker = Cli::default();
        let node = docker.run(create_postgres());
        let store = prepare_store(node.get_host_port_ipv4(5432)).await.unwrap();
        let account = Account {
            id: Some(AccountID(1)),
            email: "test@email.iv".to_string(),
            password: "test".to_string(),
        };
        let result = register(store, account).await.unwrap().into_response();
        assert_eq!(result.status(), 200)
    }

    #[tokio::test]
    async fn medium_test_user_can_login() {
        let docker = Cli::default();
        let node = docker.run(create_postgres());
        let store = prepare_store(node.get_host_port_ipv4(5432)).await.unwrap();
        let account = store.clone().add_test_account(2).await.unwrap();
        let result = login(store, account).await.unwrap().into_response();
        assert_eq!(result.status(), 200)
    }

    #[tokio::test]
    async fn medium_test_not_registret_cant_login() {
        let docker = Cli::default();
        let node = docker.run(create_postgres());
        let store = prepare_store(node.get_host_port_ipv4(5432)).await.unwrap();
        let account = Account {
            id: Some(AccountID(1)),
            email: "test@email.iv".to_string(),
            password: "test".to_string(),
        };
        let result = login(store, account).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn medium_test_user_cant_login_with_wrong_password() {
        let docker = Cli::default();
        let node = docker.run(create_postgres());
        let store = prepare_store(node.get_host_port_ipv4(5432)).await.unwrap();
        let mut account = store.clone().add_test_account(2).await.unwrap();
        account.password = "test".to_string();
        let result = login(store, account).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn medium_test_user_cant_login_with_wrong_email() {
        let docker = Cli::default();
        let node = docker.run(create_postgres());
        let store = prepare_store(node.get_host_port_ipv4(5432)).await.unwrap();
        let mut account = store.clone().add_test_account(2).await.unwrap();
        account.password = "test".to_string();
        let result = login(store, account).await;
        assert!(result.is_err());
    }
}
