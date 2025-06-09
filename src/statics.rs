use crate::individuals::{
    Age, Class, Education, Gender, MaritalStatus, Property, Region, Religion, Vehicle,
};
use serde::Deserialize;
use std::{collections::HashMap, error::Error};

/// To Parse the CSV file
#[derive(Debug, Deserialize)]
struct Fields {
    #[serde(flatten)]
    field: Statics,
    rate: u64,
}

/// The statics of the universe
/// Parses from a CSV file
#[derive(Debug, PartialEq, Eq, Hash, Deserialize)]
pub enum Statics {
    Age(Age),
    Class(Class),
    Gender(Gender),
    Region(Region),
    Property(Property),
    Religion(Religion),
    Vehicle(Vehicle),
    Education(Education),
    MaritalStatus(MaritalStatus),
}

impl Statics {
    /// Parse the CSV file and return a HashMap of Statics and their rates
    pub fn from_csv(path: &str) -> Result<HashMap<Self, u64>, Box<dyn Error>> {
        // Initialize the statics
        let mut statics = HashMap::new();

        // Read the CSV file
        let mut rdr = csv::Reader::from_path(path)?;
        for result in rdr.deserialize() {
            let Fields { field, rate } = result?;
            statics.insert(field, rate);
        }

        Ok(statics)
    }
}

#[cfg(test)]
mod statics_tests {
    use super::*;

    #[test]
    fn test_statics_from_csv() {
        let statics = Statics::from_csv("statics.csv");
        assert!(statics.is_ok());
    }
}
