use serde::Deserialize;
use std::env;

#[derive(Deserialize, Clone)]
pub struct DeepSeekSettings {
    pub api_endpoint: String,
    pub api_key: String,
    pub model_name: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct WebsearchSettings {
    pub tavily_key: String,
}

#[derive(Deserialize, Clone)]
pub struct DatabaseSettings {
    pub connection_string: String,
}

#[derive(Deserialize, Clone)]
pub struct Settings {
    pub deepseek: DeepSeekSettings,
    pub websearch: WebsearchSettings,
    pub database: DatabaseSettings,
}

pub fn get_settings() -> Result<Settings, Box<dyn std::error::Error>> {
    Ok(
        match env::var("PROFILE").unwrap_or("local".to_string()).as_str() {
            "local" => {
                config::Config::builder().add_source(config::File::with_name("Settings.toml"))
            }
            _ => config::Config::builder().add_source(config::Environment::with_prefix("APP_")),
        }
        .build()?
        .try_deserialize::<Settings>()?,
    )
}
