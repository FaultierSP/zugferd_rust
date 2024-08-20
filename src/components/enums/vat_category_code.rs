use serde::{Serialize,Serializer};

#[derive(Clone, Copy, PartialEq)]
pub enum VATCategoryCode {
    StandardRate,
    ZeroRatedGoods,
    ExemptFromTax,
    VatReverseCharge,
    VatExemptEEAIntraCommunity,
    FreeExportItemTaxNotCharged,
    ServiceOutsideScopeOfTax,
    CanaryIslandsGeneralIndirectTax,
    TaxForProductionServicesImportationCeutaMelilla,
}

impl VATCategoryCode {
    pub fn as_str(&self) -> &str {
        match self {
            VATCategoryCode::StandardRate => "S",
            VATCategoryCode::ZeroRatedGoods => "Z",
            VATCategoryCode::ExemptFromTax => "E",
            VATCategoryCode::VatReverseCharge => "AE",
            VATCategoryCode::VatExemptEEAIntraCommunity => "K",
            VATCategoryCode::FreeExportItemTaxNotCharged => "G",
            VATCategoryCode::ServiceOutsideScopeOfTax => "O",
            VATCategoryCode::CanaryIslandsGeneralIndirectTax => "L",
            VATCategoryCode::TaxForProductionServicesImportationCeutaMelilla => "M",
        }
    }
}

impl Serialize for VATCategoryCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}