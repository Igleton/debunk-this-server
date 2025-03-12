use crate::api::synthesize;

pub fn router() -> axum::Router {
    axum::Router::new().nest("/synthesize", synthesize::router())
}