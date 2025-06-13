use serde::{Serialize, Serializer};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AllowanceChargeReasonCode {
    BonusForWorksAheadOfSchedule,
    OtherBonus,
    ManufacturersConsumerDiscount,
    DueToMilitaryStatus,
    DueToWorkAccident,
    SpecialAgreement,
    ProductionErrorDiscount,
    NewOutletDiscount,
    SampleDiscount,
    EndOfRangeDiscount,
    IncotermDiscount,
    PointOfSalesThresholdAllowance,
    MaterialSurchargeDeduction,
    Discount,
    SpecialRebate,
    FixedLongTerm,
    Temporary,
    Standard,
    YearlyTurnover,
}

impl AllowanceChargeReasonCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            AllowanceChargeReasonCode::BonusForWorksAheadOfSchedule => "41",
            AllowanceChargeReasonCode::OtherBonus => "42",
            AllowanceChargeReasonCode::ManufacturersConsumerDiscount => "60",
            AllowanceChargeReasonCode::DueToMilitaryStatus => "62",
            AllowanceChargeReasonCode::DueToWorkAccident => "63",
            AllowanceChargeReasonCode::SpecialAgreement => "64",
            AllowanceChargeReasonCode::ProductionErrorDiscount => "65",
            AllowanceChargeReasonCode::NewOutletDiscount => "66",
            AllowanceChargeReasonCode::SampleDiscount => "66",
            AllowanceChargeReasonCode::EndOfRangeDiscount => "68",
            AllowanceChargeReasonCode::IncotermDiscount => "70",
            AllowanceChargeReasonCode::PointOfSalesThresholdAllowance => "71",
            AllowanceChargeReasonCode::MaterialSurchargeDeduction => "88",
            AllowanceChargeReasonCode::Discount => "95",
            AllowanceChargeReasonCode::SpecialRebate => "100",
            AllowanceChargeReasonCode::FixedLongTerm => "102",
            AllowanceChargeReasonCode::Temporary => "103",
            AllowanceChargeReasonCode::Standard => "104",
            AllowanceChargeReasonCode::YearlyTurnover => "105",
        }
    }
}

impl Serialize for AllowanceChargeReasonCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
