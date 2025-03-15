use crate::core::transcript::base::TranscriptPart;
use anyhow::anyhow;
use rig::completion::Prompt;
use rig::providers::deepseek;

pub struct VideoAnalyzer {
    client: deepseek::Client,
    tavily_api_key: String,
    model_name: String,
}

impl VideoAnalyzer {
    pub fn new(api_key: &str, api_endpoint: &str, tavily_api_key: &str, model_name: Option<String>) -> Self {
        let client = deepseek::Client::from_url(api_key, api_endpoint);
        VideoAnalyzer {
            client: client.clone(),
            tavily_api_key: tavily_api_key.to_string(),
            model_name: model_name.unwrap_or(deepseek::DEEPSEEK_REASONER.to_string()),
        }
    }

    pub async fn analyze(
        &self,
        prompt: String,
        transcripts: Vec<TranscriptPart>,
        video_name: String,
        video_description: String,
        video_channel_name: String,
    ) -> Result<String, anyhow::Error> {
        match self.client
            .agent(self.model_name.as_str())
            .preamble(prompt.as_str())
            .context(format!("VIDEO NAME: {video_name}").as_str())
            .context(format!("VIDEO DESCRIPTION: {video_description}").as_str())
            .context(format!("VIDEO CHANNEL: {video_channel_name}").as_str())
            // .tool(Websearch::new(self.tavily_api_key.clone()))
            .build()
            .prompt(transcripts
                .iter()
                .map(|t| t.text.to_owned())
                .collect::<String>()
                .as_str()).await {
            Ok(value) => Ok(value),
            Err(err) => Err(anyhow!(err)),
        }
    }
}
