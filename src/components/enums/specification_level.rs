use serde::{Serialize, Serializer};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SpecificationLevel {
    Minimum,
    BasicWithoutLines,
    Basic,
    En16931,
    XRechnung,
    Extended,
}

impl SpecificationLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            SpecificationLevel::Minimum => "urn:factur-x.eu:1p0:minimum",
            SpecificationLevel::BasicWithoutLines => "urn:factur-x.eu:1p0:basicwl",
            SpecificationLevel::Basic => "urn:cen.eu:en16931:2017#compliant#urn:factur-x.eu:1p0:basic",
            SpecificationLevel::En16931 => "urn:cen.eu:en16931:2017",
            SpecificationLevel::Extended => "urn:cen.eu:en16931:2017#conformant#urn:factur-x.eu:1p0:extended",
            SpecificationLevel::XRechnung => "urn:cen.eu:en16931:2017#compliant#urn:xeinkauf.de:kosit:xrechnung_3.0"
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