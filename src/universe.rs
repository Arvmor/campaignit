use std::{
    collections::HashMap,
    time::{SystemTime, SystemTimeError, UNIX_EPOCH},
};

use crate::individuals::Individual;

/// The world of the universe
pub struct World {
    /// The individuals of the world
    pub individuals: Vec<Individual>,
    /// The time of the world
    pub time: u64,
}

impl World {
    /// Create a new world
    pub fn new(stats: HashMap<String, u64>) -> Result<Self, SystemTimeError> {
        Ok(Self {
            individuals: Vec::new(),
            time: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
        })
    }
}

#[cfg(test)]
mod universe_tests {
    use super::*;

    #[test]
    fn test_new_world() {
        let stats = HashMap::new();
        let world = World::new(stats);

        assert!(world.is_ok());
    }
}
