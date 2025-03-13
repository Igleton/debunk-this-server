use crate::core::analysis::analyzer::VideoAnalyzer;
use crate::settings::Settings;

pub struct AppState {
    pub settings: Settings,
    pub analyzer: VideoAnalyzer 
}