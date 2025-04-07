use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    pub exp: DateTime<Utc>,
    pub account_id: AccountID,
    pub nbf: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, Hash, PartialEq, ToSchema)]
pub struct AccountID(pub i32);

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Account {
    pub id: Option<AccountID>,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct PubAccount {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenAnswer {
    pub token: String,
}
