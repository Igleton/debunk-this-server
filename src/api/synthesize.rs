use crate::core::transcript::base::TranscriptPart;
use crate::core::transcript::youtube::get_youtube_video_transcript;
use axum::Json;
use axum::extract::Path;
use axum::routing::get;
use serde_json::Value;

pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/{video_id}", get(synthesize))
        .route("/{video_id}/transcript", get(transcript))
}

async fn synthesize(Path(video_id): Path<String>) -> Json<Value> {
    Json(Value::Null)
}

async fn transcript(Path(video_id): Path<String>) -> Json<Vec<TranscriptPart>> {
    get_youtube_video_transcript(video_id.as_str(), None)
        .await
        .map_or(Json(vec![]), |parts| Json(parts))
}
