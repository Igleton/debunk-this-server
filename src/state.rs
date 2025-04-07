use crate::core::analysis::analyzer::VideoAnalyzer;
use crate::core::prompt::PromptRepository;
use crate::core::video_repository::VideoRepository;
use crate::core::video_synthesis::VideoSynthesisRepository;
use crate::settings::Settings;

pub struct AppState {
    pub settings: Settings,
    pub analyzer: VideoAnalyzer,
    pub pool: sqlx::Pool<sqlx::Postgres>,
    pub prompts: PromptRepository,
    pub syntheses: VideoSynthesisRepository,
    pub videos: VideoRepository
}