use serde::Deserialize;

#[derive(Deserialize)]
pub struct DeepSeekSettings {
    pub api_endpoint: String,
    pub api_key: String
}

#[derive(Deserialize)]
pub struct Settings {
    pub deepseek: DeepSeekSettings
}