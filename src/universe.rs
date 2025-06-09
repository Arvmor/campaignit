use std::{
    collections::HashMap,
    time::{SystemTime, SystemTimeError, UNIX_EPOCH},
};

use crate::individuals::Individual;

pub struct World {
    pub individuals: Vec<Individual>,
    pub time: u64,
}

impl World {
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
