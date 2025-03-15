use crate::core::analysis::analyzer::VideoAnalyzer;
use crate::core::prompt::PromptRepository;
use crate::settings::Settings;

pub struct AppState {
    pub settings: Settings,
    pub analyzer: VideoAnalyzer,
    pub pool: sqlx::Pool<sqlx::Postgres>,
    pub prompts: PromptRepository
}