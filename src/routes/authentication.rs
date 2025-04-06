#![allow(dead_code)]
use argon2::{self, Config};
use chrono::prelude::*;
use regex::Regex;
use std::{env, future};
use warp::Filter;

use crate::store::Store;
use crate::types::account::{Account, AccountID, PubAccount, Session};

#[derive(Debug)]
struct PassCriteria {
    capital: u8,
    digits: u8,
    special: u8,
}

#[utoipa::path(
        post,
        path = "registration",
        request_body = PubAccount,
        responses(
            (status = 200, description = "Account added", body = String),
            (status = 406, description = "Short password or email"),
        )
    )]
pub async fn register(store: Store, account: Account) -> Result<impl warp::Reply, warp::Rejection> {
    if !is_password_valid(&account.password) {
        return Err(warp::reject::custom(handle_errors::Error::PasswordInvalid));
    }
    if !is_email_valid(&account.email) {
        return Err(warp::reject::custom(handle_errors::Error::WrongEmailType));
    }

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
    let salt = rand::random::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}

pub fn is_email_valid(email: &str) -> bool {
    if email.len() < 3 {
        return false;
    }
    let email_regex = Regex::new(
        r"^([a-zA-Z0-9_+]([a-zA-Z0-9_+.]*[a-zA-Z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap();
    email_regex.is_match(email)
}

pub fn is_password_valid(pass: &str) -> bool {
    let mut answer: PassCriteria = PassCriteria {
        capital: 0,
        digits: 0,
        special: 0,
    };
    let allowed_special_symbols =
        env::var("ALLOWED_SPECIAL_SYMBOLS").unwrap_or("!#$%&()*+,-./:;<=>?@[]^_{|}~".to_string());

    let min_len = env::var("MIN_PASS_LEN")
        .unwrap_or("8".to_string())
        .parse::<usize>()
        .expect("Provide number");
    let max_len = env::var("MAX_PASS_LEN")
        .unwrap_or("128".to_string())
        .parse::<usize>()
        .unwrap();
    let capital_words = env::var("NUMBER_CAPITAL_WORDS")
        .unwrap_or("2".to_string())
        .parse::<u8>()
        .unwrap();
    let spec_symbols = env::var("NUMBER_SPECIAL_SYMBOLS")
        .unwrap_or("2".to_string())
        .parse::<u8>()
        .unwrap();
    let digits = env::var("NUMBER_OF_DIGITS")
        .unwrap_or("1".to_string())
        .parse::<u8>()
        .unwrap();

    if max_len < min_len {
        panic!("Max len password should more than min");
    }

    if spec_symbols + digits + capital_words > max_len as u8 {
        panic!("Password criteria couldn't meet, all sum options greater than max password len");
    }

    if pass.len() < min_len || pass.len() > max_len {
        return false;
    }

    for char in pass.chars() {
        if char.is_uppercase() {
            answer.capital += 1;
        }
        if char.is_numeric() {
            answer.digits += 1;
        }
        if allowed_special_symbols.contains(char) {
            answer.special += 1;
        }
    }

    if answer.capital >= capital_words && answer.digits >= digits && answer.special >= spec_symbols
    {
        return true;
    }

    false
}

#[utoipa::path(
        post,
        path = "login",
        request_body = PubAccount,
        responses(
            (status = 200, description = "Ok", body = String),
            (status = 406, description = "Short password or email"),
            (status = 511, description = "Server error"),
        )
    )]
pub async fn login(store: Store, login: Account) -> Result<impl warp::Reply, warp::Rejection> {
    match store.get_account(login.email).await {
        Ok(account) => match verify_password(&account.password, login.password.as_bytes()) {
            Ok(verified) => {
                if verified {
                    Ok(warp::reply::json(&issue_token(
                        account.id.expect("id not found"),
                    )))
                } else {
                    Err(warp::reject::custom(handle_errors::Error::Unauthorized))
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

    use crate::routes::authentication::{is_email_valid, is_password_valid, login, register};
    use testcontainers::clients::Cli;
    use warp::reply::Reply;

    use crate::{
        tests::helpers::{create_postgres, prepare_store},
        types::account::Account,
    };

    use super::{auth, env, issue_token, AccountID};

    #[tokio::test]
    async fn small_test_validation_password_postive_case() {
        env::set_var("MIN_PASS_LEN", "8");
        env::set_var("MAX_PASS_LEN", "34");
        env::set_var("NUMBER_CAPITAL_WORDS", "2");
        env::set_var("NUMBER_OF_DIGITS", "1");
        env::set_var("NUMBER_SPECIAL_SYMBOLS", "28");
        let good_pass = "AbcD1x!#$%&()*+,-./:;<=>?@[]^_{|}~";
        assert!(is_password_valid(good_pass));
        env::set_var("NUMBER_SPECIAL_SYMBOLS", "2");
    }

    #[tokio::test]
    async fn small_test_len_password_not_meet_min_criteria() {
        env::set_var("NUMBER_OF_DIGITS", "0");
        env::set_var("NUMBER_SPECIAL_SYMBOLS", "0");
        env::set_var("NUMBER_CAPITAL_WORDS", "0");
        env::set_var("MIN_PASS_LEN", "2");
        println!("{:?}", is_password_valid("a"));
        assert!(!is_password_valid("a"));
    }

    #[tokio::test]
    async fn small_test_len_password_not_meet_max_criteria() {
        env::set_var("MAX_PASS_LEN", "2");
        env::set_var("NUMBER_OF_DIGITS", "0");
        env::set_var("NUMBER_SPECIAL_SYMBOLS", "0");
        env::set_var("NUMBER_CAPITAL_WORDS", "0");
        assert!(!is_password_valid("aaa"));
    }

    #[tokio::test]
    async fn small_test_capital_words_in_password_not_meet_critiria() {
        env::set_var("MIN_PASS_LEN", "2");
        env::set_var("NUMBER_OF_DIGITS", "0");
        env::set_var("NUMBER_SPECIAL_SYMBOLS", "0");
        env::set_var("NUMBER_CAPITAL_WORDS", "1");
        assert!(!is_password_valid("aa"));
    }
    #[tokio::test]
    async fn small_test_digit_in_password_not_meet_critiria() {
        env::set_var("MIN_PASS_LEN", "2");
        env::set_var("NUMBER_CAPITAL_WORDS", "0");
        env::set_var("NUMBER_SPECIAL_SYMBOLS", "0");
        env::set_var("NUMBER_OF_DIGITS", "1");
        assert!(!is_password_valid("aa"));
    }

    #[tokio::test]
    async fn small_test_special_symbols_in_password_not_meet_critiria() {
        env::set_var("MIN_PASS_LEN", "2");
        env::set_var("MAX_PASS_LEN", "8");
        env::set_var("NUMBER_CAPITAL_WORDS", "0");
        env::set_var("NUMBER_OF_DIGITS", "0");
        env::set_var("NUMBER_SPECIAL_SYMBOLS", "1");
        assert!(!is_password_valid("aa"));
    }

    #[tokio::test]
    #[should_panic(expected = "Max len password should more than min")]
    async fn small_test_min_should_less_then_max() {
        env::set_var("MIN_PASS_LEN", "2");
        env::set_var("MAX_PASS_LEN", "1");
        is_password_valid("aa");
    }

    #[tokio::test]
    #[should_panic(
        expected = "Password criteria couldn't meet, all sum options greater than max password len"
    )]
    async fn small_test_if_options_sum_greater_max_function_should_panic() {
        env::set_var("MIN_PASS_LEN", "2");
        env::set_var("MAX_PASS_LEN", "2");
        env::set_var("NUMBER_CAPITAL_WORDS", "1");
        env::set_var("NUMBER_OF_DIGITS", "1");
        env::set_var("NUMBER_SPECIAL_SYMBOLS", "1");
        is_password_valid("aa");
    }

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
    async fn small_test_post_activities_wrong_token() {
        env::set_var("PASETO_KEY", "RANDOM WORDS WINTER MACINTOSH PC");
        let mut token = issue_token(AccountID(3));
        token.push('a');

        let filter = auth();

        let res = warp::test::request()
            .header("Authorization", token)
            .filter(&filter)
            .await;

        assert!(res.is_err());
    }

    #[tokio::test]
    async fn medium_test_user_should_have_possibilities_for_registration() {
        env::set_var("NUMBER_SPECIAL_SYMBOLS", "2");
        env::set_var("MIN_PASS_LEN", "8");
        env::set_var("MAX_PASS_LEN", "8");
        env::set_var("NUMBER_CAPITAL_WORDS", "2");
        env::set_var("NUMBER_OF_DIGITS", "1");
        let docker = Cli::default();
        let node = docker.run(create_postgres());
        let store = prepare_store(node.get_host_port_ipv4(5432)).await.unwrap();
        let account = Account {
            id: Some(AccountID(1)),
            email: "test@email.iv".to_string(),
            password: "AbcD1x!#".to_string(),
        };
        let result = register(store, account).await.unwrap().into_response();
        assert_eq!(result.status(), 200)
    }

    #[tokio::test]
    async fn medium_test_user_can_login() {
        env::set_var("PASETO_KEY", "RANDOM WORDS WINTER MACINTOSH PC");
        let docker = Cli::default();
        let node = docker.run(create_postgres());
        let store = prepare_store(node.get_host_port_ipv4(5432)).await.unwrap();
        let account = store.clone().add_test_account(2).await.unwrap();
        let result = login(store, account).await.unwrap().into_response();
        assert_eq!(result.status(), 200)
    }

    #[tokio::test]
    async fn medium_test_not_registered_cant_login() {
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

    #[test]
    fn small_test_is_email_valid() {
        let email_addresses = [
            "foo@bar.com",
            "foo.bar42@c.com",
            "42@c.com",
            "f@42.co",
            "foo@4-2.team",
            "foo_bar@bar.com",
            "_bar@bar.com",
            "foo_@bar.com",
            "foo+bar@bar.com",
            "+bar@bar.com",
            "foo+@bar.com",
            "foo.lastname@bar.com",
            "dYDPFjl5bBwaJvE@scheduler.iv",
        ];
        for email_address in &email_addresses {
            assert!(is_email_valid(email_address));
        }
    }
}
