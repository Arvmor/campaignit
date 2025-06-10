use crate::{model::send_model_chat, statics::Statics};
use ollama_rs::Ollama;
use rand::distr::{Distribution, weighted::WeightedIndex};
use serde::Deserialize;
use std::error::Error;

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
    ($stats:ident, $field_name:ident) => {
        let weights = $stats.$field_name.values().collect::<Vec<_>>();

        // Weighted Random Choice
        let position = WeightedIndex::new(weights)?.sample(&mut rand::rng());
        let $field_name = $stats.$field_name.keys().nth(position).unwrap();
    };
}

impl<'a> Individual<'a> {
    /// Create a new individual
    pub fn new(stats: &'a Statics) -> Result<Self, Box<dyn Error>> {
        extract_stats!(stats, age);
        extract_stats!(stats, class);
        extract_stats!(stats, gender);
        extract_stats!(stats, region);
        extract_stats!(stats, property);
        extract_stats!(stats, religion);
        extract_stats!(stats, vehicle);
        extract_stats!(stats, education);
        extract_stats!(stats, marital_status);

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

    pub async fn ask_model(
        &self,
        client: &mut Ollama,
        prompt: String,
    ) -> Result<String, Box<dyn Error>> {
        let individual = format!(
            "You are an individual with the following information: {:?}",
            &self,
        );

        send_model_chat(client, prompt, individual).await
    }
}

/// The age of the individual
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub enum Age {
    Newborn,
    Child,
    Teenager,
    Adult,
    Senior,
}

/// The marital status of the individual
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub enum MaritalStatus {
    Single,
    Dating,
    Married,
    Divorced,
}

/// The education of the individual
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub enum Education {
    HighSchool,
    University,
    Postgraduate,
    NoEducation,
}

/// The religion of the individual
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub enum Religion {
    Atheist,
    Christian,
    Muslim,
    Jewish,
    Hindu,
}

/// The property of the individual
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub enum Property {
    Owned,
    Rented,
    Shared,
}

/// The gender of the individual
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub enum Gender {
    Male,
    TowardsMale,
    NonBinary,
    TowardsFemale,
    Female,
}

/// The region of the individual
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
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
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub enum Class {
    LowerClass,
    MiddleClass,
    UpperClass,
    WorkingClass,
    Elite,
}

/// The vehicle of the individual
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub enum Vehicle {
    MultiVehicle,
    LowTier,
    MiddleTier,
    HighTier,
    NoVehicle,
}
