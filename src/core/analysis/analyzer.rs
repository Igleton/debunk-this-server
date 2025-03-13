use crate::core::analysis::model::VideoAnalysis;
use crate::core::transcript::base::TranscriptPart;
use anyhow::anyhow;
use rig::agent::Agent;
use rig::completion::Prompt;
use rig::extractor::Extractor;
use rig::providers::deepseek;
use rig::providers::deepseek::DeepSeekCompletionModel;

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
        content: Vec<TranscriptPart>,
    ) -> Result<String, anyhow::Error> {
        match self.client
            .agent(self.model_name.as_str())
            // .tool(Websearch::new(self.tavily_api_key.clone()))
            // .extractor::<VideoAnalysis>("")
            .preamble("You are a video analysis expert")
            .build()
            .prompt(content
                .iter()
                .map(|t| t.text.to_owned())
                .collect::<String>()
                .as_str()).await {
            Ok(value) => Ok(value),
            Err(err) => Err(anyhow!(err)),
        }
    }
}
