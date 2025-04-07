use config::Case;
use serde::Deserialize;
use tracing::{debug, info};


#[derive(Deserialize, Clone)]
pub struct Settings {
    pub deepseek_api_endpoint: String,
    pub deepseek_api_key: String,
    pub deepseek_model_name: Option<String>,
    pub websearch_tavily_key: String,
}

pub fn get_settings() -> Result<Settings, anyhow::Error> {
    let config = config::Config::builder()
        .add_source(config::Environment::with_convert_case(Case::Snake))
        .build()?;

    Ok(config
        .try_deserialize::<Settings>()?)
}
