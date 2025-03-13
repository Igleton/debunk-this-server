use crate::core::transcript::base::TranscriptPart;
use crate::core::transcript::youtube::get_youtube_video_transcript;
use crate::state::AppState;
use axum::extract::Path;
use axum::routing::get;
use axum::{Extension, Json};
use serde_json::{Value, json};
use std::sync::Arc;

pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/{video_id}", get(synthesize))
        .route("/{video_id}/transcript", get(transcript))
}

async fn synthesize(
    Path(video_id): Path<String>,
    Extension(state): Extension<Arc<AppState>>,
) -> Json<Value> {
    let transcripts = get_youtube_video_transcript(video_id.as_str(), None).await;
    match transcripts {
        Ok(transcripts) => {
            let analysis = state.analyzer.analyze(transcripts).await;
            match analysis {
                Ok(analysis) => Json(json!({"analysis": analysis})),
                Err(err) => Json(json!({"error": err.to_string()})),
            }
        }
        Err(err) => Json(json!({"error": err.to_string()})),
    }
}

async fn transcript(Path(video_id): Path<String>) -> Json<Vec<TranscriptPart>> {
    get_youtube_video_transcript(video_id.as_str(), None)
        .await
        .map_or(Json(vec![]), |parts| Json(parts))
}
