use super::agreement::ApplicableHeaderTradeAgreement;
use super::agreement::{ObjectReference, ReferenceView};
use super::context_document::DateTimeString;
use super::delivery::ApplicableHeaderTradeDelivery;
use super::serde_helpers::f64_format;
use super::serde_helpers::f64_format_with_precision_4;
use super::serde_helpers::format_f64_option;
use super::serde_helpers::vector_is_empty;
use super::settlement::ApplicableHeaderTradeSettlement;
use super::settlement::ApplicableTradeTax;
use super::settlement::{ChargeView, TradeAllowanceCharge};
use crate::components::enums::{
    allowance_charge_reason_code::AllowanceChargeReasonCode, country_code::CountryCode,
    identifier_scheme_code::IdentifierSchemeCode, unit_code::UnitCode,
};
use chrono::NaiveDate;
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
    #[serde(rename = "ram:SpecifiedTradeAllowanceCharge")]
    pub specified_trade_allowance_charge: Vec<SpecifiedLineTradeAllowanceCharge<'invoice>>,
    #[serde(rename = "ram:SpecifiedTradeSettlementLineMonetarySummation")]
    pub specified_trade_settlement_line_monetary_summation:
        SpecifiedTradeSettlementLineMonetarySummation,
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

#[derive(Serialize, Clone, Debug)]
pub struct ProductCharacteristic<'a> {
    pub name: &'a str,
    pub value: &'a str,
}

#[derive(Serialize, Clone, Debug)]
pub struct ProductClassification<'a> {
    pub code: &'a str,
    pub list_id: &'a str,
    pub list_version_id: Option<&'a str>,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct InvoiceLineDetails<'a> {
    pub seller_assigned_id: Option<&'a str>,
    pub buyer_assigned_id: Option<&'a str>,
    pub origin_country: Option<CountryCode>,
    pub classifications: Vec<ProductClassification<'a>>,
    pub characteristics: Vec<ProductCharacteristic<'a>>,
    pub buyer_order_line_id: Option<&'a str>,
    pub object_reference: Option<ObjectReference<'a>>,
    pub period_start: Option<NaiveDate>,
    pub period_end: Option<NaiveDate>,
    pub charges: Vec<TradeAllowanceCharge<'a>>,
    pub buyer_accounting_reference: Option<&'a str>,
}

#[derive(Serialize)]
struct DateTimeView<'a> {
    #[serde(rename = "udt:DateTimeString")]
    value: DateTimeString<'a>,
}

#[derive(Serialize)]
struct PeriodView<'a> {
    #[serde(rename = "ram:StartDateTime", skip_serializing_if = "Option::is_none")]
    start: Option<DateTimeView<'a>>,
    #[serde(rename = "ram:EndDateTime", skip_serializing_if = "Option::is_none")]
    end: Option<DateTimeView<'a>>,
}

#[derive(Serialize)]
struct ProductCharacteristicView<'a> {
    #[serde(rename = "ram:Description")]
    name: &'a str,
    #[serde(rename = "ram:Value")]
    value: &'a str,
}

#[derive(Serialize)]
struct ClassificationCode<'a> {
    #[serde(rename = "@listID")]
    list_id: &'a str,
    #[serde(rename = "@listVersionID", skip_serializing_if = "Option::is_none")]
    list_version_id: Option<&'a str>,
    #[serde(rename = "$value")]
    value: &'a str,
}

#[derive(Serialize)]
struct ClassificationView<'a> {
    #[serde(rename = "ram:ClassCode")]
    code: ClassificationCode<'a>,
}

#[derive(Serialize)]
struct CountryView {
    #[serde(rename = "ram:ID")]
    id: CountryCode,
}

#[derive(Serialize)]
struct ProductView<'s, 'a> {
    #[serde(rename = "ram:GlobalID", skip_serializing_if = "Option::is_none")]
    global_id: &'s Option<GlobalID<'a>>,
    #[serde(
        rename = "ram:SellerAssignedID",
        skip_serializing_if = "Option::is_none"
    )]
    seller_id: Option<&'a str>,
    #[serde(
        rename = "ram:BuyerAssignedID",
        skip_serializing_if = "Option::is_none"
    )]
    buyer_id: Option<&'a str>,
    #[serde(rename = "ram:Name")]
    name: &'a str,
    #[serde(rename = "ram:Description", skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(rename = "ram:ApplicableProductCharacteristic")]
    characteristics: Vec<ProductCharacteristicView<'a>>,
    #[serde(rename = "ram:DesignatedProductClassification")]
    classifications: Vec<ClassificationView<'a>>,
    #[serde(
        rename = "ram:OriginTradeCountry",
        skip_serializing_if = "Option::is_none"
    )]
    origin: Option<CountryView>,
}

#[derive(Serialize)]
struct LineAgreementView<'s, 'a> {
    #[serde(
        rename = "ram:BuyerOrderReferencedDocument",
        skip_serializing_if = "Option::is_none"
    )]
    buyer_order: Option<ReferenceView<'a>>,
    #[serde(
        rename = "ram:GrossPriceProductTradePrice",
        skip_serializing_if = "Option::is_none"
    )]
    gross_price: &'s Option<GrossPriceProductTradePrice>,
    #[serde(rename = "ram:NetPriceProductTradePrice")]
    net_price: &'s NetPriceProductTradePrice,
}

#[derive(Serialize)]
struct AccountingAccount<'a> {
    #[serde(rename = "ram:ID")]
    id: &'a str,
}

#[derive(Serialize)]
struct LineSettlementView<'s, 'a> {
    #[serde(rename = "ram:ApplicableTradeTax")]
    tax: &'s ApplicableTradeTax<'a>,
    #[serde(
        rename = "ram:BillingSpecifiedPeriod",
        skip_serializing_if = "Option::is_none"
    )]
    period: Option<PeriodView<'a>>,
    #[serde(rename = "ram:SpecifiedTradeAllowanceCharge")]
    allowances: &'s Vec<SpecifiedLineTradeAllowanceCharge<'a>>,
    #[serde(rename = "ram:SpecifiedTradeAllowanceCharge")]
    charges: Vec<ChargeView<'s, 'a>>,
    #[serde(rename = "ram:SpecifiedTradeSettlementLineMonetarySummation")]
    totals: &'s SpecifiedTradeSettlementLineMonetarySummation,
    #[serde(
        rename = "ram:AdditionalReferencedDocument",
        skip_serializing_if = "Option::is_none"
    )]
    object_reference: Option<ReferenceView<'a>>,
    #[serde(
        rename = "ram:ReceivableSpecifiedTradeAccountingAccount",
        skip_serializing_if = "Option::is_none"
    )]
    accounting_account: Option<AccountingAccount<'a>>,
}

#[derive(Serialize)]
pub(super) struct LineView<'s, 'a> {
    #[serde(rename = "ram:AssociatedDocumentLineDocument")]
    document: &'s AssociatedDocumentLineDocument<'a>,
    #[serde(rename = "ram:SpecifiedTradeProduct")]
    product: ProductView<'s, 'a>,
    #[serde(rename = "ram:SpecifiedLineTradeAgreement")]
    agreement: LineAgreementView<'s, 'a>,
    #[serde(rename = "ram:SpecifiedLineTradeDelivery")]
    delivery: &'s SpecifiedLineTradeDelivery,
    #[serde(rename = "ram:SpecifiedLineTradeSettlement")]
    settlement: LineSettlementView<'s, 'a>,
}

impl<'s, 'a> LineView<'s, 'a> {
    pub(super) fn new(
        base: &'s IncludedSupplyChainTradeLineItem<'a>,
        details: Option<&'s InvoiceLineDetails<'a>>,
    ) -> Self {
        let product = &base.specified_trade_product;
        let agreement = &base.specified_line_trade_agreement;
        let settlement = &base.specified_line_trade_settlement;
        let period = details.and_then(|d| {
            if d.period_start.is_none() && d.period_end.is_none() {
                return None;
            }
            Some(PeriodView {
                start: d.period_start.map(|date| DateTimeView {
                    value: DateTimeString::new(date),
                }),
                end: d.period_end.map(|date| DateTimeView {
                    value: DateTimeString::new(date),
                }),
            })
        });
        Self {
            document: &base.associated_document_line_document,
            product: ProductView {
                global_id: &product.global_id,
                seller_id: details.and_then(|d| d.seller_assigned_id),
                buyer_id: details.and_then(|d| d.buyer_assigned_id),
                name: product.name,
                description: product.description,
                characteristics: details
                    .map(|d| {
                        d.characteristics
                            .iter()
                            .map(|c| ProductCharacteristicView {
                                name: c.name,
                                value: c.value,
                            })
                            .collect()
                    })
                    .unwrap_or_default(),
                classifications: details
                    .map(|d| {
                        d.classifications
                            .iter()
                            .map(|c| ClassificationView {
                                code: ClassificationCode {
                                    list_id: c.list_id,
                                    list_version_id: c.list_version_id,
                                    value: c.code,
                                },
                            })
                            .collect()
                    })
                    .unwrap_or_default(),
                origin: details
                    .and_then(|d| d.origin_country)
                    .map(|id| CountryView { id }),
            },
            agreement: LineAgreementView {
                buyer_order: details
                    .and_then(|d| d.buyer_order_line_id)
                    .map(ReferenceView::line),
                gross_price: &agreement.gross_price_product_trade_price,
                net_price: &agreement.net_price_product_trade_price,
            },
            delivery: &base.specified_line_trade_delivery,
            settlement: LineSettlementView {
                tax: &settlement.applicable_trade_tax,
                period,
                allowances: &settlement.specified_trade_allowance_charge,
                charges: details
                    .map(|d| d.charges.iter().map(ChargeView::from).collect())
                    .unwrap_or_default(),
                totals: &settlement.specified_trade_settlement_line_monetary_summation,
                object_reference: details
                    .and_then(|d| d.object_reference.as_ref())
                    .map(|r| ReferenceView::id(r.id, Some("130"), r.scheme_id)),
                accounting_account: details
                    .and_then(|d| d.buyer_accounting_reference)
                    .map(|id| AccountingAccount { id }),
            },
        }
    }
}
