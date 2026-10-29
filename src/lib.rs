//! # Zugferd
//!
//! Build the zugferd datastructure in rust  by using [InvoiceBuilder]:
//!
//! ```rust
//! use zugferd::*;
//!
//! let mut invoice_builder = InvoiceBuilder::new();
//!
//! invoice_builder.set_business_process("process1")
//!    .set_invoice_type_code(InvoiceTypeCode::CommercialInvoice)
//!    .set_invoice_nr("INV-123456")
//!    .set_date_of_issue(chrono::NaiveDate::from_ymd_opt(2024,08,10).unwrap())
//!    .set_buyer_reference("BR-7890")
//!    .set_sellers_name("Seller Corp.")
//!    .set_sellers_specified_legal_organization("LegalOrg-001")
//!    .set_sellers_postal_trade_address_country_code(CountryCode::Germany)
//!    .set_sellers_specified_tax_registration("DE123456789")
//!    .set_buyers_name("Buyer Inc.")
//!    .set_buyers_specified_legal_organization("LegalOrg-002")
//!    .set_buyers_order_specified_document("OD-2024-001")
//!    .set_invoice_currency_code(CurrencyCode::Euro);
//! ```
//! Validate that all fields are defined for a certain [SpecificationLevel]:
//!
//! ```rust
//! # use zugferd::*;
//! # let mut invoice_builder = InvoiceBuilder::new();
//! match invoice_builder.all_fields_are_set(SpecificationLevel::Minimum) {
//!     Ok(_) => {
//!         //Carry on
//!         println!("All fields are set.");
//!     },
//!     Err(e) => {
//!         //Check what is missing
//!         println!("I want this: {}",e);
//!     }
//! }
//! ```
//!
//! Finally, generate the xml for a certain [SpecificationLevel]:
//!
//! ```rust
//! # use zugferd::*;
//! # let mut invoice_builder = InvoiceBuilder::new();
//! let mut xml_string: String = String::new();
//!
//! match invoice_builder.to_xml_string(SpecificationLevel::Minimum) {
//!     Ok(string_returned_by_function) => {
//!         xml_string = string_returned_by_function;
//!     },
//!     Err(e) => {
//!         println!("Something happened at the XML generation: {}",e);
//!     }
//! }
//!
//! println!("Generated ZUGFeRD XML: {}",xml_string);
//!
//! ```

pub mod components;
pub mod invoice_builder;

pub use crate::components::enums::{
    country_code::CountryCode, currency_code::CurrencyCode, invoice_type_code::InvoiceTypeCode,
    specification_level::SpecificationLevel, vat_category_code::VATCategoryCode,
};
pub use components::business_rules::validate as validate_business_rules;
pub use components::structs::*;
pub use invoice_builder::{InvoiceBuilder, InvoiceBuilderError};
