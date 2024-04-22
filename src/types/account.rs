use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    pub exp: DateTime<Utc>,
    pub account_id: AccountID,
    pub nbf: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct AccountID(pub i32);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub id: Option<AccountID>,
    pub email: String,
    pub password: String,
}
