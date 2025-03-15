use crate::core::analysis::analyzer::VideoAnalyzer;
use crate::core::analysis::model::VideoAnalysis;
use crate::settings::Settings;
use crate::state::AppState;
use axum::{Extension, Router};
use config::Config;
use rig::providers::deepseek;
use std::env;
use std::sync::Arc;
use tower_http::trace::{self, TraceLayer};
use tracing::{Level, level_filters::LevelFilter};
use tracing_subscriber::prelude::*;

mod api;
mod core;
mod settings;
mod state;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .try_init()?;

    let settings = match env::var("PROFILE").unwrap_or("local".to_string()).as_str() {
        "local" => Config::builder().add_source(config::File::with_name("Settings.toml")),
        _ => Config::builder().add_source(config::Environment::with_prefix("APP_")),
    }
    .build()?
    .try_deserialize::<Settings>()?;

    let shared_state = Arc::new(AppState {
        settings: settings.clone(),
        analyzer: VideoAnalyzer::new(
            &settings.deepseek.api_key,
            &settings.deepseek.api_endpoint,
            &settings.websearch.tavily_key,
        ),
    });
    let app = Router::new()
        .nest("/api", api::api::router())
        .layer(Extension(shared_state))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
