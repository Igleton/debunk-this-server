use crate::api::errors::ApiError;
use crate::core::transcript::base::TranscriptPart;
use crate::core::transcript::youtube::get_youtube_video_transcript;
use crate::core::video::video::get_video_info;
use crate::state::AppState;
use axum::extract::Path;
use axum::routing::{get, post};
use axum::{Extension, Json};
use rustypipe::model::VideoDetails;
use rustypipe::model::richtext::ToMarkdown;
use serde_json::{Value, json};
use std::sync::Arc;
use tracing::{debug, info};
use crate::core::video_repository::VideoInfo;

pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/{video_id}", post(synthesize))
        .route("/{video_id}/transcript", get(transcript))
        .route("/{video_id}/info", get(info))
}

async fn synthesize(
    Path(video_id): Path<String>,
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<Value>, ApiError> {
    tokio::spawn(async move {
        let transcripts = get_youtube_video_transcript(video_id.as_str(), None).await.unwrap();
        debug!("Gathered transcripts ({})", transcripts.len());
        let video_info = match state.videos.get(video_id.to_owned()).await.unwrap() {
            None => {
                let _info = get_video_info(video_id.as_str()).await.unwrap();
                state.videos.create(_info.clone().into()).await.unwrap();
                _info.into()
            }
            Some(info) => info
        };
        debug!("Gathered video info ({})", video_info.id);
        let prompt = state.prompts.get_prompt("video_analysis".to_string()).await.unwrap().map(|p|p.prompt).ok_or(ApiError::ProcessingError("Prompt not found".to_string())).unwrap();
        debug!("Starting analysis");
        let analysis = state
            .analyzer
            .analyze(
                prompt,
                transcripts.clone(),
                video_info.name,
                video_info.description,
                video_info.channel_name,
            )
            .await.unwrap();
        debug!("Got a result from analyzer !");
        state.syntheses.add_synthesis_for_video(video_id.as_str(), transcripts, &analysis).await.unwrap();
    });

    Ok(Json(json!({"analysis": ""})))
}

async fn transcript(Path(video_id): Path<String>) -> Result<Json<Vec<TranscriptPart>>, ApiError> {
    Ok(Json(
        get_youtube_video_transcript(video_id.as_str(), None).await?,
    ))
}

async fn info(Path(video_id): Path<String>) -> Result<Json<VideoDetails>, ApiError> {
    Ok(Json(get_video_info(video_id.as_str()).await?))
}
