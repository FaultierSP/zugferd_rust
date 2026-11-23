use std::fmt;

use serde::Serialize;

use crate::components::enums::{currency_code::CurrencyCode, invoice_type_code::InvoiceTypeCode};
use crate::components::structs::*;

mod assembly;
mod document;
mod line_items;
mod parties;
mod settlement;
mod tax;
mod validation;

// Error Handling

#[derive(Debug)]
pub enum InvoiceBuilderError {
    MissingField(String),
}

impl fmt::Display for InvoiceBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InvoiceBuilderError::MissingField(field) => write!(f, "Missing field: {}", field),
        }
    }
}

impl std::error::Error for InvoiceBuilderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            InvoiceBuilderError::MissingField(_) => None,
        }
    }
}

#[derive(Serialize, Clone)]
pub struct InvoiceBuilder<'invoice_builder> {
    //Minimum specification
    // BT-23
    business_process: Option<&'invoice_builder str>,
    // BT-24
    invoice_type_code: Option<InvoiceTypeCode>,
    // BT-1
    invoice_nr: Option<&'invoice_builder str>,
    // BT-2
    date_of_issue: Option<DateTimeString<'invoice_builder>>,
    // BT-
    document_notes: Option<Vec<IncludedNote>>,
    // BT-10
    buyer_reference: Option<&'invoice_builder str>,
    // BT-27
    sellers_name: Option<&'invoice_builder str>,
    // BT-30
    sellers_specified_legal_organization: Option<&'invoice_builder str>,
    // BT-31
    sellers_specified_tax_registration: Option<&'invoice_builder str>,
    // BT-40 --- Laendercode des Verkäufers - nicht explizit
    //buyer information
    // BT-44
    buyers_name: Option<&'invoice_builder str>,
    // BT-47
    buyers_specified_legal_organization: Option<&'invoice_builder str>,
    buyers_order_specified_document: Option<&'invoice_builder str>,
    occurrence_date: Option<DateTimeString<'invoice_builder>>,
    invoice_currency_code: Option<CurrencyCode>,
    // BT-110
    tax_total_amount: Option<f64>,
    // BT-115
    monetary_summation: SpecifiedTradeSettlementHeaderMonetarySummation,

    //Basic WL specification
    
    applicable_trade_tax: Vec<ApplicableTradeTax<'invoice_builder>>,
    sellers_postal_trade_address: PostalTradeAddress<'invoice_builder>,
    buyers_postal_trade_address: PostalTradeAddress<'invoice_builder>,
    monetary_summation_line_total_amount: Option<f64>,
    monetary_summation_charge_total_amount: Option<f64>,
    monetary_summation_allowance_total_amount: Option<f64>,
    specified_trade_settlement_payment_means: Vec<SpecifiedTradeSettlementPaymentMeans<'invoice_builder>>,
    specified_trade_settlement_payment_means_type_code: Option<&'invoice_builder str>,
    payer_party_debtor_financial_account_ibanid: Option<&'invoice_builder str>,
    payer_payee_creditor_financial_account_ibanid: Option<&'invoice_builder str>,
    payee_party_creditor_financial_account_proprietary_id: Option<&'invoice_builder str>,
    specified_trade_payment_terms: Option<SpecifiedTradePaymentTerms<'invoice_builder>>,

    //Basic specification
    included_supply_chain_trade_line_items: Vec<IncludedSupplyChainTradeLineItem<'invoice_builder>>,

    //EN 16931 specification
    //Payment information according to EN 16931 specification
    specified_trade_settlement_payment_means_information: Option<&'invoice_builder str>,
    applicable_settlement_financial_card_id: Option<&'invoice_builder str>,
    applicable_settlement_financial_card_cardholder_name: Option<&'invoice_builder str>,
    payee_party_creditor_financial_account_name: Option<&'invoice_builder str>,
    payee_specified_creditor_financial_institution_bicid: Option<&'invoice_builder str>,
    // parties according to EN 16931 specification
    // BT-45
    buyer_trade_party_trading_business_name: Option<&'invoice_builder str>,
    buyers_defined_trade_contact_person_name: Option<&'invoice_builder str>,
    buyers_defined_trade_contact_department_name: Option<&'invoice_builder str>,
    buyers_defined_trade_contact_complete_number: Option<&'invoice_builder str>,

}

impl<'invoice_builder> Default for InvoiceBuilder<'invoice_builder> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'invoice_builder> InvoiceBuilder<'invoice_builder> {
    pub fn new() -> Self {
        Self {
            // ??????
            specified_trade_settlement_payment_means: Vec::new(),
            business_process: None,
            invoice_type_code: None,
            invoice_nr: None,
            date_of_issue: None,
            document_notes: None,
            buyer_reference: None,
            sellers_name: None,
            sellers_specified_legal_organization: None,
            sellers_specified_tax_registration: None,
            buyers_name: None,
            buyers_specified_legal_organization: None,
            buyers_order_specified_document: None,
            occurrence_date: None,
            invoice_currency_code: None,
            tax_total_amount: None,
            monetary_summation: SpecifiedTradeSettlementHeaderMonetarySummation::default(),

            applicable_trade_tax: Vec::new(),
            sellers_postal_trade_address: PostalTradeAddress::default(),
            buyers_postal_trade_address: PostalTradeAddress::default(),
            monetary_summation_line_total_amount: None,
            monetary_summation_charge_total_amount: None,
            monetary_summation_allowance_total_amount: None,
            specified_trade_settlement_payment_means_type_code: None,
            specified_trade_settlement_payment_means_information: None,
            applicable_settlement_financial_card_id: None,
            applicable_settlement_financial_card_cardholder_name: None,
            payer_party_debtor_financial_account_ibanid: None,
            payer_payee_creditor_financial_account_ibanid: None,
            payee_party_creditor_financial_account_name: None,
            payee_party_creditor_financial_account_proprietary_id: None,
            payee_specified_creditor_financial_institution_bicid: None,
            specified_trade_payment_terms: None,

            included_supply_chain_trade_line_items: Vec::new(),
            buyer_trade_party_trading_business_name: None,
            buyers_defined_trade_contact_person_name: None,
            buyers_defined_trade_contact_department_name: None,
            buyers_defined_trade_contact_complete_number: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::components::enums::country_code::CountryCode;

    #[test]
    /// Tests if a minimum invoice can be built without specifying the legal organization
    fn test_all_fields_are_set_legal_organization() {
        let specification_level =
            crate::components::enums::specification_level::SpecificationLevel::Minimum;

        let sum_net: f64 = 100.0;
        let tax: f64 = sum_net * 19.0 / 100.0;
        let sum_gross: f64 = sum_net + tax;
        let customer_paid_already: f64 = 50.0;

        let mut invoice_builder = InvoiceBuilder::new();

        invoice_builder
            .set_business_process("process1")
            .set_invoice_type_code(InvoiceTypeCode::CommercialInvoice)
            .set_invoice_nr("INV-123456")
            .set_date_of_issue(chrono::NaiveDate::from_ymd_opt(2024, 8, 10).unwrap())
            .set_buyer_reference("BR-7890")
            .set_sellers_name("Seller Corp.")
            // .set_sellers_specified_legal_organization("LegalOrg-001")
            .set_sellers_postal_trade_address_country_code(CountryCode::Germany)
            .set_sellers_specified_tax_registration("DE123456789")
            .set_buyers_name("Buyer Inc.")
            // .set_buyers_specified_legal_organization("LegalOrg-002")
            .set_buyers_order_specified_document("")
            .set_invoice_currency_code(CurrencyCode::Euro)
            .set_monetary_summation_tax_basis_total_amount(sum_net)
            .set_monetary_summation_tax_total_amount(tax)
            .set_monetary_summation_grand_total_amount(sum_gross)
            .set_monetary_summation_due_payable_amount(sum_gross - customer_paid_already);

        assert!(invoice_builder
            .all_fields_are_set(specification_level)
            .is_ok());
        assert!(invoice_builder.build(specification_level).is_ok());
    }

    #[test]
    fn test_payment_account_details_are_emitted() {
        let specification_level =
            crate::components::enums::specification_level::SpecificationLevel::Minimum;

        let sum_net = 100.0;
        let tax = sum_net * 19.0 / 100.0;
        let sum_gross = sum_net + tax;

        let mut invoice_builder = InvoiceBuilder::new();
        invoice_builder
            .set_business_process("process1")
            .set_invoice_type_code(InvoiceTypeCode::CommercialInvoice)
            .set_invoice_nr("INV-123456")
            .set_date_of_issue(chrono::NaiveDate::from_ymd_opt(2024, 8, 10).unwrap())
            .set_buyer_reference("BR-7890")
            .set_sellers_name("Seller Corp.")
            .set_sellers_postal_trade_address_country_code(CountryCode::Germany)
            .set_sellers_specified_tax_registration("DE123456789")
            .set_buyers_name("Buyer Inc.")
            .set_buyers_order_specified_document("")
            .set_invoice_currency_code(CurrencyCode::Euro)
            .set_monetary_summation_tax_basis_total_amount(sum_net)
            .set_monetary_summation_tax_total_amount(tax)
            .set_monetary_summation_grand_total_amount(sum_gross)
            .set_monetary_summation_due_payable_amount(sum_gross)
            .set_payer_party_debtor_financial_account_ibanid("DE89370400440532013000")
            .set_payer_payee_creditor_financial_account_ibanid("DE02120300000000202051")
            .set_payee_party_creditor_financial_account_name("Someone's Account")
            .set_payee_party_creditor_financial_account_proprietary_id("ACC-123")
            .set_payee_party_creditor_financial_account_payee_specified_creditor_financial_institution_bicid(
                "COBADEFFXXX",
            );
        let xml = invoice_builder.to_xml_string(specification_level).unwrap();

        assert!(xml.contains("<ram:TypeCode>30</ram:TypeCode>"));
        assert!(xml.contains("<ram:IBANID>DE89370400440532013000</ram:IBANID>"));
        assert!(xml.contains("<ram:IBANID>DE02120300000000202051</ram:IBANID>"));
        assert!(xml.contains("<ram:AccountName>Someone's Account</ram:AccountName>"));
        assert!(xml.contains("<ram:ProprietaryID>ACC-123</ram:ProprietaryID>"));
        assert!(xml.contains("<ram:BICID>COBADEFFXXX</ram:BICID>"));
    }

    #[test]
    fn test_payment_means_fields_are_emitted_without_account_details() {
        let specification_level =
            crate::components::enums::specification_level::SpecificationLevel::Minimum;

        let mut invoice_builder = InvoiceBuilder::new();
        invoice_builder
            .set_business_process("process1")
            .set_invoice_type_code(InvoiceTypeCode::CommercialInvoice)
            .set_invoice_nr("INV-123456")
            .set_date_of_issue(chrono::NaiveDate::from_ymd_opt(2024, 8, 10).unwrap())
            .set_buyer_reference("BR-7890")
            .set_sellers_name("Seller Corp.")
            .set_sellers_postal_trade_address_country_code(CountryCode::Germany)
            .set_sellers_specified_tax_registration("DE123456789")
            .set_buyers_name("Buyer Inc.")
            .set_buyers_order_specified_document("")
            .set_invoice_currency_code(CurrencyCode::Euro)
            .set_monetary_summation_tax_basis_total_amount(100.0)
            .set_monetary_summation_tax_total_amount(19.0)
            .set_monetary_summation_grand_total_amount(119.0)
            .set_monetary_summation_due_payable_amount(119.0)
            .set_specified_trade_settlement_payment_means_type_code("58")
            .set_specified_trade_settlement_payment_means_information("SEPA transfer")
            .set_applicable_settlement_financial_card_id("CARD-123")
            .set_applicable_settlement_financial_card_cardholder_name("Invoice Buyer");

        let xml = invoice_builder.to_xml_string(specification_level).unwrap();
        assert!(xml.contains("<ram:TypeCode>58</ram:TypeCode>"));
        assert!(xml.contains("<ram:Information>SEPA transfer</ram:Information>"));
        assert!(xml.contains("<ram:ID>CARD-123</ram:ID>"));
        assert!(xml.contains("<ram:CardholderName>Invoice Buyer</ram:CardholderName>"));
    }
}
