pub struct Individual {
    /// The age of the individual
    pub age: u8,
    /// The income of the individual
    pub income: u32,
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

/// The marital status of the individual
pub enum MaritalStatus {
    Single,
    Dating,
    Married,
    Divorced,
}

/// The education of the individual
pub enum Education {
    HighSchool,
    University,
    Postgraduate,
}

/// The religion of the individual
pub enum Religion {
    Atheist,
    Christian,
    Muslim,
    Jewish,
    Hindu,
}

/// The property of the individual
pub enum Property {
    Owned,
    Rented,
    Shared,
}

/// The gender of the individual
pub enum Gender {
    Male,
    TowardsMale,
    NonBinary,
    TowardsFemale,
    Female,
}

/// The region of the individual
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
pub enum Class {
    LowerClass,
    MiddleClass,
    UpperClass,
    WorkingClass,
    Elite,
}

/// The vehicle of the individual
pub enum Vehicle {
    MultiVehicle,
    LowTier,
    MiddleTier,
    HighTier,
}
