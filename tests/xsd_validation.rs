//! Validates generated XML against the official ZUGFeRD / Factur-X XSDs.
//!
//! The schemas are not shipped with this repository. Download the specification
//! package from <https://www.ferd-net.de> and unpack its XSDs (the per-profile
//! folders with all their imported `.xsd` files) below `tests/Schema/`, or point
//! `ZUGFERD_XSD_DIR` at the folder. Each profile's entry schema is found by its
//! file name suffix, e.g. `FACTUR-X_MINIMUM.xsd`.
//!
//! Without schemas the tests print a notice and pass. Set `ZUGFERD_REQUIRE_XSD=1`
//! (as CI should) to make missing schemas or a missing `xmllint` a failure.
//!
//! Validation shells out to `xmllint` (libxml2), which avoids a new dependency.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

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

fn require_xsd() -> bool {
    std::env::var_os("ZUGFERD_REQUIRE_XSD").is_some()
}

fn xsd_root() -> PathBuf {
    std::env::var_os("ZUGFERD_XSD_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/Schema"))
}

/// Finds the entry schema of a profile: a `.xsd` whose stem ends in `_<suffix>`.
fn find_schema(dir: &Path, suffix: &str) -> Option<PathBuf> {
    let wanted = format!("_{}.xsd", suffix.to_ascii_lowercase());
    for entry in std::fs::read_dir(dir).ok()?.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if let Some(found) = find_schema(&path, suffix) {
                return Some(found);
            }
        } else if path
            .file_name()
            .and_then(|n| n.to_str())
            .is_some_and(|n| n.to_ascii_lowercase().ends_with(&wanted))
        {
            return Some(path);
        }
    }
    None
}

/// Returns `false` (after a notice) when validation had to be skipped.
fn validate(xml: &str, schema_suffix: &str) -> bool {
    let root = xsd_root();
    let Some(schema) = find_schema(&root, schema_suffix) else {
        let msg = format!(
            "no *_{schema_suffix}.xsd found under {} (see tests/xsd_validation.rs)",
            root.display()
        );
        assert!(!require_xsd(), "{msg}");
        eprintln!("SKIPPED: {msg}");
        return false;
    };

    let spawned = Command::new("xmllint")
        .arg("--noout")
        .arg("--schema")
        .arg(&schema)
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();
    let mut child = match spawned {
        Ok(child) => child,
        Err(e) => {
            assert!(
                !require_xsd(),
                "xmllint is required but failed to start: {e}"
            );
            eprintln!("SKIPPED: xmllint not available: {e}");
            return false;
        }
    };
    child
        .stdin
        .take()
        .unwrap()
        .write_all(xml.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();

    assert!(
        output.status.success(),
        "XSD validation against {} failed:\n{}\n--- generated XML ---\n{}",
        schema.display(),
        String::from_utf8_lossy(&output.stderr),
        xml
    );
    true
}

fn line_item(
    line_id: &'static str,
    gtin: &'static str,
    name: &'static str,
    net_price: f64,
    quantity: f64,
) -> IncludedSupplyChainTradeLineItem<'static> {
    let basis = net_price * quantity;
    let tax = basis * 0.19;
    IncludedSupplyChainTradeLineItem {
        associated_document_line_document: AssociatedDocumentLineDocument {
            line_id,
            included_note: None,
        },
        specified_trade_product: SpecifiedTradeProduct {
            global_id: Some(GlobalID {
                value: gtin,
                scheme_id: IdentifierSchemeCode::GTIN,
            }),
            name,
            description: None,
        },
        specified_line_trade_agreement: SpecifiedLineTradeAgreement {
            gross_price_product_trade_price: None,
            net_price_product_trade_price: NetPriceProductTradePrice {
                charge_amount: net_price,
            },
        },
        specified_line_trade_delivery: SpecifiedLineTradeDelivery {
            billed_quantity: BilledQuantity {
                value: quantity,
                unit_code: UnitCode::Piece,
            },
        },
        specified_line_trade_settlement: SpecifiedLineTradeSettlement {
            applicable_trade_tax: ApplicableTradeTax {
                calculated_amount: Some(tax),
                type_code: "VAT",
                category_code: VATCategoryCode::StandardRate,
                basis_amount: Some(basis),
                rate_applicable_percent: Some(19.0),
                exemption_reason: None,
                exemption_reason_code: None,
            },
            specified_trade_settlement_line_monetary_summation:
                SpecifiedTradeSettlementLineMonetarySummation {
                    line_total_amount: basis,
                },
            specified_trade_allowance_charge: Vec::new(),
        },
    }
}

/// Builds an invoice carrying only the data its profile asks for.
fn builder_for(level: SpecificationLevel) -> InvoiceBuilder<'static> {
    let net = 100.0;
    let tax = net * 0.19;
    let gross = net + tax;

    let mut b = InvoiceBuilder::new();
    b.set_business_process("process1")
        .set_invoice_type_code(InvoiceTypeCode::CommercialInvoice)
        .set_invoice_nr("INV-123456")
        .set_date_of_issue(date(2024, 8, 10))
        .set_buyer_reference("BR-7890")
        .set_sellers_name("Seller Corp.")
        .set_sellers_specified_legal_organization("LegalOrg-001")
        .set_sellers_postal_trade_address_country_code(CountryCode::Germany)
        .set_sellers_specified_tax_registration("DE123456789")
        .set_buyers_name("Buyer Inc.")
        .set_buyers_specified_legal_organization("LegalOrg-002")
        .set_buyers_order_specified_document("OD-2024-001")
        .set_invoice_currency_code(CurrencyCode::Euro)
        .set_monetary_summation_tax_basis_total_amount(net)
        .set_monetary_summation_tax_total_amount(tax)
        .set_monetary_summation_grand_total_amount(gross)
        .set_monetary_summation_due_payable_amount(gross - 50.0);

    if level >= SpecificationLevel::BasicWithoutLines {
        b.set_invoice_notes(vec!["Note 1"])
            .set_sellers_postal_trade_address_line_one("Best street")
            .set_sellers_postal_trade_address_postcode_code("66666")
            .set_sellers_postal_trade_address_city_name("Hometown")
            .set_buyers_postal_trade_address_line_one("Main street")
            .set_buyers_postal_trade_address_postcode_code("77777")
            .set_buyers_postal_trade_address_city_name("Springfield")
            .set_occurrence_date(date(2024, 7, 6))
            .add_applicable_trade_tax(ApplicableTradeTax {
                basis_amount: Some(net),
                calculated_amount: Some(tax),
                rate_applicable_percent: Some(19.0),
                ..Default::default()
            })
            .set_specified_trade_payment_terms_due_date(date(2025, 4, 12))
            .set_monetary_summation_line_total_amount(net)
            .set_monetary_summation_charge_total_amount(0.0)
            .set_monetary_summation_allowance_total_amount(0.0);
    }
    if level >= SpecificationLevel::Basic {
        b.add_supply_chain_trade_line_item(line_item(
            "1",
            "1234567890123",
            "Product 1",
            100.0,
            1.0,
        ));
    }
    b
}

fn check(level: SpecificationLevel, schema_suffix: &str) {
    let xml = builder_for(level)
        .to_xml_string(level)
        .expect("invoice should build");
    validate(&xml, schema_suffix);
}

#[test]
fn minimum_validates_against_xsd() {
    check(SpecificationLevel::Minimum, "MINIMUM");
}

#[test]
fn basic_wl_validates_against_xsd() {
    check(SpecificationLevel::BasicWithoutLines, "BASIC-WL");
}

#[test]
fn basic_validates_against_xsd() {
    check(SpecificationLevel::Basic, "BASIC");
}

#[test]
fn basic_wl_with_two_vat_categories_validates_against_xsd() {
    let level = SpecificationLevel::BasicWithoutLines;
    let mut b = builder_for(level);
    b.add_applicable_trade_tax(ApplicableTradeTax {
        basis_amount: Some(44.0),
        calculated_amount: Some(0.0),
        rate_applicable_percent: Some(0.0),
        category_code: zugferd::VATCategoryCode::ExemptFromTax,
        exemption_reason: Some("Tax-exempt service"),
        ..Default::default()
    });
    let xml = b.to_xml_string(level).expect("invoice should build");
    assert_eq!(xml.matches("<ram:ApplicableTradeTax>").count(), 2);
    validate(&xml, "BASIC-WL");
}

#[test]
fn minimum_omits_applicable_trade_tax() {
    let level = SpecificationLevel::Minimum;
    let xml = builder_for(level).to_xml_string(level).unwrap();
    assert!(!xml.contains("ApplicableTradeTax"));
}

#[test]
fn basic_wl_without_applicable_trade_tax_is_rejected() {
    let b = InvoiceBuilder::new();
    let err = b
        .all_fields_are_set(SpecificationLevel::BasicWithoutLines)
        .unwrap_err()
        .to_string();
    assert!(err.contains("Applicable trade tax not set"));
}
