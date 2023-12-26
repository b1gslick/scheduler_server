use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct AccountID(pub i32);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub id: Option<AccountID>,
    pub email: String,
    pub password: String,
}
