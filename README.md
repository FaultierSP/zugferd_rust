# Generator of ZUGFeRD XML
## Summary
This crate generates an XML with the ZUGFeRD specification that can be embedded in PDF to generate E-invoice compliant with the EU regulations.

> [!WARNING]
> This crate is pretty much work in progress.

Although you can generate a minimum and basic wl level XML, this crate is more of a starting point and an invitation for feedback and collaboration. Breaking changes are not intended, but are not impossible either.

## Installation
Add the crate:
~~~
cargo add zugferd
~~~
Import the crate:
~~~rs
use zugferd::{InvoiceBuilder,InvoiceTypeCode,CountryCode,CurrencyCode,SpecificationLevel};
~~~
Initialize and pass first data:
~~~rs
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
~~~
You can always check if the provided data enough for the specified level. (At this stage only "minimum" is supported.)
~~~rs
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
~~~
Or simply
~~~rs
invoice_builder.all_fields_are_set(SpecificationLevel::Minimum)?;
~~~
Calculate your data further, for example:
~~~rs
let sum_net: f64 = 100.0;
let tax: f64 = sum_net * 19.0 /100.0;
let sum_gross: f64 = sum_net + tax;
let customer_paid_already: f64 = 50.0;
~~~
Pass missing data to the instance:
~~~rs
invoice_builder
    .set_monetary_summation_tax_basis_total_amount(sum_net)
    .set_monetary_summation_tax_total_amount(tax)
    .set_monetary_summation_grand_total_amount(sum_gross)
    .set_monetary_summation_due_payable_amount(sum_gross - customer_paid_already);
~~~
Generate XML:
~~~rs
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
~~~
Please check `main.rs` for further examples.
## Roadmap
- [x] generation of minimum level
- [x] generation of basic level without lines
- [x] generation of basic level
- [ ] generation of EN 16931 level
- [ ] generation of extended level
- [ ] validation of all levels
- [ ] parsing of all levels
- [ ] embedding the generated XML into PDF/A-3 files
## Further reading

[ZUGFeRD 2.3 specification](https://www.ferd-net.de/standards/zugferd-2.3/zugferd-2.3.html)
