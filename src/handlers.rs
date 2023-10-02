use super::models::Activities;
use super::state::AppState;
use actix_web::{web, HttpResponse};
use chrono::Utc;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

pub async fn new_activities(
    new_activity: web::Json<Activities>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    println!("Received new activites");
    let activities_count_for_user = app_state
        .activites
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|activites| activites.activity_id == new_activity.activity_id)
        .count();
    let new_activity = Activities {
        activity_id: Some((activities_count_for_user + 1) as i32),
        activity_name: new_activity.activity_name.clone(),
        activity_description: new_activity.activity_description.clone(),
        planned_time: new_activity.planned_time,
        planned_day: new_activity.planned_day,
        posted_time: Some(Utc::now().naive_utc()),
    };
    app_state.activites.lock().unwrap().push(new_activity);
    HttpResponse::Ok().json("Activity added")
}

pub async fn get_activity_by_id(
    app_state: web::Data<AppState>,
    params: web::Path<String>,
) -> HttpResponse {
    let activity_id: i32 = params.parse::<i32>().unwrap();

    let filtered_activities = app_state
        .activites
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|activity| activity.activity_id == Some(activity_id))
        .collect::<Vec<Activities>>();

    if filtered_activities.len() > 0 {
        HttpResponse::Ok().json(filtered_activities)
    } else {
        HttpResponse::Ok().json("No activities found for id".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::body::MessageBody;
    use actix_web::http::StatusCode;
    use chrono::NaiveDate;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn post_activity_test() {
        let activity = web::Json(Activities {
            activity_id: None,
            activity_name: "First activity".into(),
            activity_description: None,
            planned_time: 30,
            planned_day: NaiveDate::from_ymd_opt(2023, 8, 23),
            posted_time: None,
        });
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            activites: Mutex::new(vec![]),
        });
        let resp = new_activities(activity, app_state).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_by_id_test() {
        let activity = web::Json(Activities {
            activity_id: None,
            activity_name: "First activity".into(),
            activity_description: None,
            planned_time: 30,
            planned_day: NaiveDate::from_ymd_opt(2023, 8, 23),
            posted_time: None,
        });
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            activites: Mutex::new(vec![]),
        });

        let path_item: web::Path<String> = web::Path::from("1".to_string());

        let _ = new_activities(activity, app_state.clone()).await;
        let resp = get_activity_by_id(app_state, path_item);

        // let byte = resp.await.into_body().try_into_bytes().ok().unwrap();
        assert_eq!(resp.await.body().size().is_eof(), false); // try to check size because need to
                                                              // learn hot to get body for compare
    }
}
