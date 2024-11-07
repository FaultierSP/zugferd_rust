use zugferd::components::enums::unit_code::UnitCode;
use zugferd::{ApplicableTradeTax, BilledQuantity, CountryCode, CurrencyCode, InvoiceBuilder, InvoiceTypeCode, NetPriceProductTradePrice, SpecificationLevel, SpecifiedTradeSettlementLineMonetarySummation};
// For Basic specification
use zugferd::{
    IncludedSupplyChainTradeLineItem,
    AssociatedDocumentLineDocument,
    SpecifiedTradeProduct,
    GlobalID,
    SpecifiedLineTradeAgreement,
    SpecifiedLineTradeDelivery,
    SpecifiedLineTradeSettlement,
    components::enums::identifier_scheme_code::IdentifierSchemeCode,
};

fn main() {
    //Initialize and pass first data
    let mut invoice_builder = InvoiceBuilder::new();

    invoice_builder.set_business_process("process1")
                    .set_invoice_type_code(InvoiceTypeCode::CommercialInvoice)
                    .set_invoice_nr("INV-123456")
                    .set_date_of_issue(chrono::NaiveDate::from_ymd_opt(2024,08,10).unwrap())
                    .set_buyer_reference("BR-7890")
                    .set_sellers_name("Seller Corp.")
                    .set_sellers_specified_legal_organization("LegalOrg-001")
                    .set_sellers_postal_trade_address_country_code(CountryCode::Germany)
                    .set_sellers_specified_tax_registration("DE123456789")
                    .set_buyers_name("Buyer Inc.")
                    .set_buyers_specified_legal_organization("LegalOrg-002")
                    .set_buyers_order_specified_document("OD-2024-001")
                    .set_invoice_currency_code(CurrencyCode::Euro);

    // You can check if all the fields for your desired specification level are set:
    match invoice_builder.all_fields_are_set(SpecificationLevel::Minimum) {
        Ok(_) => {
            //Carry on
            println!("All fields are set.");
        },
        Err(e) => {
            //Check what is missing
            println!("I want this: {}",e);
        }
    }

    //Do your computations
    let sum_net: f64 = 100.0;
    let tax: f64 = sum_net * 19.0 /100.0;
    let sum_gross: f64 = sum_net + tax;
    let customer_paid_already: f64 = 50.0;

    invoice_builder.set_monetary_summation_tax_basis_total_amount(sum_net)
            .set_monetary_summation_tax_total_amount(tax)
            .set_monetary_summation_grand_total_amount(sum_gross)
            .set_monetary_summation_due_payable_amount(sum_gross - customer_paid_already);

    //You can check again
    match invoice_builder.all_fields_are_set(SpecificationLevel::Minimum) {
        Ok(_) => {
            //Carry on
            println!("All fields are set.");
        },
        Err(e) => {
            //Check what is missing
            println!("I want this: {}",e);
            return;
        }
    }

    //Setting further data
    //Basic WL
    invoice_builder.set_invoice_notes(vec![
        "Note 1",
        "Very important note 2",
    ]);

    invoice_builder
        .set_sellers_postal_trade_address_line_one("Best street")
        .set_sellers_postal_trade_address_line_two("Building 3")
        .set_sellers_postal_trade_address_line_three("Ap. 18")
        .set_sellers_postal_trade_address_postcode_code("66666")
        .set_sellers_postal_trade_address_city_name("Hometown");

    invoice_builder
        .set_buyers_postal_trade_address_line_one("Main street")
        .set_buyers_postal_trade_address_line_two("Near pizzeria")
        .set_buyers_postal_trade_address_postcode_code("777777")
        .set_buyers_postal_trade_address_city_name("Springfield");

    invoice_builder
        .set_occurrence_date(chrono::NaiveDate::from_ymd_opt(2024,07,06).unwrap());

    invoice_builder
        .set_applicable_trade_tax_basis_amount(sum_net)
        .set_applicable_trade_tax_calculated_amount(12.44)
        .set_applicable_trade_tax_rate_applicable_percent(42.0);

    invoice_builder.set_specified_trade_payment_terms_due_date(chrono::NaiveDate::from_ymd_opt(2025,4,12).unwrap());

    invoice_builder
        .set_monetary_summation_line_total_amount(16.90)
        .set_monetary_summation_charge_total_amount(0.0)
        .set_monetary_summation_allowance_total_amount(0.0);

    //Going even further with Basic specification
    invoice_builder.add_supply_chain_trade_line_item(IncludedSupplyChainTradeLineItem {
        associated_document_line_document:AssociatedDocumentLineDocument {
            line_id: "1",
            included_note: None,
        },
        specified_trade_product:SpecifiedTradeProduct {
            global_id: Some(GlobalID {
                value: "1234567890123",
                scheme_id: IdentifierSchemeCode::GTIN,
            }),
            name: "Product 1",
        },
        specified_line_trade_agreement:SpecifiedLineTradeAgreement {
            net_price_product_trade_price: NetPriceProductTradePrice {
                charge_amount: 100.0,
            },
        },
        specified_line_trade_delivery:SpecifiedLineTradeDelivery {
            billed_quantity: BilledQuantity {
                value: 1.0,
                unit_code: UnitCode::Piece,
            },
        },
        specified_line_trade_settlement:SpecifiedLineTradeSettlement {
            applicable_trade_tax: Some(ApplicableTradeTax {
                calculated_amount: Some(19.0),
                type_code: "VAT",
                category_code: zugferd::VATCategoryCode::StandardRate,
                basis_amount: Some(100.0),
                rate_applicable_percent: Some(19.0),
            }),
            specified_trade_settlement_line_monetary_summation: SpecifiedTradeSettlementLineMonetarySummation {
                line_total_amount: 119.0,
            },
        },
    });

    invoice_builder.add_supply_chain_trade_line_item(IncludedSupplyChainTradeLineItem {
        associated_document_line_document:AssociatedDocumentLineDocument {
            line_id: "1",
            included_note: None,
        },
        specified_trade_product:SpecifiedTradeProduct {
            global_id: Some(GlobalID {
                value: "2546585465423",
                scheme_id: IdentifierSchemeCode::GTIN,
            }),
            name: "Product 2",
        },
        specified_line_trade_agreement:SpecifiedLineTradeAgreement {
            net_price_product_trade_price: NetPriceProductTradePrice {
                charge_amount: 1.0,
            },
        },
        specified_line_trade_delivery:SpecifiedLineTradeDelivery {
            billed_quantity: BilledQuantity {
                value: 44.0,
                unit_code: UnitCode::Piece,
            },
        },
        specified_line_trade_settlement:SpecifiedLineTradeSettlement {
            applicable_trade_tax: Some(ApplicableTradeTax {
                calculated_amount: Some(0.44*19.0),
                type_code: "VAT",
                category_code: zugferd::VATCategoryCode::StandardRate,
                basis_amount: Some(44.0),
                rate_applicable_percent: Some(19.0),
            }),
            specified_trade_settlement_line_monetary_summation: SpecifiedTradeSettlementLineMonetarySummation {
                line_total_amount: (44.0*1.19),
            },
        },
    });

    //Convert the instance to an actual XML string
    let mut xml_string: String = String::new();

    match invoice_builder.to_xml_string(SpecificationLevel::Basic) {
        Ok(string_returned_by_function) => {
            xml_string = string_returned_by_function;
        },
        Err(e) => {
            println!("Something happened at the XML generation: {}",e);
        }
    }

    //println!("Generated ZUGFeRD XML: {}",xml_string);

    let _ = zugferd::components::functions::write_xml_to_file(xml_string,"examples/generated_basic.xml",true);
}
