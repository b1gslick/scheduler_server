use actix_web::web;
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Activities {
    pub activity_id: Option<i32>,
    pub activity_name: String,
    pub activity_description: Option<String>,
    pub planned_time: i32,
    pub planned_day: Option<NaiveDate>,
    pub posted_time: Option<NaiveDateTime>,
}

impl From<web::Json<Activities>> for Activities {
    fn from(activites: web::Json<Activities>) -> Self {
        Activities {
            activity_id: activites.activity_id,
            activity_name: activites.activity_name.clone(),
            activity_description: activites.activity_description.clone(),
            planned_time: activites.planned_time,
            planned_day: activites.planned_day,
            posted_time: activites.posted_time,
        }
    }
}
