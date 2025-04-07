use tracing::info;
use crate::StatusCode;

#[utoipa::path(
        get,
        path = "healthz",
        responses(
            (status = 200, description = "OK"),
            (status = 404, description = "Not found")
        ),
    )]
pub async fn healthz() -> Result<impl warp::Reply, warp::Rejection> {
    info!("healthz");

    Ok(warp::reply::with_status("OK", StatusCode::OK))
}


