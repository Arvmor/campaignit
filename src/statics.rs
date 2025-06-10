use crate::individuals::{
    Age, Class, Education, Gender, MaritalStatus, Property, Region, Religion, Vehicle,
};
use serde::Deserialize;
use std::{collections::BTreeMap, error::Error, fs::File};

/// The statics of the universe
/// Parses from a CSV file
#[derive(Debug, Default, Deserialize)]
pub struct Statics {
    pub age: BTreeMap<Age, u8>,
    pub class: BTreeMap<Class, u8>,
    pub gender: BTreeMap<Gender, u8>,
    pub region: BTreeMap<Region, u8>,
    pub property: BTreeMap<Property, u8>,
    pub religion: BTreeMap<Religion, u8>,
    pub vehicle: BTreeMap<Vehicle, u8>,
    pub education: BTreeMap<Education, u8>,
    pub marital_status: BTreeMap<MaritalStatus, u8>,
}

impl Statics {
    /// Parse the CSV file and return a HashMap of Statics and their rates
    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        // Read the JSON file
        let file = File::open(path)?;
        let statics: Statics = serde_json::from_reader(file)?;

        Ok(statics)
    }
}

#[cfg(test)]
mod statics_tests {
    use super::*;

    #[test]
    fn test_statics_from_file() {
        let statics = Statics::from_file("statics.json");
        println!("{:?}", statics);
        assert!(statics.is_ok());
    }
}
