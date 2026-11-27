# Generator of ZUGFeRD XML

## Summary

This crate generates an XML with the ZUGFeRD specification that can be embedded in PDF to generate E-invoice compliant with the EU regulations.

> [!WARNING]
> This crate is pretty much work in progress.

Although you can generate a minimum and basic levels XML, this crate is more of a starting point and an invitation for feedback and collaboration. Breaking changes are not intended, but are not impossible either.

## Installation

Add the crate:

~~~bash
cargo add zugferd
~~~

Import the crate:

~~~rust
use zugferd::{InvoiceBuilder,InvoiceTypeCode,CountryCode,CurrencyCode,SpecificationLevel};
~~~

> [!WARNING]
> Although you can populate data into structs without the invoice builder, at this point I strongly suggest you use it. Breaking changes may occure. Besides, it takes care of conversion from Strings.

Initialize and pass first data:

~~~rust
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

You can always check if the provided data enough for the specified level. (At this stage only "minimum", "basic wl" and "basic" are supported.)

~~~rust
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

~~~rust
invoice_builder.all_fields_are_set(SpecificationLevel::Minimum)?;
~~~

Calculate your data further, for example:

~~~rust
let sum_net: f64 = 100.0;
let tax: f64 = sum_net * 19.0 /100.0;
let sum_gross: f64 = sum_net + tax;
let customer_paid_already: f64 = 50.0;
~~~

Pass missing data to the instance:

~~~rust
invoice_builder
    .set_monetary_summation_tax_basis_total_amount(sum_net)
    .set_monetary_summation_tax_total_amount(tax)
    .set_monetary_summation_grand_total_amount(sum_gross)
    .set_monetary_summation_due_payable_amount(sum_gross - customer_paid_already);
~~~

After setting the required EN 16931 data and adding invoice lines, set
additional fields on the builder:

~~~rust
use zugferd::{ProjectReference, SpecificationLevel, TaxPoint};

invoice_builder
    .set_sellers_definedtrade_contact_person_name("Seller contact")
    .set_sellers_definedtrade_contact_email("seller@example.org")
    .set_buyers_definedtrade_contact_email("buyer@example.org")
    .set_project(ProjectReference { id: "P-123", name: "Project" })
    .set_tax_point(TaxPoint::DateCode("5"));
let xml_string = invoice_builder.to_xml_string(SpecificationLevel::En16931)?;
~~~

The standard `build` and `to_xml_string` methods include these fields for the
EN 16931 and XRechnung profiles. Use `set_line_details` for additional line
fields: if provided, it must contain one entry for each invoice line, in the
same order. The contact setters update the existing seller and buyer trade
contacts; `set_seller_contact` and `set_buyer_contact` also update those same
contacts, changing only fields supplied as `Some`. Attachments take bytes and
are encoded as base64. `TradeAllowanceCharge` accepts separate allowance and
charge reason codes. Invoice totals and code-list validity remain the caller's
responsibility; XSD validation does not replace EN 16931 business-rule validation.

To build an EN 16931 invoice with a group of additional fields, use
`build_with_details(AdditionalInvoiceDetails)`. It combines its input with
builder fields and reports duplicate values rather than silently overriding them.

The EN 16931 XML mappings use `ram:ID` for a project reference,
`ram:ReferenceTypeCode` for an object-reference scheme, `ram:Description` for
an item attribute name, and `ram:DueDateTypeCode` for the VAT point date code.

Generate XML:

~~~rust
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

Document-level allowances and charges can be added with
`InvoiceBuilder::add_specified_trade_allowance_charge`. Line-level allowances
and charges belong in `SpecifiedLineTradeSettlement::specified_trade_allowance_charge`.
Document-level allowances and charges require at least Basic Without Lines.
Amounts and totals must be calculated by the caller.

For the first VAT breakdown, exemption text and code can be set using
`set_applicable_trade_tax_exemption_reason` and
`set_applicable_trade_tax_exemption_reason_code`. Additional breakdowns can
provide these fields through `add_applicable_trade_tax`.

// Please check `main.rs` for further examples.

## Roadmap

- [x] generation of minimum level
- [x] generation of basic level without lines
- [x] generation of basic level
- [x] generation of EN 16931 level
- [ ] generation of extended level
- [ ] validation of all levels
- [ ] parsing of all levels

## Further reading

ZUGFeRD 2.5.2 specification: in [German](https://www.ferd-net.de/publikationen-produkte/publikationen/detailseite/zugferd-252-deutsch) and [English](https://www.ferd-net.de/en/downloads/publications/details/zugferd-252-english)
