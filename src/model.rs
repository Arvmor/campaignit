use ollama_rs::{
    Ollama,
    generation::{
        chat::{ChatMessage, request::ChatMessageRequest},
        parameters::{FormatType, JsonStructure},
    },
};
use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
pub enum CampaignImpact {
    Positive,
    Negative,
    Neutral,
}

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
pub struct ResponseFormat {
    pub campaign_impact: CampaignImpact,
    pub reason: String,
}

/// Model type for the DeepSeek model
static MODEL: LazyLock<String> = LazyLock::new(|| "deepseek-r1:8b".to_string());

/// Response format for the model
static RESPONSE_FORMAT: LazyLock<FormatType> = LazyLock::new(|| {
    FormatType::StructuredJson(JsonStructure::new_for_schema(schema_for!(ResponseFormat)))
});

/// Sends a chat message to the Ollama model and returns the response.
pub async fn send_model_chat(
    client: &mut Ollama,
    prompt: String,
    system: String,
) -> Result<String, Box<dyn std::error::Error>> {
    // Create the system message and user prompt
    let history = vec![ChatMessage::system(system)];
    let prompt = vec![ChatMessage::user(prompt)];

    // Create a request
    let request = ChatMessageRequest::new(MODEL.clone(), prompt).format(RESPONSE_FORMAT.clone());
    let result = client
        .send_chat_messages_with_history(&mut history.clone(), request)
        .await?;

    Ok(result.message.content)
}
