use crate::statics::Statics;
use rand::distr::{Distribution, weighted::WeightedIndex};
use serde::Deserialize;
use std::{collections::HashMap, error::Error};

/// The individual of the universe
#[derive(Debug)]
pub struct Individual<'a> {
    /// The age of the individual
    pub age: &'a Age,
    /// The class of the individual
    pub class: &'a Class,
    /// The gender of the individual
    pub gender: &'a Gender,
    /// The region of the individual
    pub region: &'a Region,
    /// The property of the individual
    pub property: &'a Property,
    /// The religion of the individual
    pub religion: &'a Religion,
    /// The vehicle of the individual
    pub vehicle: &'a Vehicle,
    /// The education of the individual
    pub education: &'a Education,
    /// The marital status of the individual
    pub marital_status: &'a MaritalStatus,
}

// Macro to extract stats for the fields
macro_rules! extract_stats {
    ($stats:ident, $field:ident, $field_name:ident) => {
        let mut samples = Vec::new();
        let mut weights = Vec::new();

        for (f, w) in $stats {
            if let Statics::$field(f) = f {
                samples.push(f);
                weights.push(w);
            }
        }

        // Weighted Random Choice
        let position = WeightedIndex::new(weights)?.sample(&mut rand::rng());
        let $field_name = samples[position];
    };
}

impl<'a> Individual<'a> {
    /// Create a new individual
    pub fn new(stats: &'a HashMap<Statics, u64>) -> Result<Self, Box<dyn Error>> {
        extract_stats!(stats, Age, age);
        extract_stats!(stats, Class, class);
        extract_stats!(stats, Gender, gender);
        extract_stats!(stats, Region, region);
        extract_stats!(stats, Property, property);
        extract_stats!(stats, Religion, religion);
        extract_stats!(stats, Vehicle, vehicle);
        extract_stats!(stats, Education, education);
        extract_stats!(stats, MaritalStatus, marital_status);

        Ok(Self {
            age,
            class,
            gender,
            region,
            property,
            religion,
            vehicle,
            education,
            marital_status,
        })
    }
}

/// The age of the individual
#[derive(Debug, PartialEq, Eq, Hash, Deserialize)]
pub enum Age {
    Newborn,
    Child,
    Teenager,
    Adult,
    Senior,
}

/// The marital status of the individual
#[derive(Debug, PartialEq, Eq, Hash, Deserialize)]
pub enum MaritalStatus {
    Single,
    Dating,
    Married,
    Divorced,
}

/// The education of the individual
#[derive(Debug, PartialEq, Eq, Hash, Deserialize)]
pub enum Education {
    HighSchool,
    University,
    Postgraduate,
    NoEducation,
}

/// The religion of the individual
#[derive(Debug, PartialEq, Eq, Hash, Deserialize)]
pub enum Religion {
    Atheist,
    Christian,
    Muslim,
    Jewish,
    Hindu,
}

/// The property of the individual
#[derive(Debug, PartialEq, Eq, Hash, Deserialize)]
pub enum Property {
    Owned,
    Rented,
    Shared,
}

/// The gender of the individual
#[derive(Debug, PartialEq, Eq, Hash, Deserialize)]
pub enum Gender {
    Male,
    TowardsMale,
    NonBinary,
    TowardsFemale,
    Female,
}

/// The region of the individual
#[derive(Debug, PartialEq, Eq, Hash, Deserialize)]
pub enum Region {
    NorthernAfrica,
    SubSaharanAfrica,
    CentralAsia,
    EasternAsia,
    SouthernAsia,
    SouthEasternAsia,
    WesternAsia,
    NorthernEurope,
    SouthernEurope,
    WesternEurope,
    EasternEurope,
    NorthernAmerica,
    CentralAmerica,
    SouthAmerica,
    Oceania,
    Antarctica,
}

/// The class of the individual
#[derive(Debug, PartialEq, Eq, Hash, Deserialize)]
pub enum Class {
    LowerClass,
    MiddleClass,
    UpperClass,
    WorkingClass,
    Elite,
}

/// The vehicle of the individual
#[derive(Debug, PartialEq, Eq, Hash, Deserialize)]
pub enum Vehicle {
    MultiVehicle,
    LowTier,
    MiddleTier,
    HighTier,
    NoVehicle,
}
