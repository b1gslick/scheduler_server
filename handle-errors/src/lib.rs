use warp::{
    filters::{body::BodyDeserializeError, cors::CorsForbidden},
    http::StatusCode,
    reject::Reject,
    Rejection, Reply,
};

use argon2::Error as ArgonError;
use std::fmt::Debug;
use tracing::{event, instrument, Level};

#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MigrationError(sqlx::migrate::MigrateError),
    MissingParameters,
    TimeSpentNotFound,
    DatabaseQueryError(sqlx::Error),
    WrongPassword,
    ArgonLibraryError(ArgonError),
    CannotDecryptionToken,
    Unauthorized,
}

// so that two trait bounds essentially collapse into one.
pub trait HelperTrait: Debug {
    // + PartialEq + warp::Reply {
    fn helper_method(&mut self);
    // fn eq(&self, other: &Self) -> bool;
}

impl<T> HelperTrait for T
where
    T: Debug,
    // T: PartialEq,
    // T: warp::Reply,
{
    fn helper_method(&mut self) {
        println!("{:?}", self);
    }
    // fn eq(&self, other: &Self) -> bool {
    //     self == other
    // }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Error::ParseError(ref err) => {
                write!(f, "Cannot parse parameter: {}", err)
            }
            Error::Unauthorized => write!(f, "No permission to change the underlying resource"),
            Error::MigrationError(_) => write!(f, "Cannot migrate data"),
            Error::MissingParameters => write!(f, "Missing parameter"),
            Error::TimeSpentNotFound => write!(f, "Time spent not Found"),
            Error::DatabaseQueryError(_) => {
                write!(f, "Cannot update. invalid data.")
            }
            Error::WrongPassword => {
                write!(f, "Wrong password")
            }
            Error::ArgonLibraryError(_) => {
                write!(f, "Cannot verifiy password")
            }
            Error::CannotDecryptionToken => {
                write!(f, "Cannot decrypt token provide")
            }
        }
    }
}

impl Reject for Error {}

const DUPLICATE_KEY: u32 = 23505;

#[instrument]
pub async fn return_error(r: Rejection) -> Result<impl Reply + HelperTrait, Rejection> {
    if let Some(crate::Error::DatabaseQueryError(e)) = r.find() {
        event!(Level::ERROR, "Database query error");

        match e {
            sqlx::Error::Database(err) => {
                if err.code().unwrap().parse::<u32>().unwrap() == DUPLICATE_KEY {
                    Ok(warp::reply::with_status(
                        "Account already exsists".to_string(),
                        StatusCode::UNPROCESSABLE_ENTITY,
                    ))
                } else {
                    Ok(warp::reply::with_status(
                        "Cannot update data".to_string(),
                        StatusCode::UNPROCESSABLE_ENTITY,
                    ))
                }
            }
            _ => Ok(warp::reply::with_status(
                "Cannot update data".to_string(),
                StatusCode::UNPROCESSABLE_ENTITY,
            )),
        }
    } else if let Some(error) = r.find::<Error>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::RANGE_NOT_SATISFIABLE,
        ))
    } else if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(error) = r.find::<BodyDeserializeError>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else if let Some(crate::Error::WrongPassword) = r.find() {
        event!(Level::ERROR, "Entered wrong password");
        Ok(warp::reply::with_status(
            "Wrong E-Mail/Password combination".to_string(),
            StatusCode::UNAUTHORIZED,
        ))
    } else if let Some(crate::Error::Unauthorized) = r.find() {
        event!(Level::ERROR, "Not matching account id");
        Ok(warp::reply::with_status(
            "No permission to change underlying resource".to_string(),
            StatusCode::UNAUTHORIZED,
        ))
    } else if let Some(crate::Error::CannotDecryptionToken) = r.find() {
        event!(Level::ERROR, "Can't decryption provided token");
        Ok(warp::reply::with_status(
            "Not authorized".to_string(),
            StatusCode::NETWORK_AUTHENTICATION_REQUIRED,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}

#[cfg(test)]
mod handle_error_tests {

    use super::*;

    #[tokio::test]
    async fn test() {
        let error_code = warp::reject::custom(Error::Unauthorized);
        let answer = return_error(error_code).await.unwrap();
        // assert_eq!(
        // answer,
        // warp::reply::with_status("".to_string(), StatusCode::NOT_FOUND)
        // );
    }
}
