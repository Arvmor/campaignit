use crate::{individuals::Individual, statics::Statics};
use std::{
    error::Error,
    time::{SystemTime, UNIX_EPOCH},
};

/// The world of the universe
pub struct World<'a> {
    /// The individuals of the world
    pub individuals: Vec<Individual<'a>>,
    /// The time of the world
    pub time: u64,
}

impl<'a> World<'a> {
    /// Create a new world
    pub fn new(stats: &'a Statics, population: usize) -> Result<Self, Box<dyn Error>> {
        let time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let mut individuals = Vec::with_capacity(population);

        // Create Individuals tied to the statics
        for _ in 0..population {
            let individual = Individual::new(stats)?;
            individuals.push(individual);
        }

        Ok(Self { individuals, time })
    }
}

#[cfg(test)]
mod universe_tests {
    use super::*;

    #[test]
    fn test_new_world() {
        let stats = Statics::default();
        let population = 0;
        let world = World::new(&stats, population);

        assert!(world.is_ok());
    }
}
