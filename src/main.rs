use ollama_rs::Ollama;
use std::error::Error;

pub mod individuals;
pub mod model;
pub mod statics;
pub mod universe;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Ollama Client
    let mut ollama = Ollama::default();
    let prompt = "Apple is launching their new iPhone 17 with 75% discount. How would this marketing campaign affect you during the lunch-day and first month? note: explain in a short paragraph.".to_string();

    // Initialize Universe
    let statics = statics::Statics::from_file("statics.json")?;
    let universe = universe::World::new(&statics, 100)?;

    // Gether Results per individual
    for individual in universe.individuals {
        let result = individual.ask_model(&mut ollama, prompt.clone()).await?;
        println!("Response:\t{:?}", result);
    }

    Ok(())
}
