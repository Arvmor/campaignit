use crate::{individuals::Individual, statics::Statics};
use std::{
    collections::HashMap,
    time::{SystemTime, SystemTimeError, UNIX_EPOCH},
};

/// The world of the universe
pub struct World {
    /// The individuals of the world
    pub individuals: Vec<Individual>,
    /// The time of the world
    pub time: u64,
}

impl World {
    /// Create a new world
    pub fn new(stats: HashMap<Statics, u64>, population: u64) -> Result<Self, SystemTimeError> {
        let time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let mut individuals = Vec::new();

        // Create Individuals tied to the statics

        Ok(Self { individuals, time })
    }
}

#[cfg(test)]
mod universe_tests {
    use super::*;

    #[test]
    fn test_new_world() {
        let stats = HashMap::new();
        let world = World::new(stats, 1);

        assert!(world.is_ok());
    }
}
