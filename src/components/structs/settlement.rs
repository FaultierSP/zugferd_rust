use super::context_document::DateTimeString;
use super::line_item::ChargeIndicator;
use super::serde_helpers::f64_format;
use super::serde_helpers::format_f64_option;
use crate::components::enums::{
    allowance_charge_reason_code::AllowanceChargeReasonCode, currency_code::CurrencyCode,
    payment_means_code::PaymentMeansCode, vat_category_code::VATCategoryCode,
};
use serde::Serialize;

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

#[derive(Serialize, Clone, Copy, Debug)]
pub struct SpecifiedTradeSettlementPaymentMeans<'invoice> {
    #[serde(rename = "ram:TypeCode")]
    pub type_code: PaymentMeansCode<'invoice>,
    #[serde(rename = "ram:Information", skip_serializing_if = "Option::is_none")]
    pub information: Option<&'invoice str>,
    #[serde(
        rename = "ram:ApplicableTradeSettlementFinancialCard",
        skip_serializing_if = "Option::is_none"
    )]
    pub applicable_trade_settlement_financial_card: Option<&'invoice str>,
    #[serde(
        rename = "ram:PayerPartyDebtorFinancialAccount",
        skip_serializing_if = "Option::is_none"
    )]
    pub payer_party_debtor_financial_account: Option<PayerPartyDebtorFinancialAccount<'invoice>>,
    #[serde(
        rename = "ram:PayeePartyCreditorFinancialAccount",
        skip_serializing_if = "Option::is_none"
    )]
    pub payee_party_creditor_financial_account:
        Option<PayeePartyCreditorFinancialAccount<'invoice>>,
    #[serde(
        rename = "ram:PayeeSpecifiedCreditorFinancialInstitution",
        skip_serializing_if = "Option::is_none"
    )]
    pub payee_specified_creditor_financial_institution:
        Option<PayeeSpecifiedCreditorFinancialInstitution<'invoice>>,
}

#[derive(Serialize, Clone, Copy, Debug)]
pub struct ApplicableTradeSettlementFinancialCard<'invoice> {
    pub id: &'invoice str, // TODO: should this be an enum?
    pub cardholder_name: &'invoice str,
}

#[derive(Serialize, Clone, Copy, Debug)]
pub struct PayerPartyDebtorFinancialAccount<'invoice> {
    #[serde(rename = "ram:IBANID")]
    pub ibanid: &'invoice str,
}

#[derive(Serialize, Clone, Copy, Debug)]
pub struct PayeePartyCreditorFinancialAccount<'invoice> {
    #[serde(rename = "ram:IBANID", skip_serializing_if = "Option::is_none")]
    pub ibanid: Option<&'invoice str>, // TODO: should this be a custom type? Or the crate `iban`?
    #[serde(rename = "ram:AccountName", skip_serializing_if = "Option::is_none")]
    pub account_name: Option<&'invoice str>, // TODO: should this be a custom type? Or the crate `iban`?
    #[serde(rename = "ram:ProprietaryID", skip_serializing_if = "Option::is_none")]
    pub proprietary_id: Option<&'invoice str>,
}

#[derive(Serialize, Clone, Copy, Debug)]
pub struct PayeeSpecifiedCreditorFinancialInstitution<'invoice> {
    #[serde(rename = "ram:BICID")]
    pub bicid: &'invoice str,
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

#[derive(Serialize, Clone, Debug)]
pub struct SpecifiedTradePaymentTerms<'invoice> {
    /// `BT-20`: A textual description of the payment terms that apply to the amount due for payment (Including description of possible penalties).
    #[serde(rename = "ram:Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<&'invoice str>,
    /// `BT-9-00`: The date when the payment is due.
    #[serde(
        rename = "ram:DueDateDateTime",
        skip_serializing_if = "Option::is_none"
    )]
    pub due_date_time: Option<DueDateDateTime<'invoice>>,
}

#[derive(Serialize, Clone, Debug)]
pub struct DueDateDateTime<'invoice> {
    /// `BT-9`: The date when the payment is due.
    #[serde(rename = "udt:DateTimeString")]
    pub payment_due_date: DateTimeString<'invoice>,
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
