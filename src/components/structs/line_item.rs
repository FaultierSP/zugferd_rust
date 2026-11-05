use super::agreement::ApplicableHeaderTradeAgreement;
use super::delivery::ApplicableHeaderTradeDelivery;
use super::serde_helpers::f64_format;
use super::serde_helpers::f64_format_with_precision_4;
use super::serde_helpers::format_f64_option;
use super::serde_helpers::vector_is_empty;
use super::settlement::ApplicableHeaderTradeSettlement;
use super::settlement::ApplicableTradeTax;
use crate::components::enums::{
    allowance_charge_reason_code::AllowanceChargeReasonCode,
    identifier_scheme_code::IdentifierSchemeCode, unit_code::UnitCode,
};
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct SupplyChainTradeTransaction<'invoice> {
    #[serde(
        rename = "ram:IncludedSupplyChainTradeLineItem",
        skip_serializing_if = "vector_is_empty"
    )]
    pub included_supply_chain_trade_line_items: Vec<IncludedSupplyChainTradeLineItem<'invoice>>,
    #[serde(rename = "ram:ApplicableHeaderTradeAgreement")]
    pub applicable_header_trade_agreement: ApplicableHeaderTradeAgreement<'invoice>,
    #[serde(rename = "ram:ApplicableHeaderTradeDelivery")]
    pub applicable_header_trade_delivery: ApplicableHeaderTradeDelivery<'invoice>,
    #[serde(rename = "ram:ApplicableHeaderTradeSettlement")]
    pub applicable_header_trade_settlement: ApplicableHeaderTradeSettlement<'invoice>,
}

/// Contains information about a sigle line item on the invoice
///
/// BG-25
#[derive(Serialize, Clone, Debug)]
pub struct IncludedSupplyChainTradeLineItem<'invoice> {
    #[serde(rename = "ram:AssociatedDocumentLineDocument")]
    pub associated_document_line_document: AssociatedDocumentLineDocument<'invoice>,
    #[serde(rename = "ram:SpecifiedTradeProduct")]
    pub specified_trade_product: SpecifiedTradeProduct<'invoice>,
    #[serde(rename = "ram:SpecifiedLineTradeAgreement")]
    pub specified_line_trade_agreement: SpecifiedLineTradeAgreement,
    #[serde(rename = "ram:SpecifiedLineTradeDelivery")]
    pub specified_line_trade_delivery: SpecifiedLineTradeDelivery,
    #[serde(rename = "ram:SpecifiedLineTradeSettlement")]
    pub specified_line_trade_settlement: SpecifiedLineTradeSettlement<'invoice>,
}

/// Groups general line item information
///
/// BT-126-00
#[derive(Serialize, Clone, Debug)]
pub struct AssociatedDocumentLineDocument<'invoice> {
    /// A unique identifier for this line item in the invoice
    ///
    /// BT-126
    #[serde(rename = "ram:LineID")]
    pub line_id: &'invoice str,
    /// Unstructured additional information that are relevant to this line item
    ///
    /// BT-127-00
    #[serde(rename = "ram:IncludedNote", skip_serializing_if = "Option::is_none")]
    pub included_note: Option<&'invoice str>,
}

/// Groups information about the goods and services of this line item
///
/// BG-31
#[derive(Serialize, Clone, Debug)]
pub struct SpecifiedTradeProduct<'invoice> {
    /// Identifier for this item that is valid across invoices
    ///
    /// BT-157
    #[serde(rename = "ram:GlobalID", skip_serializing_if = "Option::is_none")]
    /// A unique identifier for this item that is valid across invoices
    pub global_id: Option<GlobalID<'invoice>>,
    /// Name of the article
    ///
    /// BT-153
    #[serde(rename = "ram:Name")]
    /// The name of the article
    pub name: &'invoice str,
    /// BT-154
    #[serde(rename = "ram:Description", skip_serializing_if = "Option::is_none")]
    /// A description of the article
    pub description: Option<&'invoice str>,
}

#[derive(Serialize, Clone, Debug)]
pub struct GlobalID<'invoice> {
    #[serde(rename = "@schemeID")]
    pub scheme_id: IdentifierSchemeCode,
    #[serde(rename = "$value")]
    pub value: &'invoice str,
}

impl<'invoice> GlobalID<'invoice> {
    pub fn new(scheme_id: IdentifierSchemeCode, value: &'invoice str) -> Self {
        Self { scheme_id, value }
    }
}

/// Groups pricing information about the line item
///
/// BG-29
#[derive(Serialize, Clone, Debug)]
pub struct SpecifiedLineTradeAgreement {
    #[serde(
        rename = "ram:GrossPriceProductTradePrice",
        skip_serializing_if = "Option::is_none"
    )]
    pub gross_price_product_trade_price: Option<GrossPriceProductTradePrice>,
    #[serde(rename = "ram:NetPriceProductTradePrice")]
    pub net_price_product_trade_price: NetPriceProductTradePrice,
}

/// BT-148-00
#[derive(Serialize, Clone, Debug)]
pub struct GrossPriceProductTradePrice {
    /// The item price without VAT before deductions
    ///
    /// BR-28
    #[serde(rename = "ram:ChargeAmount", serialize_with = "f64_format")]
    pub charge_amount: f64,
}

/// The item price without vat with deductions and charges
///
/// BT-146-00
#[derive(Serialize, Clone, Debug)]
pub struct NetPriceProductTradePrice {
    /// Price of one item without VAT and after deductions
    ///
    /// BT-146
    #[serde(rename = "ram:ChargeAmount", serialize_with = "f64_format")]
    pub charge_amount: f64,
}

/// Groups delivery information about the line item
///
/// BT-129-00
#[derive(Serialize, Clone, Debug)]
pub struct SpecifiedLineTradeDelivery {
    #[serde(rename = "ram:BilledQuantity")]
    pub billed_quantity: BilledQuantity,
}

/// amount of the articles that are invoiced
///
/// BT-129
#[derive(Serialize, Clone, Debug)]
pub struct BilledQuantity {
    /// Unit of the quantity
    ///
    /// BT-130
    #[serde(rename = "@unitCode")]
    pub unit_code: UnitCode,
    /// Amount of the quantity
    #[serde(rename = "$value", serialize_with = "f64_format_with_precision_4")]
    pub value: f64,
}

impl BilledQuantity {
    pub fn new(unit_code: UnitCode, value: f64) -> Self {
        Self { unit_code, value }
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct SpecifiedLineTradeSettlement<'invoice> {
    #[serde(rename = "ram:ApplicableTradeTax")]
    pub applicable_trade_tax: ApplicableTradeTax<'invoice>,
    #[serde(rename = "ram:SpecifiedTradeSettlementLineMonetarySummation")]
    pub specified_trade_settlement_line_monetary_summation:
        SpecifiedTradeSettlementLineMonetarySummation,
    #[serde(rename = "ram:SpecifiedTradeAllowanceCharge")]
    pub specified_trade_allowance_charge: Vec<SpecifiedLineTradeAllowanceCharge<'invoice>>,
}

#[derive(Serialize, Clone, Debug)]
pub struct SpecifiedLineTradeAllowanceCharge<'invoice> {
    #[serde(rename = "ram:ChargeIndicator")]
    pub charge_indicator: ChargeIndicator,
    #[serde(
        rename = "ram:CalculationPercent",
        serialize_with = "format_f64_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub calculation_percent: Option<f64>,
    #[serde(
        rename = "ram:BasisAmount",
        serialize_with = "format_f64_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub basis_amount: Option<f64>,
    #[serde(rename = "ram:ActualAmount", serialize_with = "f64_format")]
    pub actual_amount: f64,
    #[serde(rename = "ram:ReasonCode", skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<AllowanceChargeReasonCode>,
    #[serde(rename = "ram:Reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<&'invoice str>,
}

#[derive(Serialize, Clone, Debug)]
pub struct ChargeIndicator {
    #[serde(rename = "udt:Indicator")]
    pub indicator: bool,
}

#[derive(Serialize, Clone, Debug)]
pub struct SpecifiedTradeSettlementLineMonetarySummation {
    #[serde(rename = "ram:LineTotalAmount", serialize_with = "f64_format")]
    pub line_total_amount: f64,
}
