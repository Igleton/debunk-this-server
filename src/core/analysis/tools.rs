use rig::completion::ToolDefinition;
use rig::tool::Tool;
use serde::{Deserialize, Serialize};
use serde_json::json;
#[derive(Debug, thiserror::Error)]
#[error("Websearch error")]
enum WebsearchError {
    Reqwest(#[from] reqwest::Error)
}

#[derive(Deserialize)]
struct WebsearchArgs {
    query: String
}

#[derive(Serialize, Deserialize)]
struct WebsearchOutputResult {
    title: String,
    url: String,
    content: String,
    score: f32,
    raw_content: Option<String>
}

#[derive(Serialize, Deserialize)]
struct WebsearchOutput {
    query: String,
    follow_up_questions: Option<String>,
    answer: Option<String>,
    images: Vec<String>,
    results: Vec<WebsearchOutputResult>,
    response_time: f32,
}

const TAVILY_ENDPOINT: &'static str = "https://api.tavily.com/search";

struct Websearch {
    pub tavily_api_key: String
}

impl Websearch {
    pub fn new(tavily_api_key: &str) -> Self {
        Websearch {
            tavily_api_key: tavily_api_key.to_string()
        }
    }
}

impl Tool for Websearch {
    const NAME: &'static str = "Web search";

    type Error = WebsearchError;
    type Args = WebsearchArgs;
    type Output = WebsearchOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "websearch".to_string(),
            description: "Search the web".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "The query to search for"
                    },
                }
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        println!("[tool-call] Search the web for {}", args.query);
        let result = reqwest::Client::new()
            .post(TAVILY_ENDPOINT)
            .json(&json!({"query": args.query}))
            .bearer_auth(self.tavily_api_key.to_owned())
            .send()
            .await?
            .json::<Self::Output>()
            .await?;
        Ok(result)
    }
}