//! Empty optional values must not show up in the generated XML.

use zugferd::components::enums::identifier_scheme_code::IdentifierSchemeCode;
use zugferd::components::enums::unit_code::UnitCode;
use zugferd::{
    ApplicableTradeTax, AssociatedDocumentLineDocument, BilledQuantity, CountryCode, CurrencyCode,
    GlobalID, IncludedSupplyChainTradeLineItem, InvoiceBuilder, InvoiceTypeCode,
    NetPriceProductTradePrice, SpecificationLevel, SpecifiedLineTradeAgreement,
    SpecifiedLineTradeDelivery, SpecifiedLineTradeSettlement, SpecifiedTradeProduct,
    SpecifiedTradeSettlementLineMonetarySummation, VATCategoryCode,
};

fn date(y: i32, m: u32, d: u32) -> chrono::NaiveDate {
    chrono::NaiveDate::from_ymd_opt(y, m, d).unwrap()
}

fn minimum_builder<'a>(buyer_ref: &'a str, legal_org: &'a str) -> InvoiceBuilder<'a> {
    let mut b = InvoiceBuilder::new();
    b.set_invoice_type_code(InvoiceTypeCode::CommercialInvoice)
        .set_invoice_nr("INV-1")
        .set_date_of_issue(date(2024, 8, 10))
        .set_buyer_reference(buyer_ref)
        .set_sellers_name("Seller Corp.")
        .set_sellers_specified_legal_organization(legal_org)
        .set_sellers_postal_trade_address_country_code(CountryCode::Germany)
        .set_sellers_specified_tax_registration("DE123456789")
        .set_buyers_name("Buyer Inc.")
        .set_buyers_specified_legal_organization(legal_org)
        .set_buyers_order_specified_document("OD-1")
        .set_invoice_currency_code(CurrencyCode::Euro)
        .set_monetary_summation_tax_basis_total_amount(100.0)
        .set_monetary_summation_tax_total_amount(19.0)
        .set_monetary_summation_grand_total_amount(119.0)
        .set_monetary_summation_due_payable_amount(119.0);
    b
}

#[test]
fn empty_strings_are_omitted() {
    let xml = minimum_builder("", "  ")
        .to_xml_string(SpecificationLevel::Minimum)
        .expect("invoice should build");
    assert!(!xml.contains("BuyerReference"), "{xml}");
    assert!(!xml.contains("SpecifiedLegalOrganization"), "{xml}");
    assert!(!xml.contains("IncludedNote"), "{xml}");
    assert!(!xml.contains("<ram:LineOne"), "{xml}");
}

#[test]
fn non_empty_strings_are_kept() {
    let xml = minimum_builder("BR-7890", "LegalOrg-001")
        .to_xml_string(SpecificationLevel::Minimum)
        .expect("invoice should build");
    assert!(
        xml.contains("<ram:BuyerReference>BR-7890</ram:BuyerReference>"),
        "{xml}"
    );
    assert!(xml.contains("LegalOrg-001"), "{xml}");
}

/// Full invoice as required from BASIC WL upwards, with empty optional values mixed in.
fn full_builder<'a>() -> InvoiceBuilder<'a> {
    let mut b = minimum_builder("", "");
    b.set_sellers_postal_trade_address_line_one("Best street")
        .set_sellers_postal_trade_address_line_two("")
        .set_sellers_postal_trade_address_line_three("  ")
        .set_sellers_postal_trade_address_postcode_code("66666")
        .set_sellers_postal_trade_address_city_name("Hometown")
        .set_buyers_postal_trade_address_line_one("Main street")
        .set_buyers_postal_trade_address_line_two("")
        .set_buyers_postal_trade_address_postcode_code("77777")
        .set_buyers_postal_trade_address_city_name("Springfield")
        .set_occurrence_date(date(2024, 7, 6))
        .set_applicable_trade_tax_basis_amount(100.0)
        .set_applicable_trade_tax_calculated_amount(19.0)
        .set_applicable_trade_tax_rate_applicable_percent(19.0)
        .set_specified_trade_payment_terms_due_date(date(2025, 4, 12))
        .set_monetary_summation_line_total_amount(100.0)
        .set_monetary_summation_charge_total_amount(0.0)
        .set_monetary_summation_allowance_total_amount(0.0)
        .add_supply_chain_trade_line_item(line_item());
    b
}

fn line_item() -> IncludedSupplyChainTradeLineItem<'static> {
    IncludedSupplyChainTradeLineItem {
        associated_document_line_document: AssociatedDocumentLineDocument {
            line_id: "1",
            included_note: None,
        },
        specified_trade_product: SpecifiedTradeProduct {
            global_id: Some(GlobalID {
                value: "1234567890123",
                scheme_id: IdentifierSchemeCode::GTIN,
            }),
            name: "Product 1",
            description: None,
        },
        specified_line_trade_agreement: SpecifiedLineTradeAgreement {
            gross_price_product_trade_price: None,
            net_price_product_trade_price: NetPriceProductTradePrice {
                charge_amount: 100.0,
            },
        },
        specified_line_trade_delivery: SpecifiedLineTradeDelivery {
            billed_quantity: BilledQuantity {
                value: 1.0,
                unit_code: UnitCode::Piece,
            },
        },
        specified_line_trade_settlement: SpecifiedLineTradeSettlement {
            applicable_trade_tax: ApplicableTradeTax {
                calculated_amount: Some(19.0),
                type_code: "VAT",
                category_code: VATCategoryCode::StandardRate,
                basis_amount: Some(100.0),
                rate_applicable_percent: Some(19.0),
                exemption_reason: None,
                exemption_reason_code: None,
            },
            specified_trade_settlement_line_monetary_summation:
                SpecifiedTradeSettlementLineMonetarySummation {
                    line_total_amount: 100.0,
                },
            specified_trade_allowance_charge: Vec::new(),
        },
    }
}

fn assert_empty_fields_omitted(level: SpecificationLevel) {
    let xml = full_builder()
        .to_xml_string(level)
        .expect("invoice should build");
    assert!(!xml.contains("BuyerReference"), "{xml}");
    assert!(!xml.contains("SpecifiedLegalOrganization"), "{xml}");
    assert!(!xml.contains("IncludedNote"), "{xml}");
    assert!(!xml.contains("<ram:LineTwo"), "{xml}");
    assert!(!xml.contains("<ram:LineThree"), "{xml}");
    assert!(
        !xml.contains("<ram:LineTwo/>") && !xml.contains("<ram:LineThree/>"),
        "{xml}"
    );
    for kept in ["Best street", "Main street", "66666", "Springfield"] {
        assert!(xml.contains(kept), "missing {kept}: {xml}");
    }
}

#[test]
fn basic_omits_empty_fields() {
    assert_empty_fields_omitted(SpecificationLevel::Basic);
}

// Enable once the EN 16931 profile is supported (it needs more mandatory fields).
#[test]
#[ignore = "EN 16931 profile not implemented yet"]
fn en16931_omits_empty_fields() {
    assert_empty_fields_omitted(SpecificationLevel::En16931);
}
