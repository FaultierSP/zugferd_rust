use zugferd::*;

fn main() {
    //Initialize and pass first data
    let mut invoice_builder = InvoiceBuilder::new();

    invoice_builder
        .set_business_process("process1")
        .set_invoice_type_code(InvoiceTypeCode::CommercialInvoice)
        .set_invoice_nr("INV-123456")
        .set_date_of_issue(chrono::NaiveDate::from_ymd_opt(2024, 08, 10).unwrap())
        .set_buyer_reference("BR-7890")
        .set_sellers_name("Seller Corp.")
        .set_sellers_specified_legal_organization("LegalOrg-001")
        .set_sellers_postal_trade_address_country_code(CountryCode::Germany)
        .set_sellers_specified_tax_registration("DE123456789")
        .set_buyers_name("Buyer Inc.")
        .set_buyers_specified_legal_organization("LegalOrg-002")
        .set_buyers_order_specified_document("OD-2024-001")
        .set_invoice_currency_code(CurrencyCode::Euro);

    //Do your computations
    let sum_net: f64 = 100.0;
    let tax: f64 = sum_net * 19.0 / 100.0;
    let sum_gross: f64 = sum_net + tax;
    let customer_paid_already: f64 = 50.0;

    invoice_builder
        .set_monetary_summation_tax_basis_total_amount(sum_net)
        .set_monetary_summation_tax_total_amount(tax)
        .set_monetary_summation_grand_total_amount(sum_gross)
        .set_monetary_summation_due_payable_amount(sum_gross - customer_paid_already);

    let invoice: Invoice = invoice_builder
        .build(SpecificationLevel::Minimum)
        .expect("that we could build the invoice");

    let xml_string = invoice
        .to_xml_string()
        .expect("that we could render the invoice as xml");

    let _ = zugferd::components::functions::write_xml_to_file(
        xml_string,
        "examples/generated_minimum.xml",
        true,
    );
}
