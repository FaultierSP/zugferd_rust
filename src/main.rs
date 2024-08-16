use zugferd::{InvoiceBuilder,InvoiceTypeCode,CountryCode,CurrencyCode,SpecificationLevel};

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
    
    invoice_builder.set_tax_basis_total_amount(sum_net)
            .set_tax_total_amount(tax)
            .set_grand_total_amount(sum_gross)
            .set_due_payable_amount(sum_gross - customer_paid_already);

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

    //Convert the instance to an actual XML string
    let mut xml_string: String = String::new();

    match invoice_builder.to_xml_string(SpecificationLevel::Minimum) {
        Ok(string_returned_by_function) => {
            xml_string = string_returned_by_function;
        },
        Err(e) => {
            println!("Something happened at the XML generation: {}",e);
        }
    }

    println!("Generated ZUGFeRD XML: {}",xml_string);

    let _ = zugferd::components::functions::write_xml_to_file(xml_string,"examples/generated_minimum.xml",true);
}
