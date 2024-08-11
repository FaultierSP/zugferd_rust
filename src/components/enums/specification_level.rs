use serde::{Serialize, Serializer};

#[derive(Debug, Clone, Copy)]
pub enum SpecificationLevel {
    Basic,
    En16931,
    Extended,
    Minimum,
    BasicWithLess,
}

impl SpecificationLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            SpecificationLevel::Basic => "urn:factur-x.eu:1p0:basic",
            SpecificationLevel::En16931 => "urn:factur-x.eu:1p0:en16931",
            SpecificationLevel::Extended => "urn:factur-x.eu:1p0:extended",
            SpecificationLevel::Minimum => "urn:factur-x.eu:1p0:minimum",
            SpecificationLevel::BasicWithLess => "urn:factur-x.eu:1p0:basicwl",
        }
    }
}

impl Serialize for SpecificationLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}