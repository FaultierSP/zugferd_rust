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

use zugferd::components::enums::allowance_charge_reason_code::AllowanceChargeReasonCode;
use zugferd::components::enums::charge_codes_untdid7161::ChargeCode;
use zugferd::components::enums::identifier_scheme_code::IdentifierSchemeCode;
use zugferd::components::enums::unit_code::UnitCode;
use zugferd::{
    ApplicableTradeTax, AssociatedDocumentLineDocument, BilledQuantity, CategoryTradeTax,
    ChargeIndicator, CountryCode, CurrencyCode, GlobalID, IncludedSupplyChainTradeLineItem,
    InvoiceBuilder, InvoiceTypeCode, NetPriceProductTradePrice, SpecificationLevel,
    SpecifiedLineTradeAgreement, SpecifiedLineTradeAllowanceCharge, SpecifiedLineTradeDelivery,
    SpecifiedLineTradeSettlement, SpecifiedTradeAllowanceCharge, SpecifiedTradeProduct,
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

fn assert_xml_rejected(xml: &str) {
    let spawned = Command::new("xmllint")
        .arg("--noout")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn();
    let mut child = match spawned {
        Ok(child) => child,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            assert!(!require_xsd(), "xmllint is required but not available: {e}");
            eprintln!("SKIPPED: xmllint not available: {e}");
            return;
        }
        Err(e) => panic!("failed to start xmllint: {e}"),
    };
    child
        .stdin
        .take()
        .unwrap()
        .write_all(xml.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();

    assert!(
        !output.status.success(),
        "xmllint accepted malformed XML:\n{xml}"
    );
}

#[test]
fn rejects_unclosed_opening_element() {
    assert_xml_rejected("<root>");
}

#[test]
fn rejects_unmatched_closing_element() {
    assert_xml_rejected("</root>");
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
            .set_payer_financial_account_ibanid("DE89370400440532013000")
            .set_payee_account_ibanid("DE02120300000000202051")
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
    check(SpecificationLevel::BasicWithoutLines, "BASICWL");
}

#[test]
fn basic_validates_against_xsd() {
    check(SpecificationLevel::Basic, "BASIC");
}

#[test]
fn en16931_allowances_and_charges_validate_against_xsd() {
    let level = SpecificationLevel::En16931;
    let mut builder = builder_for(level);
    for (indicator, reason, reason_code) in [
        (false, "Discount", Some(AllowanceChargeReasonCode::Discount)),
        (true, "Surcharge", None),
    ] {
        builder.add_specified_trade_allowance_charge(SpecifiedTradeAllowanceCharge {
            charge_indicator: ChargeIndicator { indicator },
            calculation_percent: Some(5.0),
            basis_amount: Some(100.0),
            actual_amount: 5.0,
            reason_code,
            reason: Some(reason),
            category_trade_tax: CategoryTradeTax {
                type_code: "VAT",
                category_code: VATCategoryCode::StandardRate,
                rate_applicable_percent: Some(19.0),
            },
        });
    }

    let mut line = line_item("2", "1234567890123", "Product 2", 100.0, 1.0);
    line.specified_line_trade_settlement
        .specified_trade_allowance_charge
        .push(SpecifiedLineTradeAllowanceCharge {
            charge_indicator: ChargeIndicator { indicator: false },
            calculation_percent: Some(5.0),
            basis_amount: Some(100.0),
            actual_amount: 5.0,
            reason_code: Some(AllowanceChargeReasonCode::Discount),
            reason: Some("Discount"),
        });
    builder.add_supply_chain_trade_line_item(line);

    let first = builder
        .build(level)
        .expect("invoice should build")
        .to_xml_string()
        .unwrap();
    let second = builder
        .build(level)
        .expect("invoice should build")
        .to_xml_string()
        .unwrap();
    assert_eq!(first, second);
    assert_eq!(
        first.matches("<ram:SpecifiedTradeAllowanceCharge>").count(),
        3
    );
    let mut reason_codes = 0;
    for allowance in first.split("<ram:SpecifiedTradeAllowanceCharge>").skip(1) {
        let allowance = allowance
            .split("</ram:SpecifiedTradeAllowanceCharge>")
            .next()
            .unwrap();
        assert!(allowance.contains("<ram:CalculationPercent>5.00</ram:CalculationPercent>"));
        assert!(allowance.contains("<ram:BasisAmount>100.00</ram:BasisAmount>"));
        if allowance.contains("<ram:ReasonCode>95</ram:ReasonCode>") {
            reason_codes += 1;
        }
    }
    assert_eq!(reason_codes, 2);
    validate(&first, "EN16931");
}

#[test]
fn minimum_rejects_document_allowances() {
    let level = SpecificationLevel::Minimum;
    let mut builder = builder_for(level);
    builder.add_specified_trade_allowance_charge(SpecifiedTradeAllowanceCharge {
        charge_indicator: ChargeIndicator { indicator: false },
        calculation_percent: None,
        basis_amount: None,
        actual_amount: 5.0,
        reason_code: None,
        reason: Some("Discount"),
        category_trade_tax: CategoryTradeTax {
            type_code: "VAT",
            category_code: VATCategoryCode::StandardRate,
            rate_applicable_percent: Some(19.0),
        },
    });
    let error = builder.build(level).unwrap_err();
    assert!(error
        .contains("Document-level allowances and charges require Basic Without Lines or higher"));
}

#[test]
fn extended_en16931_fields_are_emitted_and_schema_valid() {
    use zugferd::{
        AdditionalInvoiceDetails, DocumentAttachment, InvoiceLineDetails, ObjectReference,
        PostalAddressDetails, PostalTradeAddress, ProductCharacteristic, ProductClassification,
        ProjectReference, ShipTo, SupportingDocument, TaxPoint, TaxRepresentative, TaxTotalAmount,
        TradeAllowanceCharge, TradeAllowanceChargeReasonCode, TradeContact,
    };

    let address = || PostalAddressDetails {
        address: PostalTradeAddress {
            postcode_code: Some("12345"),
            line_one: Some("First"),
            line_two: Some("Second"),
            line_three: Some("Third"),
            city_name: Some("Berlin"),
            country_id: CountryCode::Germany,
        },
        region: Some("Berlin"),
    };
    let charge = |category_trade_tax| TradeAllowanceCharge {
        charge: true,
        calculation_percent: Some(5.0),
        basis_amount: Some(100.0),
        actual_amount: 5.0,
        reason_code: Some(TradeAllowanceChargeReasonCode::Charge(
            ChargeCode::Advertising,
        )),
        reason: Some("Advertising"),
        category_trade_tax,
    };
    let details = AdditionalInvoiceDetails {
        seller_description: Some("Legal information"),
        seller_contact: Some(TradeContact {
            person_name: Some("Seller contact"),
            telephone: Some("123456"),
            email: Some("seller@example.org"),
            ..Default::default()
        }),
        buyer_contact: Some(TradeContact {
            department_name: Some("Purchasing"),
            telephone: Some("987654"),
            email: Some("buyer@example.org"),
            ..Default::default()
        }),
        seller_region: Some("Bavaria"),
        buyer_region: Some("Hesse"),
        tax_representative: Some(TaxRepresentative {
            name: "Tax representative",
            vat_id: "DE222222222",
            address: address(),
        }),
        ship_to: Some(ShipTo { address: address() }),
        seller_order_reference: Some("SALES-1"),
        tender_reference: Some("TENDER-1"),
        object_reference: Some(ObjectReference {
            id: "OBJECT-1",
            scheme_id: Some("ABT"),
        }),
        supporting_documents: vec![SupportingDocument {
            id: "DOC-1",
            description: Some("Attachment"),
            uri: Some("https://example.org/doc"),
            attachment: Some(DocumentAttachment {
                content: b"hello",
                mime_code: "text/csv",
                filename: "doc.csv",
            }),
        }],
        project: Some(ProjectReference {
            id: "PROJECT-1",
            name: "Project",
        }),
        receiving_advice_reference: Some("RECEIPT-1"),
        accounting_currency_tax: Some(TaxTotalAmount::new(CurrencyCode::SwissFranc, 18.0)),
        tax_point: Some(TaxPoint::Date(date(2024, 8, 1))),
        document_charges: vec![charge(Some(CategoryTradeTax {
            type_code: "VAT",
            category_code: VATCategoryCode::StandardRate,
            rate_applicable_percent: Some(19.0),
        }))],
        line_details: vec![InvoiceLineDetails {
            seller_assigned_id: Some("SELLER-ITEM"),
            buyer_assigned_id: Some("BUYER-ITEM"),
            origin_country: Some(CountryCode::France),
            classifications: vec![ProductClassification {
                code: "1234",
                list_id: "STI",
                list_version_id: Some("1"),
            }],
            characteristics: vec![ProductCharacteristic {
                name: "Colour",
                value: "Blue",
            }],
            buyer_order_line_id: Some("ORDER-LINE"),
            object_reference: Some(ObjectReference {
                id: "LINE-OBJECT",
                scheme_id: Some("ABT"),
            }),
            period_start: Some(date(2024, 8, 1)),
            period_end: Some(date(2024, 8, 31)),
            charges: vec![charge(None)],
            buyer_accounting_reference: Some("ACCOUNT-1"),
        }],
    };
    let mut builder = builder_for(SpecificationLevel::En16931);
    builder
        .set_sellers_postal_trade_address_line_three("Seller third line")
        .set_buyers_postal_trade_address_line_three("Buyer third line");
    let xml = builder
        .build_with_details(details.clone())
        .unwrap()
        .to_xml_string()
        .unwrap();
    for expected in [
        "<ram:Description>Legal information</ram:Description>",
        "<ram:PersonName>Seller contact</ram:PersonName>",
        "<ram:DepartmentName>Purchasing</ram:DepartmentName>",
        "<ram:CompleteNumber>123456</ram:CompleteNumber>",
        "<ram:CompleteNumber>987654</ram:CompleteNumber>",
        "<ram:URIID>seller@example.org</ram:URIID>",
        "<ram:URIID>buyer@example.org</ram:URIID>",
        "<ram:LineThree>Seller third line</ram:LineThree>",
        "<ram:LineThree>Buyer third line</ram:LineThree>",
        "<ram:LineThree>Third</ram:LineThree>",
        "<ram:CountrySubDivisionName>Bavaria</ram:CountrySubDivisionName>",
        "<ram:CountrySubDivisionName>Hesse</ram:CountrySubDivisionName>",
        "<ram:CountrySubDivisionName>Berlin</ram:CountrySubDivisionName>",
        "<ram:SellerTaxRepresentativeTradeParty>",
        "<ram:Name>Tax representative</ram:Name>",
        "<ram:ID schemeID=\"VA\">DE222222222</ram:ID>",
        "<ram:PostcodeCode>12345</ram:PostcodeCode>",
        "<ram:LineOne>First</ram:LineOne>",
        "<ram:LineTwo>Second</ram:LineTwo>",
        "<ram:CityName>Berlin</ram:CityName>",
        "<ram:ShipToTradeParty>",
        "<ram:IssuerAssignedID>SALES-1</ram:IssuerAssignedID>",
        "<ram:TypeCode>50</ram:TypeCode>",
        "<ram:IssuerAssignedID>OBJECT-1</ram:IssuerAssignedID><ram:TypeCode>130</ram:TypeCode><ram:ReferenceTypeCode>ABT</ram:ReferenceTypeCode>",
        "<ram:TypeCode>916</ram:TypeCode>",
        "<ram:Name>Attachment</ram:Name>",
        "<ram:URIID>https://example.org/doc</ram:URIID>",
        "<ram:AttachmentBinaryObject mimeCode=\"text/csv\" filename=\"doc.csv\">aGVsbG8=</ram:AttachmentBinaryObject>",
        "<ram:ID>PROJECT-1</ram:ID>",
        "<ram:Name>Project</ram:Name>",
        "<ram:IssuerAssignedID>RECEIPT-1</ram:IssuerAssignedID>",
        "<ram:TaxCurrencyCode>CHF</ram:TaxCurrencyCode>",
        "<ram:TaxTotalAmount currencyID=\"CHF\">18.00</ram:TaxTotalAmount>",
        "<udt:DateString format=\"102\">20240801</udt:DateString>",
        "<ram:SellerAssignedID>SELLER-ITEM</ram:SellerAssignedID>",
        "<ram:BuyerAssignedID>BUYER-ITEM</ram:BuyerAssignedID>",
        "<ram:ClassCode listID=\"STI\" listVersionID=\"1\">1234</ram:ClassCode>",
        "<ram:OriginTradeCountry><ram:ID>FR</ram:ID></ram:OriginTradeCountry>",
        "<ram:Description>Colour</ram:Description>",
        "<ram:Value>Blue</ram:Value>",
        "<ram:LineID>ORDER-LINE</ram:LineID>",
        "<ram:IssuerAssignedID>LINE-OBJECT</ram:IssuerAssignedID><ram:TypeCode>130</ram:TypeCode><ram:ReferenceTypeCode>ABT</ram:ReferenceTypeCode>",
        "<ram:StartDateTime><udt:DateTimeString format=\"102\">20240801</udt:DateTimeString></ram:StartDateTime>",
        "<ram:EndDateTime><udt:DateTimeString format=\"102\">20240831</udt:DateTimeString></ram:EndDateTime>",
        "<ram:ID>ACCOUNT-1</ram:ID>",
    ] {
        assert!(xml.contains(expected), "missing {expected}\n{xml}");
    }
    assert_eq!(
        xml.matches("<ram:ReasonCode>AA</ram:ReasonCode>").count(),
        2
    );
    assert_eq!(
        xml.matches("<ram:CalculationPercent>5.00</ram:CalculationPercent>")
            .count(),
        2
    );
    validate(&xml, "EN16931");

    builder
        .set_sellers_description(details.seller_description.unwrap())
        .set_seller_contact(details.seller_contact.unwrap())
        .set_buyer_contact(details.buyer_contact.unwrap())
        .set_seller_region(details.seller_region.unwrap())
        .set_buyer_region(details.buyer_region.unwrap())
        .set_tax_representative(details.tax_representative.unwrap())
        .set_ship_to(details.ship_to.unwrap())
        .set_seller_order_reference(details.seller_order_reference.unwrap())
        .set_tender_reference(details.tender_reference.unwrap())
        .set_object_reference(details.object_reference.unwrap())
        .set_project(details.project.unwrap())
        .set_receiving_advice_reference(details.receiving_advice_reference.unwrap())
        .set_accounting_currency_tax(details.accounting_currency_tax.unwrap())
        .set_tax_point(details.tax_point.unwrap())
        .set_line_details(details.line_details);
    for document in details.supporting_documents {
        builder.add_supporting_document(document);
    }
    for charge in details.document_charges {
        builder.add_document_charge(charge);
    }
    let invoice = builder.build(SpecificationLevel::En16931).unwrap();
    assert_eq!(xml, invoice.to_xml_string().unwrap());
    assert_eq!(
        xml,
        builder
            .clone()
            .to_xml_string(SpecificationLevel::En16931)
            .unwrap()
    );
    assert_eq!(
        xml,
        builder
            .build_with_details(AdditionalInvoiceDetails::default())
            .unwrap()
            .to_xml_string()
            .unwrap()
    );
    assert_eq!(
        xml.strip_prefix("<?xml version='1.0' encoding='UTF-8'?>")
            .unwrap(),
        quick_xml::se::to_string(&invoice).unwrap()
    );
}

#[test]
fn builder_reports_profile_and_post_build_line_mismatches() {
    use zugferd::{AdditionalInvoiceDetails, InvoiceLineDetails, ProjectReference};
    let mut builder = builder_for(SpecificationLevel::En16931);
    builder.set_project(ProjectReference {
        id: "P",
        name: "Project",
    });
    assert!(builder
        .build(SpecificationLevel::Basic)
        .unwrap_err()
        .contains("require the EN 16931 or XRechnung profile"));
    let duplicate = AdditionalInvoiceDetails {
        project: Some(ProjectReference {
            id: "Q",
            name: "Other project",
        }),
        ..Default::default()
    };
    assert!(builder
        .build_with_details(duplicate)
        .unwrap_err()
        .contains("project was supplied twice"));
    builder.set_line_details(vec![InvoiceLineDetails::default()]);
    let mut invoice = builder.build(SpecificationLevel::En16931).unwrap();
    invoice.context.guideline.id = SpecificationLevel::Minimum;
    assert!(invoice
        .to_xml_string()
        .unwrap_err()
        .contains("require the EN 16931 or XRechnung profile"));
    invoice.context.guideline.id = SpecificationLevel::En16931;
    invoice
        .supply_chain_trade_transaction
        .included_supply_chain_trade_line_items
        .clear();
    assert!(invoice
        .to_xml_string()
        .unwrap_err()
        .contains("one entry per invoice line"));
}

#[test]
fn builder_includes_en16931_fields_for_xrechnung() {
    use zugferd::ProjectReference;
    let mut builder = builder_for(SpecificationLevel::XRechnung);
    builder.set_project(ProjectReference {
        id: "P",
        name: "Project",
    });
    let xml = builder
        .to_xml_string(SpecificationLevel::XRechnung)
        .unwrap();
    assert!(xml.contains("<ram:SpecifiedProcuringProject><ram:ID>P</ram:ID><ram:Name>Project</ram:Name></ram:SpecifiedProcuringProject>"));
    assert!(xml.contains("urn:xeinkauf.de:kosit:xrechnung_3.0"));
    validate(&xml, "EN16931");
}

#[test]
fn extended_en16931_rejects_invalid_combinations() {
    use zugferd::{
        AdditionalInvoiceDetails, DocumentAttachment, InvoiceLineDetails, SupportingDocument,
        TaxPoint, TaxTotalAmount, TradeAllowanceCharge, TradeAllowanceChargeReasonCode,
    };
    let level = SpecificationLevel::En16931;
    let mut builder = builder_for(level);
    let details = AdditionalInvoiceDetails {
        line_details: vec![InvoiceLineDetails::default(), InvoiceLineDetails::default()],
        ..Default::default()
    };
    assert!(builder
        .build_with_details(details)
        .unwrap_err()
        .contains("one entry per invoice line"));
    let details = AdditionalInvoiceDetails {
        accounting_currency_tax: Some(TaxTotalAmount::new(CurrencyCode::Euro, 19.0)),
        ..Default::default()
    };
    assert!(builder
        .build_with_details(details)
        .unwrap_err()
        .contains("must differ"));
    let mut details = AdditionalInvoiceDetails::default();
    details.document_charges.push(TradeAllowanceCharge {
        charge: true,
        calculation_percent: None,
        basis_amount: None,
        actual_amount: 1.0,
        reason_code: Some(TradeAllowanceChargeReasonCode::Allowance(
            AllowanceChargeReasonCode::Discount,
        )),
        reason: None,
        category_trade_tax: Some(CategoryTradeTax {
            type_code: "VAT",
            category_code: VATCategoryCode::StandardRate,
            rate_applicable_percent: Some(19.0),
        }),
    });
    assert!(builder
        .build_with_details(details)
        .unwrap_err()
        .contains("reason code"));
    let details = AdditionalInvoiceDetails {
        tax_point: Some(TaxPoint::DateCode("3")),
        ..Default::default()
    };
    assert!(builder
        .build_with_details(details)
        .unwrap_err()
        .contains("5, 29, or 72"));
    let details = AdditionalInvoiceDetails {
        line_details: vec![InvoiceLineDetails {
            period_start: Some(date(2024, 9, 1)),
            period_end: Some(date(2024, 8, 1)),
            ..Default::default()
        }],
        ..Default::default()
    };
    assert!(builder
        .build_with_details(details)
        .unwrap_err()
        .contains("start must not be after"));
    let details = AdditionalInvoiceDetails {
        supporting_documents: vec![SupportingDocument {
            id: "doc",
            description: None,
            uri: None,
            attachment: Some(DocumentAttachment {
                content: b"content",
                mime_code: "text/plain",
                filename: "doc.txt",
            }),
        }],
        ..Default::default()
    };
    assert!(builder
        .build_with_details(details)
        .unwrap_err()
        .contains("MIME code"));
}

#[test]
fn extended_en16931_line_details_follow_line_order() {
    use zugferd::{AdditionalInvoiceDetails, InvoiceLineDetails};
    let mut builder = builder_for(SpecificationLevel::En16931);
    builder.add_supply_chain_trade_line_item(line_item("2", "1234567890123", "Second", 10.0, 1.0));
    let details = AdditionalInvoiceDetails {
        line_details: vec![
            InvoiceLineDetails {
                seller_assigned_id: Some("FIRST"),
                ..Default::default()
            },
            InvoiceLineDetails {
                seller_assigned_id: Some("SECOND"),
                ..Default::default()
            },
        ],
        ..Default::default()
    };
    let xml = builder
        .build_with_details(details)
        .unwrap()
        .to_xml_string()
        .unwrap();
    let lines: Vec<_> = xml
        .split("<ram:IncludedSupplyChainTradeLineItem>")
        .skip(1)
        .map(|part| {
            part.split("</ram:IncludedSupplyChainTradeLineItem>")
                .next()
                .unwrap()
        })
        .collect();
    assert_eq!(lines.len(), 2);
    assert!(lines[0].contains("<ram:SellerAssignedID>FIRST</ram:SellerAssignedID>"));
    assert!(lines[1].contains("<ram:SellerAssignedID>SECOND</ram:SellerAssignedID>"));
    validate(&xml, "EN16931");
}

#[test]
fn extended_en16931_preserves_existing_and_new_allowances() {
    use zugferd::{AdditionalInvoiceDetails, TradeAllowanceCharge, TradeAllowanceChargeReasonCode};
    let mut builder = builder_for(SpecificationLevel::En16931);
    let vat = CategoryTradeTax {
        type_code: "VAT",
        category_code: VATCategoryCode::StandardRate,
        rate_applicable_percent: Some(19.0),
    };
    builder.add_specified_trade_allowance_charge(SpecifiedTradeAllowanceCharge {
        charge_indicator: ChargeIndicator { indicator: false },
        calculation_percent: None,
        basis_amount: None,
        actual_amount: 5.0,
        reason_code: Some(AllowanceChargeReasonCode::Discount),
        reason: None,
        category_trade_tax: vat,
    });
    let details = AdditionalInvoiceDetails {
        document_charges: vec![TradeAllowanceCharge {
            charge: true,
            calculation_percent: None,
            basis_amount: None,
            actual_amount: 5.0,
            reason_code: Some(TradeAllowanceChargeReasonCode::Charge(
                ChargeCode::Advertising,
            )),
            reason: None,
            category_trade_tax: Some(vat),
        }],
        ..Default::default()
    };
    let xml = builder
        .build_with_details(details)
        .unwrap()
        .to_xml_string()
        .unwrap();
    assert_eq!(
        xml.matches("<ram:SpecifiedTradeAllowanceCharge>").count(),
        2
    );
    assert!(xml.contains("<ram:ReasonCode>95</ram:ReasonCode>"));
    assert!(xml.contains("<ram:ReasonCode>AA</ram:ReasonCode>"));
    validate(&xml, "EN16931");
}

#[test]
fn extended_en16931_without_details_preserves_existing_output() {
    let level = SpecificationLevel::En16931;
    let mut builder = builder_for(level);
    let legacy = builder.build(level).unwrap().to_xml_string().unwrap();
    let extended = builder
        .build_with_details(zugferd::AdditionalInvoiceDetails::default())
        .unwrap()
        .to_xml_string()
        .unwrap();
    assert_eq!(legacy, extended);
    validate(&extended, "EN16931");
}

#[test]
fn extended_en16931_uses_existing_contact_setters_and_tax_point_code() {
    use zugferd::{AdditionalInvoiceDetails, TaxPoint, TradeContact};
    let mut builder = builder_for(SpecificationLevel::En16931);
    builder
        .set_sellers_description("Company details")
        .set_sellers_definedtrade_contact_person_name("Seller")
        .set_sellers_definedtrade_contact_complete_number("123")
        .set_buyers_definedtrade_contact_department_name("Accounts");
    let details = AdditionalInvoiceDetails {
        tax_point: Some(TaxPoint::DateCode("5")),
        seller_contact: Some(TradeContact {
            email: Some("seller@example.org"),
            ..Default::default()
        }),
        ..Default::default()
    };
    let xml = builder
        .build_with_details(details)
        .unwrap()
        .to_xml_string()
        .unwrap();
    assert!(xml.contains("<ram:Description>Company details</ram:Description>"));
    assert!(xml.contains("<ram:TelephoneUniversalCommunication><ram:CompleteNumber>123</ram:CompleteNumber></ram:TelephoneUniversalCommunication>"));
    assert!(xml.contains("<ram:DepartmentName>Accounts</ram:DepartmentName>"));
    assert!(xml.contains("<ram:EmailURIUniversalCommunication><ram:URIID>seller@example.org</ram:URIID></ram:EmailURIUniversalCommunication>"));
    assert!(xml.contains("<ram:DueDateTypeCode>5</ram:DueDateTypeCode>"));
    assert!(!xml.contains("<ram:TaxPointDate>"));
    validate(&xml, "EN16931");
    let duplicate = AdditionalInvoiceDetails {
        seller_contact: Some(TradeContact {
            person_name: Some("Other"),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert!(builder
        .build_with_details(duplicate)
        .unwrap_err()
        .contains("Contact field was supplied both"));
    builder
        .set_seller_contact(TradeContact {
            email: Some("seller@example.org"),
            ..Default::default()
        })
        .set_tax_point(TaxPoint::DateCode("5"));
    assert_eq!(
        xml,
        builder
            .build(SpecificationLevel::En16931)
            .unwrap()
            .to_xml_string()
            .unwrap()
    );
}

#[test]
fn builder_contact_setters_share_one_contact_per_party() {
    use zugferd::{AdditionalInvoiceDetails, TradeContact};

    let mut builder = builder_for(SpecificationLevel::En16931);
    builder
        .set_seller_contact(TradeContact {
            person_name: Some("Initial seller"),
            email: Some("old@example.org"),
            ..Default::default()
        })
        .set_sellers_definedtrade_contact_person_name("Seller")
        .set_sellers_definedtrade_contact_complete_number("123")
        .set_sellers_definedtrade_contact_email("seller@example.org")
        .set_buyers_definedtrade_contact_department_name("Accounts")
        .set_buyers_definedtrade_contact_email("buyer@example.org")
        .set_buyer_contact(TradeContact {
            telephone: Some("456"),
            ..Default::default()
        });

    let xml = builder
        .build(SpecificationLevel::En16931)
        .unwrap()
        .to_xml_string()
        .unwrap();
    assert_eq!(xml.matches("<ram:DefinedTradeContact>").count(), 2);
    for expected in [
        "<ram:PersonName>Seller</ram:PersonName>",
        "<ram:DepartmentName>Accounts</ram:DepartmentName>",
        "<ram:CompleteNumber>123</ram:CompleteNumber>",
        "<ram:CompleteNumber>456</ram:CompleteNumber>",
        "<ram:URIID>seller@example.org</ram:URIID>",
        "<ram:URIID>buyer@example.org</ram:URIID>",
    ] {
        assert!(xml.contains(expected), "missing {expected}");
    }
    assert!(!xml.contains("old@example.org"));
    assert!(!xml.contains("Initial seller"));
    validate(&xml, "EN16931");

    let duplicate = AdditionalInvoiceDetails {
        buyer_contact: Some(TradeContact {
            email: Some("other@example.org"),
            ..Default::default()
        }),
        ..Default::default()
    };
    assert!(builder
        .build_with_details(duplicate)
        .unwrap_err()
        .contains("Contact field was supplied both"));
    assert!(builder
        .build(SpecificationLevel::Basic)
        .unwrap_err()
        .contains("require the EN 16931 or XRechnung profile"));
}

#[test]
fn builder_serialization_includes_contacts_and_en16931_fields() {
    use zugferd::ProjectReference;

    let mut builder = zugferd::InvoiceBuilder::new();
    builder
        .set_sellers_definedtrade_contact_person_name("Seller")
        .set_sellers_definedtrade_contact_email("seller@example.org")
        .set_project(ProjectReference {
            id: "P-1",
            name: "Project",
        });
    let xml = quick_xml::se::to_string(&builder).unwrap();
    assert!(xml.contains("<ram:PersonName>Seller</ram:PersonName>"));
    assert!(xml.contains("<ram:Email>seller@example.org</ram:Email>"));
    assert!(xml.contains("<project><id>P-1</id><name>Project</name></project>"));
    assert!(!xml.contains("<seller_contact>"));
}

#[test]
fn basic_with_direct_debit_mandate_validates_against_xsd() {
    let level = SpecificationLevel::Basic;
    let mut builder = builder_for(level);
    builder.set_specified_trade_payment_terms_direct_debit_mandate_id("MANDATE-123");
    let xml = builder.to_xml_string(level).expect("invoice should build");
    assert!(xml.contains("<ram:DirectDebitMandateID>MANDATE-123</ram:DirectDebitMandateID>"));
    validate(&xml, "BASIC");
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
    validate(&xml, "BASICWL");
}

#[test]
fn vat_exemption_setters_update_only_the_first_breakdown() {
    let level = SpecificationLevel::En16931;
    let mut builder = builder_for(level);
    builder
        .set_applicable_trade_tax_category_code(VATCategoryCode::ExemptFromTax)
        .set_applicable_trade_tax_calculated_amount(0.0)
        .set_applicable_trade_tax_rate_applicable_percent(0.0)
        .set_applicable_trade_tax_exemption_reason("Exempt")
        .set_applicable_trade_tax_exemption_reason_code("VATEX-EU-132")
        .add_applicable_trade_tax(ApplicableTradeTax {
            basis_amount: Some(50.0),
            calculated_amount: Some(9.5),
            rate_applicable_percent: Some(19.0),
            ..Default::default()
        });
    let xml = builder.to_xml_string(level).expect("invoice should build");
    let taxes: Vec<_> = xml
        .split("<ram:ApplicableHeaderTradeSettlement>")
        .nth(1)
        .unwrap()
        .split("<ram:ApplicableTradeTax>")
        .skip(1)
        .map(|part| part.split("</ram:ApplicableTradeTax>").next().unwrap())
        .collect();
    assert_eq!(taxes.len(), 2);
    assert!(taxes[0].contains("<ram:ExemptionReason>Exempt</ram:ExemptionReason>"));
    assert!(taxes[0].contains("<ram:ExemptionReasonCode>VATEX-EU-132</ram:ExemptionReasonCode>"));
    assert!(!taxes[1].contains("ExemptionReason"));
    validate(&xml, "EN16931");
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
