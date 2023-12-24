use crate::types::activities::ActivityId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct TimeSpentId(pub i32);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeSpent {
    pub id: TimeSpentId,
    pub time: i32,
    pub activity_id: ActivityId,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewTimeSpent {
    pub time: i32,
    pub activity_id: ActivityId,
}
