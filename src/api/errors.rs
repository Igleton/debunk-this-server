use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug, thiserror::Error)]
#[error("Api Error")]
pub enum ApiError {
    Anyhow(#[from] anyhow::Error)
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.to_string()),
        )
            .into_response()
    }
}