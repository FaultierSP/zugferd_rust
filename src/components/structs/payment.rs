use super::context_document::DateTimeString;
use crate::components::enums::payment_means_code::PaymentMeansCode;
use serde::Serialize;

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
    pub applicable_trade_settlement_financial_card:
        Option<ApplicableTradeSettlementFinancialCard<'invoice>>,
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
    #[serde(rename = "ram:ID")]
    pub id: &'invoice str, // TODO: should this be an enum?
    #[serde(rename = "ram:CardholderName", skip_serializing_if = "Option::is_none")]
    pub cardholder_name: Option<&'invoice str>,
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

/// Composite struct for all payee financial account information
#[derive(Serialize, Clone, Copy, Debug)]
pub struct PayeePartyCreditorFinancialAccountInfo<'invoice> {
    pub account: PayeePartyCreditorFinancialAccount<'invoice>,
    pub institution: Option<PayeeSpecifiedCreditorFinancialInstitution<'invoice>>,
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
    /// `BT-89`: The direct debit mandate reference.
    #[serde(
        rename = "ram:DirectDebitMandateID",
        skip_serializing_if = "Option::is_none"
    )]
    pub direct_debit_mandate_id: Option<&'invoice str>,
}

#[derive(Serialize, Clone, Debug)]
pub struct DueDateDateTime<'invoice> {
    /// `BT-9`: The date when the payment is due.
    #[serde(rename = "udt:DateTimeString")]
    pub payment_due_date: DateTimeString<'invoice>,
}
