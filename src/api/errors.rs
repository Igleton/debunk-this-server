use std::error::Error;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug, thiserror::Error)]
#[error("Api Error")]
pub enum ApiError {
    Anyhow(#[from] anyhow::Error),
    
    #[error("Processing error: {0}")]
    ProcessingError(String),
    
    DatabaseAccessError(#[from] sqlx::Error)
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.source().unwrap().to_string()),
        )
            .into_response()
    }
}