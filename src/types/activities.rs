use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, Hash, PartialEq, ToSchema)]
pub struct ActivityId(pub i32);

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct Activity {
    pub id: ActivityId,
    pub title: String,
    pub content: String,
    pub time: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct NewActivity {
    pub title: String,
    pub content: String,
    pub time: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct PartiaActivity {
    pub title: Option<String>,
    pub content: Option<String>,
    pub time: Option<i32>,
}
