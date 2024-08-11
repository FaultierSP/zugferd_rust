use serde::{Serialize,Serializer};

#[derive(Debug,Clone,Copy)]
pub enum CurrencyCode {
    Euro,
    BritishPound,
    SwissFranc,
    NorwegianKrone,
    SwedishKrona,
    DanishKrone,
    PolishZloty,
    HungarianForint,
    CzechKoruna,
    RomanianLeu,
    BulgarianLev,
    CroatianKuna,
}

impl CurrencyCode {
    pub fn as_str(&self) -> &str {
        match self {
            CurrencyCode::Euro => "EUR",
            CurrencyCode::BritishPound => "GBP",
            CurrencyCode::SwissFranc => "CHF",
            CurrencyCode::NorwegianKrone => "NOK",
            CurrencyCode::SwedishKrona => "SEK",
            CurrencyCode::DanishKrone => "DKK",
            CurrencyCode::PolishZloty => "PLN",
            CurrencyCode::HungarianForint => "HUF",
            CurrencyCode::CzechKoruna => "CZK",
            CurrencyCode::RomanianLeu => "RON",
            CurrencyCode::BulgarianLev => "BGN",
            CurrencyCode::CroatianKuna => "HRK",
        }
    }
}

impl Serialize for CurrencyCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}