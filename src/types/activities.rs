use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct ActivityId(pub String);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Activity {
    pub id: ActivityId,
    pub title: String,
    pub content: String,
    pub time: i32,
}
