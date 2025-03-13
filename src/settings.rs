use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct DeepSeekSettings {
    pub api_endpoint: String,
    pub api_key: String,
    pub model_name: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct WebsearchSettings {
    pub tavily_key: String
}

#[derive(Deserialize, Clone)]
pub struct DatabaseSettings {
    pub connection_string: String
}

#[derive(Deserialize, Clone)]
pub struct Settings {
    pub deepseek: DeepSeekSettings,
    pub websearch: WebsearchSettings,
    pub database: DatabaseSettings,
}