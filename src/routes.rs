use super::handlers::*;
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn activities_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/activities")
            .route("/", web::post().to(new_activities))
            .route("/{activity_id}", web::get().to(get_activity_by_id)),
    );
}
