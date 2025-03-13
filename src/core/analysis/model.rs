use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, JsonSchema, Serialize)]
/// A record representing an analysis
pub struct VideoAnalysis {
    /// The main theme of the video
    pub main_theme: String,
    /// The strong points of the video
    pub strong_points: String,
    /// Where the video is weak
    pub weak_points: String,
    pub errors_or_issues: String,
    pub conclusion: String
}