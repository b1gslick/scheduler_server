use crate::types::activities::ActivityId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, Hash, PartialEq, ToSchema)]
pub struct TimeSpentId(pub i32);

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct TimeSpent {
    pub id: TimeSpentId,
    pub time: i32,
    pub activity_id: ActivityId,
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct NewTimeSpent {
    pub time: i32,
    pub activity_id: ActivityId,
}
