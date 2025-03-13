use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct DeepSeekSettings {
    pub api_endpoint: String,
    pub api_key: String
}

#[derive(Deserialize, Clone)]
pub struct WebsearchSettings {
    pub tavily_key: String
}

#[derive(Deserialize, Clone)]
pub struct Settings {
    pub deepseek: DeepSeekSettings,
    pub websearch: WebsearchSettings
}