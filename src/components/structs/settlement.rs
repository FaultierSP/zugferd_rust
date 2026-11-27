use super::context_document::DateTimeString;
use super::invoice::AdditionalInvoiceDetails;
use super::line_item::ChargeIndicator;
use super::payment::{SpecifiedTradePaymentTerms, SpecifiedTradeSettlementPaymentMeans};
use super::serde_helpers::f64_format;
use super::serde_helpers::format_f64_option;
use crate::components::enums::{
    allowance_charge_reason_code::AllowanceChargeReasonCode, charge_codes_untdid7161::ChargeCode,
    currency_code::CurrencyCode, vat_category_code::VATCategoryCode,
};
use chrono::NaiveDate;
use serde::{Serialize, Serializer};

#[derive(Serialize, Clone, Debug)]
pub struct ApplicableHeaderTradeSettlement<'invoice> {
    #[serde(rename = "ram:InvoiceCurrencyCode")]
    pub invoice_currency_code: CurrencyCode,

    #[serde(rename = "ram:SpecifiedTradeSettlementPaymentMeans")]
    pub specified_trade_settlement_payment_means:
        Vec<SpecifiedTradeSettlementPaymentMeans<'invoice>>,

    #[serde(
        rename = "ram:ApplicableTradeTax",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub applicable_trade_tax: Vec<ApplicableTradeTax<'invoice>>,

    #[serde(rename = "ram:SpecifiedTradeAllowanceCharge")]
    pub specified_trade_allowance_charge: Vec<SpecifiedTradeAllowanceCharge<'invoice>>,

    #[serde(
        rename = "ram:SpecifiedTradePaymentTerms",
        skip_serializing_if = "Option::is_none"
    )]
    pub specified_trade_payment_terms: Option<SpecifiedTradePaymentTerms<'invoice>>,

    #[serde(rename = "ram:SpecifiedTradeSettlementHeaderMonetarySummation")]
    pub specified_trade_settlement_header_monetary_summation:
        SpecifiedTradeSettlementHeaderMonetarySummation,
}

#[derive(Serialize, Clone, Debug)]
pub struct SpecifiedTradeAllowanceCharge<'invoice> {
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
    #[serde(rename = "ram:CategoryTradeTax")]
    pub category_trade_tax: CategoryTradeTax<'invoice>,
}

#[derive(Serialize, Clone, Copy, Debug)]
pub struct CategoryTradeTax<'invoice> {
    #[serde(rename = "ram:TypeCode")]
    pub type_code: &'invoice str,
    #[serde(rename = "ram:CategoryCode")]
    pub category_code: VATCategoryCode,
    #[serde(
        rename = "ram:RateApplicablePercent",
        serialize_with = "format_f64_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub rate_applicable_percent: Option<f64>,
}

/// A set of financial information that contains information about the VAT in different categories, sets and exemption reasons
///
/// BG-23
#[derive(Serialize, Clone, Copy, Debug)]
pub struct ApplicableTradeTax<'invoice> {
    /// The total amount of tax that has to be paid for this tax category
    ///
    /// BT-117
    #[serde(
        rename = "ram:CalculatedAmount",
        serialize_with = "format_f64_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub calculated_amount: Option<f64>,
    /// Should always be "VAT"
    ///
    /// BT-118-0
    #[serde(rename = "ram:TypeCode")]
    pub type_code: &'invoice str,
    /// Reason why this amount is excempt from VAT or why no VAT is calculated
    ///
    /// BT-120
    #[serde(
        rename = "ram:ExemptionReason",
        skip_serializing_if = "Option::is_none"
    )]
    pub exemption_reason: Option<&'invoice str>,
    /// Sum of all netto amoounts
    ///
    /// BT-116
    #[serde(
        rename = "ram:BasisAmount",
        serialize_with = "format_f64_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub basis_amount: Option<f64>,
    /// Identifies a VAT category, has to match [Self::rate_applicable_percent]
    ///
    /// BT 118
    #[serde(rename = "ram:CategoryCode")]
    pub category_code: VATCategoryCode,
    /// Identifies the reason why VAT is not applied
    ///
    /// BT-121
    #[serde(
        rename = "ram:ExemptionReasonCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub exemption_reason_code: Option<&'invoice str>,
    /// VAT percentage, has to match [Self::category_code]
    ///
    /// BT-119
    #[serde(
        rename = "ram:RateApplicablePercent",
        serialize_with = "format_f64_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub rate_applicable_percent: Option<f64>,
}

impl<'invoice> Default for ApplicableTradeTax<'invoice> {
    fn default() -> Self {
        Self {
            calculated_amount: None,
            type_code: "VAT",
            basis_amount: None,
            category_code: VATCategoryCode::StandardRate,
            rate_applicable_percent: None,
            exemption_reason: None,
            exemption_reason_code: None,
        }
    }
}

/// `BG-22`: A group of business terms providing the monetary totals for the Invoice.
#[derive(Serialize, Clone, Debug, Default)]
pub struct SpecifiedTradeSettlementHeaderMonetarySummation {
    /// `BT-106`: Sum of all Invoice line net amounts in the Invoice.
    #[serde(
        rename = "ram:LineTotalAmount",
        serialize_with = "format_f64_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub line_total_amount: Option<f64>,
    /// `BT-108`: Sum of all charges on document level in the Invoice.
    #[serde(
        rename = "ram:ChargeTotalAmount",
        serialize_with = "format_f64_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub charge_total_amount: Option<f64>,
    /// `BT-107`: Sum of all allowances on document level in the Invoice.
    #[serde(
        rename = "ram:AllowanceTotalAmount",
        serialize_with = "format_f64_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub allowance_total_amount: Option<f64>,

    //Required for minimum specification
    /// `BT-109`: The total amount of the Invoice without VAT.
    #[serde(
        rename = "ram:TaxBasisTotalAmount",
        serialize_with = "format_f64_option"
    )]
    pub tax_basis_total_amount: Option<f64>,
    /// `BT-110`: The total VAT amount for the Invoice.
    #[serde(rename = "ram:TaxTotalAmount", skip_serializing_if = "Option::is_none")]
    pub tax_total_amount: Option<TaxTotalAmount>,
    /// `BT-112`: The total amount of the Invoice with VAT.
    #[serde(rename = "ram:GrandTotalAmount", serialize_with = "format_f64_option")]
    pub grand_total_amount: Option<f64>,
    /// `BT-115`: The outstanding amount that is requested to be paid.
    #[serde(rename = "ram:DuePayableAmount", serialize_with = "format_f64_option")]
    pub due_payable_amount: Option<f64>,
}

#[derive(Serialize, Clone, Debug)]
pub struct TaxTotalAmount {
    #[serde(rename = "@currencyID")]
    pub currency_id: CurrencyCode,
    #[serde(rename = "$value", serialize_with = "f64_format")]
    pub amount: f64,
}

impl TaxTotalAmount {
    pub fn new(currency_id: CurrencyCode, amount: f64) -> Self {
        TaxTotalAmount {
            currency_id,
            amount,
        }
    }
}

/// BT-7 and BT-8 are mutually exclusive.
#[derive(Serialize, Clone, Debug)]
pub enum TaxPoint<'a> {
    Date(NaiveDate),
    /// VAT due-date type code: 5, 29 or 72.
    DateCode(&'a str),
}

#[derive(Clone, Debug)]
pub enum TradeAllowanceChargeReasonCode {
    Allowance(AllowanceChargeReasonCode),
    Charge(ChargeCode),
}

/// An allowance or charge using the appropriate UNTDID reason-code list.
#[derive(Serialize, Clone, Debug)]
pub struct TradeAllowanceCharge<'a> {
    pub charge: bool,
    pub calculation_percent: Option<f64>,
    pub basis_amount: Option<f64>,
    pub actual_amount: f64,
    pub reason_code: Option<TradeAllowanceChargeReasonCode>,
    pub reason: Option<&'a str>,
    /// Required for document-level allowances and charges; absent at line level.
    pub category_trade_tax: Option<super::CategoryTradeTax<'a>>,
}

impl Serialize for TradeAllowanceChargeReasonCode {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Allowance(code) => code.serialize(serializer),
            Self::Charge(code) => code.serialize(serializer),
        }
    }
}

#[derive(Serialize)]
pub(super) struct ChargeView<'s, 'a> {
    #[serde(rename = "ram:ChargeIndicator")]
    indicator: ChargeIndicator,
    #[serde(
        rename = "ram:CalculationPercent",
        serialize_with = "format_f64_option",
        skip_serializing_if = "Option::is_none"
    )]
    calculation_percent: Option<f64>,
    #[serde(
        rename = "ram:BasisAmount",
        serialize_with = "format_f64_option",
        skip_serializing_if = "Option::is_none"
    )]
    basis_amount: Option<f64>,
    #[serde(rename = "ram:ActualAmount", serialize_with = "f64_format")]
    actual_amount: f64,
    #[serde(rename = "ram:ReasonCode", skip_serializing_if = "Option::is_none")]
    reason_code: Option<&'s TradeAllowanceChargeReasonCode>,
    #[serde(rename = "ram:Reason", skip_serializing_if = "Option::is_none")]
    reason: Option<&'a str>,
    #[serde(
        rename = "ram:CategoryTradeTax",
        skip_serializing_if = "Option::is_none"
    )]
    category_trade_tax: Option<CategoryTradeTax<'a>>,
}

impl<'s, 'a> From<&'s TradeAllowanceCharge<'a>> for ChargeView<'s, 'a> {
    fn from(value: &'s TradeAllowanceCharge<'a>) -> Self {
        Self {
            indicator: ChargeIndicator {
                indicator: value.charge,
            },
            calculation_percent: value.calculation_percent,
            basis_amount: value.basis_amount,
            actual_amount: value.actual_amount,
            reason_code: value.reason_code.as_ref(),
            reason: value.reason,
            category_trade_tax: value.category_trade_tax,
        }
    }
}

#[derive(Serialize)]
struct DateView<'a> {
    #[serde(rename = "udt:DateString")]
    value: DateTimeString<'a>,
}

#[derive(Serialize)]
struct TaxView<'a> {
    #[serde(
        rename = "ram:CalculatedAmount",
        serialize_with = "format_f64_option",
        skip_serializing_if = "Option::is_none"
    )]
    calculated_amount: Option<f64>,
    #[serde(rename = "ram:TypeCode")]
    type_code: &'a str,
    #[serde(
        rename = "ram:ExemptionReason",
        skip_serializing_if = "Option::is_none"
    )]
    exemption_reason: Option<&'a str>,
    #[serde(
        rename = "ram:BasisAmount",
        serialize_with = "format_f64_option",
        skip_serializing_if = "Option::is_none"
    )]
    basis_amount: Option<f64>,
    #[serde(rename = "ram:CategoryCode")]
    category_code: crate::components::enums::vat_category_code::VATCategoryCode,
    #[serde(
        rename = "ram:ExemptionReasonCode",
        skip_serializing_if = "Option::is_none"
    )]
    exemption_reason_code: Option<&'a str>,
    #[serde(rename = "ram:TaxPointDate", skip_serializing_if = "Option::is_none")]
    tax_point_date: Option<DateView<'a>>,
    #[serde(
        rename = "ram:DueDateTypeCode",
        skip_serializing_if = "Option::is_none"
    )]
    date_code: Option<&'a str>,
    #[serde(
        rename = "ram:RateApplicablePercent",
        serialize_with = "format_f64_option",
        skip_serializing_if = "Option::is_none"
    )]
    rate: Option<f64>,
}

impl<'a> TaxView<'a> {
    fn new(tax: &ApplicableTradeTax<'a>, point: Option<&TaxPoint<'a>>) -> Self {
        Self {
            calculated_amount: tax.calculated_amount,
            type_code: tax.type_code,
            exemption_reason: tax.exemption_reason,
            basis_amount: tax.basis_amount,
            category_code: tax.category_code,
            exemption_reason_code: tax.exemption_reason_code,
            tax_point_date: match point {
                Some(TaxPoint::Date(date)) => Some(DateView {
                    value: DateTimeString::new(*date),
                }),
                _ => None,
            },
            date_code: match point {
                Some(TaxPoint::DateCode(code)) => Some(code),
                _ => None,
            },
            rate: tax.rate_applicable_percent,
        }
    }
}

#[derive(Serialize)]
struct HeaderTotalsView<'s> {
    #[serde(
        rename = "ram:LineTotalAmount",
        serialize_with = "format_f64_option",
        skip_serializing_if = "Option::is_none"
    )]
    line_total: Option<f64>,
    #[serde(
        rename = "ram:ChargeTotalAmount",
        serialize_with = "format_f64_option",
        skip_serializing_if = "Option::is_none"
    )]
    charge_total: Option<f64>,
    #[serde(
        rename = "ram:AllowanceTotalAmount",
        serialize_with = "format_f64_option",
        skip_serializing_if = "Option::is_none"
    )]
    allowance_total: Option<f64>,
    #[serde(
        rename = "ram:TaxBasisTotalAmount",
        serialize_with = "format_f64_option"
    )]
    tax_basis: Option<f64>,
    #[serde(rename = "ram:TaxTotalAmount")]
    tax_totals: Vec<&'s TaxTotalAmount>,
    #[serde(rename = "ram:GrandTotalAmount", serialize_with = "format_f64_option")]
    grand_total: Option<f64>,
    #[serde(rename = "ram:DuePayableAmount", serialize_with = "format_f64_option")]
    due: Option<f64>,
}

#[derive(Serialize)]
pub(super) struct SettlementView<'s, 'a> {
    #[serde(
        rename = "ram:TaxCurrencyCode",
        skip_serializing_if = "Option::is_none"
    )]
    tax_currency: Option<&'s CurrencyCode>,
    #[serde(rename = "ram:InvoiceCurrencyCode")]
    invoice_currency: &'s CurrencyCode,
    #[serde(rename = "ram:SpecifiedTradeSettlementPaymentMeans")]
    payment_means: &'s Vec<SpecifiedTradeSettlementPaymentMeans<'a>>,
    #[serde(rename = "ram:ApplicableTradeTax")]
    taxes: Vec<TaxView<'a>>,
    #[serde(rename = "ram:SpecifiedTradeAllowanceCharge")]
    allowances: &'s Vec<SpecifiedTradeAllowanceCharge<'a>>,
    #[serde(rename = "ram:SpecifiedTradeAllowanceCharge")]
    extended_charges: Vec<ChargeView<'s, 'a>>,
    #[serde(
        rename = "ram:SpecifiedTradePaymentTerms",
        skip_serializing_if = "Option::is_none"
    )]
    terms: &'s Option<SpecifiedTradePaymentTerms<'a>>,
    #[serde(rename = "ram:SpecifiedTradeSettlementHeaderMonetarySummation")]
    totals: HeaderTotalsView<'s>,
}

impl<'s, 'a> SettlementView<'s, 'a> {
    pub(super) fn new(
        base: &'s ApplicableHeaderTradeSettlement<'a>,
        details: &'s AdditionalInvoiceDetails<'a>,
    ) -> Self {
        let totals = &base.specified_trade_settlement_header_monetary_summation;
        let mut tax_totals: Vec<_> = totals.tax_total_amount.iter().collect();
        tax_totals.extend(details.accounting_currency_tax.iter());
        Self {
            tax_currency: details
                .accounting_currency_tax
                .as_ref()
                .map(|tax| &tax.currency_id),
            invoice_currency: &base.invoice_currency_code,
            payment_means: &base.specified_trade_settlement_payment_means,
            taxes: base
                .applicable_trade_tax
                .iter()
                .map(|tax| TaxView::new(tax, details.tax_point.as_ref()))
                .collect(),
            allowances: &base.specified_trade_allowance_charge,
            extended_charges: details
                .document_charges
                .iter()
                .map(ChargeView::from)
                .collect(),
            terms: &base.specified_trade_payment_terms,
            totals: HeaderTotalsView {
                line_total: totals.line_total_amount,
                charge_total: totals.charge_total_amount,
                allowance_total: totals.allowance_total_amount,
                tax_basis: totals.tax_basis_total_amount,
                tax_totals,
                grand_total: totals.grand_total_amount,
                due: totals.due_payable_amount,
            },
        }
    }
}
