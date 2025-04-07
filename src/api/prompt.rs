use std::sync::Arc;
use axum::{Extension, Json};
use axum::routing::{get, post};
use serde::Deserialize;
use serde_json::Value;
use crate::api::errors::ApiError;
use crate::state::AppState;

pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/", post(create_prompt))
}

#[derive(Deserialize)]
struct CreatePromptInput {
    name: String,
    prompt: String
}

async fn create_prompt(
    Extension(state): Extension<Arc<AppState>>,
    Json(input): Json<CreatePromptInput>,
) -> Result<(), ApiError> {
    state.prompts.create(input.name.as_str(), input.prompt.as_str()).await?;
    Ok(())
}