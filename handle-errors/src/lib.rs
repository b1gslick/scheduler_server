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
    UnsupportedMediaType,
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
                write!(f, "Wrong E-Mail/Password combination")
            }
            Error::ArgonLibraryError(_) => {
                write!(f, "Cannot verifiy password")
            }
            Error::CannotDecryptionToken => {
                write!(f, "Cannot decrypt token provide")
            }
            Error::UnsupportedMediaType => {
                write!(f, "Wrong type of body")
            }
        }
    }
}

impl Reject for Error {}

const DUPLICATE_KEY: u32 = 23505;

#[instrument]
pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
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
    } else if let Some(crate::Error::MissingParameters) = r.find() {
        event!(Level::ERROR, "MissingParameters");
        Ok(warp::reply::with_status(
            "Unprocessable entity".to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else if let Some(crate::Error::UnsupportedMediaType) = r.find() {
        event!(Level::ERROR, "Wrong body format");
        Ok(warp::reply::with_status(
            "UnsupportedMediaType".to_string(),
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
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
    async fn small_test_unauthorized() {
        let error_code = warp::reject::custom(Error::Unauthorized);
        let answer = return_error(error_code).await.unwrap().into_response();
        assert_eq!(answer.status(), 401)
    }

    #[tokio::test]
    async fn small_test_cannot_decryption_token() {
        let error_code = warp::reject::custom(Error::CannotDecryptionToken);
        let answer = return_error(error_code).await.unwrap().into_response();
        assert_eq!(answer.status(), 511)
    }

    #[tokio::test]
    async fn small_test_wrong_password() {
        let error_code = warp::reject::custom(Error::WrongPassword);
        let answer = return_error(error_code).await.unwrap().into_response();
        assert_eq!(answer.status(), 401)
    }

    #[tokio::test]
    async fn small_test_missing_paramter() {
        let error_code = warp::reject::custom(Error::MissingParameters);
        let answer = return_error(error_code).await.unwrap().into_response();
        assert_eq!(answer.status(), 422);
    }
    #[tokio::test]
    async fn small_test_time_spent_not_found() {
        let error_code = warp::reject::custom(Error::TimeSpentNotFound);
        let answer = return_error(error_code).await.unwrap().into_response();
        assert_eq!(answer.status(), 404);
    }
    #[tokio::test]
    async fn small_test_unsupported_media_type() {
        let error_code = warp::reject::custom(Error::UnsupportedMediaType);
        let answer = return_error(error_code).await.unwrap().into_response();
        println!("{answer:?}");
        assert_eq!(answer.status(), 415);
    }
}
