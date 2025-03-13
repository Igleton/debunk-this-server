use crate::core::analysis::analyzer::VideoAnalyzer;
use crate::settings::get_settings;
use crate::state::AppState;
use axum::{Extension, Router};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

mod api;
mod core;
mod settings;
mod state;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = get_settings()?;

    let shared_state = Arc::new(AppState {
        settings: settings.clone(),
        analyzer: VideoAnalyzer::new(
            &settings.deepseek.api_key,
            &settings.deepseek.api_endpoint,
            &settings.websearch.tavily_key,
            settings.deepseek.model_name,
        ),
        pool: PgPoolOptions::new()
            .max_connections(5)
            .connect(settings.database.connection_string.as_str())
            .await?,
    });
    let app = Router::new()
        .nest("/api", api::api::router())
        .layer(Extension(shared_state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
