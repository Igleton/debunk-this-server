use crate::api::errors::ApiError;
use crate::core::transcript::base::TranscriptPart;
use crate::core::transcript::youtube::get_youtube_video_transcript;
use crate::core::video::video::get_video_info;
use crate::state::AppState;
use axum::extract::Path;
use axum::routing::get;
use axum::{Extension, Json};
use rustypipe::model::VideoDetails;
use rustypipe::model::richtext::ToMarkdown;
use serde_json::{Value, json};
use std::sync::Arc;

pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/{video_id}", get(synthesize))
        .route("/{video_id}/transcript", get(transcript))
        .route("/{video_id}/info", get(info))
}

async fn synthesize(
    Path(video_id): Path<String>,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<Value>, ApiError> {
    let transcripts = get_youtube_video_transcript(video_id.as_str(), None).await?;
    let video_info = get_video_info(video_id.as_str()).await?;
    let analysis = state
        .analyzer
        .analyze(
            "You are expected to analyze the transcript of this video.".to_string(),
            transcripts,
            video_info.name,
            video_info.description.to_markdown(),
            video_info.channel.name,
        )
        .await?;
    Ok(Json(json!({"analysis": analysis})))
}

async fn transcript(Path(video_id): Path<String>) -> Result<Json<Vec<TranscriptPart>>, ApiError> {
    Ok(Json(
        get_youtube_video_transcript(video_id.as_str(), None).await?,
    ))
}

async fn info(Path(video_id): Path<String>) -> Result<Json<VideoDetails>, ApiError> {
    Ok(Json(get_video_info(video_id.as_str()).await?))
}
