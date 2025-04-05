use utoipa::{IntoParams};
use serde::{Deserialize};

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct Pagination {
    #[param(inline)]
    pub limit: Option<i32>,
    #[param(inline)]
    pub offset: Option<i32>,
}
