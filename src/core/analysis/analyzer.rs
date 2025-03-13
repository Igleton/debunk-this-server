use crate::core::analysis::model::VideoAnalysis;
use crate::core::transcript::base::TranscriptPart;
use anyhow::anyhow;
use rig::extractor::Extractor;
use rig::providers::deepseek;
use rig::providers::deepseek::DeepSeekCompletionModel;

pub struct VideoAnalyzer {
    client: deepseek::Client,
    tavily_api_key: String,
}

impl VideoAnalyzer {
    pub fn new(api_key: &str, api_endpoint: &str, tavily_api_key: &str) -> Self {
        let client = deepseek::Client::from_url(api_key, api_endpoint);
        VideoAnalyzer {
            client: client.clone(),
            tavily_api_key: tavily_api_key.to_string(),
        }
    }

    fn build_extractor(&self, system_prompt: &str) -> Extractor<DeepSeekCompletionModel, VideoAnalysis> {
        self.client
            //.agent(deepseek::DEEPSEEK_REASONER)
            //.tool(Websearch::new(self.tavily_api_key.clone()))
            .extractor::<VideoAnalysis>("deepseek-r1-distill-llama-70b")
            .preamble(system_prompt)
            .build()
    }
    pub async fn analyze(
        &self,
        content: Vec<TranscriptPart>,
    ) -> Result<VideoAnalysis, anyhow::Error> {
        match self
            .build_extractor("You are a video analysis expert.")
            .extract(
                content
                    .iter()
                    .map(|t| t.text.to_owned())
                    .collect::<String>()
                    .as_str(),
            )
            .await
        {
            Ok(result) => Ok(result),
            Err(err) => Err(anyhow!(err)),
        }
    }
}
