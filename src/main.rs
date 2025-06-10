use std::error::Error;
use ollama_rs::{generation::{chat::{request::ChatMessageRequest, ChatMessage, MessageRole}, parameters::{ FormatType, JsonStructure}}, Ollama};
use serde::{Deserialize, Serialize};
use schemars::{schema_for, JsonSchema};

pub mod individuals;
pub mod statics;
pub mod universe;

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
enum CampaignImpact {
    Positive,
    Negative,
    Neutral,
}

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
struct ResponseFormat {
    campaign_impact: CampaignImpact,
    reason: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Ollama Client
    let mut ollama = Ollama::new("http://10.0.0.9".to_string(), 11434);
    let model = "deepseek-r1:8b".to_string();

    // Marketing Campaign
    let campaign = "Apple is launching their new iPhone 17 with 75% discount.".to_string();
    let format = FormatType::StructuredJson(JsonStructure::new_for_schema(schema_for!(ResponseFormat)));
    let history = vec![ChatMessage::new(MessageRole::System, campaign)];

    // Initialize Universe
    let statics = statics::Statics::from_file("statics.json")?;
    let universe = universe::World::new(&statics, 3)?;

    // Gether Results per individual
    for individual in universe.individuals {
        let prompt = ChatMessage::assistant(format!("You are an individual with the following information: {:?}\nHow would this marketing campaign affect you during the lunch-day and first month? note: explain in a short paragraph.", individual));
        let request = ChatMessageRequest::new(model.clone(), vec![prompt])
            .format(format.clone());

        let result = ollama.send_chat_messages_with_history(&mut history.clone(), request).await?;
        println!("Response: \t{}\n", result.message.content);
    }

    Ok(())
}
