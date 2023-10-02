use super::models::Activities;
use std::sync::Mutex;

pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<u32>,
    pub activites: Mutex<Vec<Activities>>,
}
