use serde::{Serialize,Serializer};

#[derive(Clone, Copy)]
pub enum CountryCode {
    Germany,
    France,
    UnitedStates,
    UnitedKingdom,
    Italy,
    Spain,
}

impl CountryCode {
    pub fn as_str(&self) -> &str {
        match self {
            CountryCode::Germany => "DE",
            CountryCode::France => "FR",
            CountryCode::UnitedStates => "US",
            CountryCode::UnitedKingdom => "GB",
            CountryCode::Italy => "IT",
            CountryCode::Spain => "ES",
        }
    }
}

impl Serialize for CountryCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}