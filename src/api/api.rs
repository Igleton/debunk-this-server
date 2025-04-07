use crate::api::{prompt, synthesize};

pub fn router() -> axum::Router {
    axum::Router::new().nest("/synthesize", synthesize::router()).nest("/prompts", prompt::router())
}