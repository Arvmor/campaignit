use std::error::Error;

pub mod individuals;
pub mod statics;
pub mod universe;

fn main() -> Result<(), Box<dyn Error>> {
    let statics = statics::Statics::from_file("statics.json")?;
    let universe = universe::World::new(&statics, 3_000)?;

    dbg!(universe.individuals.len());

    Ok(())
}
