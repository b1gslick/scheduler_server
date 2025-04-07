use std::sync::Arc;

use warp::{
    http::Uri,
    hyper::{Response, StatusCode},
    path::{FullPath, Tail},
    Rejection, Reply,
};

use utoipa_swagger_ui::Config as SwaggerConfig;

use utoipa::{
    openapi::{
        security::{ApiKey, ApiKeyValue, SecurityScheme},
        Components,
    },
    Modify, OpenApi,
};

use crate::routes;
use crate::VERSION;

#[derive(OpenApi)]
#[openapi(paths(
    routes::health::healthz,
    routes::authentication::register,
    routes::authentication::login,
    routes::activities::get_activities,
    routes::activities::get_activity_by_id,
    routes::activities::add_activity,
    routes::activities::update_activities,
    routes::activities::deleted_activities,
    routes::timer::start,
    routes::timer::stop,
))]
pub struct SchedulerApi;

pub struct SecurityAddon;

#[derive(OpenApi)]
#[openapi(
        nest(
            (path = format!("/{}/", VERSION), api = SchedulerApi)
        ),
        modifiers(&SecurityAddon),
)]
pub struct ApiDoc;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert(Components::new());
        components.add_security_scheme(
            "Authorization",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization"))),
        )
    }
}

pub async fn serve_swagger(
    full_path: FullPath,
    tail: Tail,
    config: Arc<SwaggerConfig<'static>>,
) -> Result<Box<dyn Reply + 'static>, Rejection> {
    if full_path.as_str() == "/docs" {
        return Ok(Box::new(warp::redirect::found(Uri::from_static("/docs/"))));
    }

    let path = tail.as_str();
    match utoipa_swagger_ui::serve(path, config) {
        Ok(file) => {
            if let Some(file) = file {
                Ok(Box::new(
                    Response::builder()
                        .header("Content-Type", file.content_type)
                        .body(file.bytes),
                ))
            } else {
                Ok(Box::new(StatusCode::NOT_FOUND))
            }
        }
        Err(error) => Ok(Box::new(
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(error.to_string()),
        )),
    }
}
