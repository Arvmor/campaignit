use serde::Deserialize;

pub struct Individual {
    /// The age of the individual
    pub age: Age,
    /// The class of the individual
    pub class: Class,
    /// The gender of the individual
    pub gender: Gender,
    /// The region of the individual
    pub region: Region,
    /// The property of the individual
    pub property: Property,
    /// The religion of the individual
    pub religion: Religion,
    /// The vehicle of the individual
    pub vehicle: Option<Vehicle>,
    /// The education of the individual
    pub education: Option<Education>,
    /// The marital status of the individual
    pub marital_status: MaritalStatus,
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
}
