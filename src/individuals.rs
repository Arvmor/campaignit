pub struct Individual {
    pub age: u8,
    pub income: u32,
    pub class: Class,
    pub gender: Gender,
    pub region: Region,
    pub property: Property,
    pub religion: Religion,
    pub vehicle: Option<Vehicle>,
    pub education: Option<Education>,
    pub marital_status: MaritalStatus,
}

pub enum MaritalStatus {
    Single,
    Dating,
    Married,
    Divorced,
}

pub enum Education {
    HighSchool,
    University,
    Postgraduate,
}

pub enum Religion {
    Atheist,
    Christian,
    Muslim,
    Jewish,
    Hindu,
}

pub enum Property {
    Owned,
    Rented,
    Shared,
}

pub enum Gender {
    Male,
    TowardsMale,
    NonBinary,
    TowardsFemale,
    Female,
}

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

pub enum Class {
    LowerClass,
    MiddleClass,
    UpperClass,
    WorkingClass,
    Elite,
}

pub enum Vehicle {
    MultiVehicle,
    LowTier,
    MiddleTier,
    HighTier,
}
