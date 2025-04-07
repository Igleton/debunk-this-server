use crate::core::analysis::analyzer::VideoAnalyzer;
use crate::settings::get_settings;
use crate::state::AppState;
use axum::{Extension, Router};
use std::sync::Arc;
use tracing::{error, info};

mod api;
mod core;
mod settings;
mod state;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
    #[shuttle_shared_db::Postgres(
        local_uri = "postgres://postgres:postgres@localhost:5432/debunk_this"
    )] conn_string: String,
) -> shuttle_axum::ShuttleAxum {
    secrets.into_iter().for_each(|(key, val)| unsafe {
        std::env::set_var(key, val);
    });

    let settings = get_settings()?;

    let pool = sqlx::PgPool::connect(&conn_string).await.unwrap();
    sqlx::migrate!("db/migrations").run(&pool).await.expect("Failed to run db migrations");
    let shared_state = Arc::new(AppState {
        settings: settings.clone(),
        analyzer: VideoAnalyzer::new(
            &settings.deepseek_api_key,
            &settings.deepseek_api_endpoint,
            &settings.websearch_tavily_key,
            settings.deepseek_model_name,
        ),
        pool: pool.clone(),
        prompts: core::prompt::PromptRepository::new(pool.clone()),
        syntheses: core::video_synthesis::VideoSynthesisRepository::new(pool.clone()),
        videos: core::video_repository::VideoRepository::new(pool.clone()),
    });
    let app = Router::new()
        .nest("/api", api::api::router())
        .layer(Extension(shared_state));

    Ok(app.into())
}
