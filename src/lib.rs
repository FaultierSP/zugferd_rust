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

use chrono::NaiveDate;
use serde::Serialize;

pub mod components;

pub use crate::components::enums::{
    country_code::CountryCode, currency_code::CurrencyCode, invoice_type_code::InvoiceTypeCode,
    specification_level::SpecificationLevel, vat_category_code::VATCategoryCode,
};
pub use components::business_rules::validate as validate_business_rules;
pub use components::structs::*;

#[derive(Serialize, Clone)]
pub struct InvoiceBuilder<'invoice_builder> {
    //Minimal specification
    business_process: Option<&'invoice_builder str>,
    invoice_type_code: Option<InvoiceTypeCode>,
    invoice_nr: Option<&'invoice_builder str>,
    date_of_issue: Option<DateTimeString<'invoice_builder>>,
    document_notes: Option<Vec<IncludedNote>>,
    buyer_reference: Option<&'invoice_builder str>,
    sellers_name: Option<&'invoice_builder str>,
    sellers_specified_legal_organization: Option<&'invoice_builder str>,
    sellers_specified_tax_registration: Option<&'invoice_builder str>,
    buyers_name: Option<&'invoice_builder str>,
    buyers_specified_legal_organization: Option<&'invoice_builder str>,
    buyers_order_specified_document: Option<&'invoice_builder str>,
    occurrence_date: Option<DateTimeString<'invoice_builder>>,
    invoice_currency_code: Option<CurrencyCode>,
    monetary_summation: SpecifiedTradeSettlementHeaderMonetarySummation,

    //Basic WL specification
    applicable_trade_tax: Option<ApplicableTradeTax<'invoice_builder>>,
    sellers_postal_trade_address: PostalTradeAddress<'invoice_builder>,
    buyers_postal_trade_address: PostalTradeAddress<'invoice_builder>,
    monetary_summation_line_total_amount: Option<f64>,
    monetary_summation_charge_total_amount: Option<f64>,
    monetary_summation_allowance_total_amount: Option<f64>,
    specified_trade_payment_terms: Option<SpecifiedTradePaymentTerms<'invoice_builder>>,

    //Basic specification
    included_supply_chain_trade_line_items: Vec<IncludedSupplyChainTradeLineItem<'invoice_builder>>,
}

impl<'invoice_builder> InvoiceBuilder<'invoice_builder> {
    pub fn new() -> Self {
        Self {
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
            monetary_summation: SpecifiedTradeSettlementHeaderMonetarySummation::default(),

            applicable_trade_tax: None,
            sellers_postal_trade_address: PostalTradeAddress::default(),
            buyers_postal_trade_address: PostalTradeAddress::default(),
            monetary_summation_line_total_amount: None,
            monetary_summation_charge_total_amount: None,
            monetary_summation_allowance_total_amount: None,
            specified_trade_payment_terms: None,

            included_supply_chain_trade_line_items: Vec::new(),
        }
    }

    pub fn all_fields_are_set(
        &self,
        specification_level: SpecificationLevel,
    ) -> Result<(), String> {
        let mut error_text: String = String::new();

        // Check fields required for minimum specification
        if self.invoice_type_code.is_none() {
            error_text += "Invoice type code not set\n";
        }
        if self.invoice_nr.is_none() {
            error_text += "Invoice number not set\n";
        }
        if self.date_of_issue.is_none() {
            error_text += "Date of issue not set\n";
        }
        if self.sellers_name.is_none() {
            error_text += "Seller's name not set\n";
        }
        if self.sellers_postal_trade_address.country_id == CountryCode::NotSet {
            error_text += "Seller's postal trade address country code not set\n";
        }
        if self.sellers_specified_tax_registration.is_none() {
            error_text += "Seller's specified tax registration not set\n";
        }
        if self.buyers_name.is_none() {
            error_text += "Buyer's name not set\n";
        }
        if self.buyers_order_specified_document.is_none() {
            error_text += "Buyer's order specified document not set\n";
        }
        if self.monetary_summation.tax_basis_total_amount.is_none() {
            error_text +=
                "Specified trade settlement monetary summation: Tax basis total amount not set\n";
        }
        if self.monetary_summation.tax_total_amount.is_none() {
            error_text +=
                "Specified trade settlement monetary summation: Tax total amount not set\n";
        }
        if self.monetary_summation.grand_total_amount.is_none() {
            error_text +=
                "Specified trade settlement monetary summation: Grand total amount not set\n";
        }
        if self.monetary_summation.due_payable_amount.is_none() {
            error_text +=
                "Specified trade settlement monetary summation: Due payable amount not set\n";
        }

        if self.invoice_currency_code.is_none() {
            error_text += "Invoice currency code not set\n";
        }

        // Additional checks for different specification levels
        if specification_level >= SpecificationLevel::BasicWithoutLines {
            if self.applicable_trade_tax.is_none() {
                error_text += "Applicable trade tax not set\n";
            }

            if self.sellers_postal_trade_address.postcode_code.is_none() {
                error_text += "Sellers postal trade address: Postcode not set\n";
            }

            if self.sellers_postal_trade_address.line_one.is_none() {
                error_text += "Sellers postal trade address: Line one not set\n";
            }

            if self.sellers_postal_trade_address.city_name.is_none() {
                error_text += "Sellers postal trade address: City name not set\n";
            }

            if self.buyers_postal_trade_address.postcode_code.is_none() {
                error_text += "Buyers postal trade address: Postcode not set\n";
            }

            if self.buyers_postal_trade_address.line_one.is_none() {
                error_text += "Buyers postal trade address: Line one not set\n";
            }

            if self.buyers_postal_trade_address.city_name.is_none() {
                error_text += "Buyers postal trade address: City name not set\n";
            }

            if self.occurrence_date.is_none() {
                error_text += "Occurrence date not set\n";
            }

            if let Some(applicable_trade_tax_checker) = self.applicable_trade_tax.as_ref() {
                if applicable_trade_tax_checker.calculated_amount.is_none() {
                    error_text += "Applicable trade tax: Calculated amount not set\n";
                }

                if applicable_trade_tax_checker.basis_amount.is_none() {
                    error_text += "Applicable trade tax: Basis amount not set\n";
                }

                if applicable_trade_tax_checker
                    .rate_applicable_percent
                    .is_none()
                {
                    error_text += "Applicable trade tax: Applicable percent rate not set\n";
                }
            } else {
                error_text += "Applicable trade tax not set\n";
            }

            if self.specified_trade_payment_terms.is_none() {
                error_text += "Specified trade payment terms: Due date time not set\n";
            }

            if self.monetary_summation.line_total_amount.is_none() {
                error_text +=
                    "Specified trade settlement monetary summation: Line total amount not set\n";
            }

            if self.monetary_summation.charge_total_amount.is_none() {
                error_text +=
                    "Specified trade settlement monetary summation: Charge total amount not set\n";
            }

            if self.monetary_summation.allowance_total_amount.is_none() {
                error_text += "Specified trade settlement monetary summation: Allowance total amount not set\n";
            }
        }

        if specification_level >= SpecificationLevel::Basic {
            if self.included_supply_chain_trade_line_items.is_empty() {
                error_text += "No included supply chain trade line items set\n";
            }
        }

        if specification_level >= SpecificationLevel::Extended {
            if self.buyer_reference.is_none() {
                error_text += "Buyer reference not set\n";
            }
            if self.sellers_specified_legal_organization.is_none() {
                error_text += "Seller's specified legal organization not set\n";
            }
            if self.buyers_specified_legal_organization.is_none() {
                error_text += "Buyer's specified legal organization not set\n";
            }
        }

        if !error_text.is_empty() {
            return Err(format!(
                "Errors for specification level {:?}:\n{}",
                specification_level, error_text
            ));
        }

        Ok(())
    }

    // Set functions

    /// Identifies the business process context in which the transaction appears,
    /// to enable the Buyer to process the Invoice in an appropriate way.
    ///
    /// To be specified by the buyer
    ///
    /// The codes to be used are defined in the CHORUSPRO specifications:
    /// A1 (invoice deposit), A2 (prepaid invoice deposit), ...
    pub fn set_business_process<T: Into<&'invoice_builder str>>(
        &mut self,
        business_process: T,
    ) -> &mut Self {
        self.business_process = Some(business_process.into());
        self
    }

    pub fn set_invoice_type_code(&mut self, invoice_type_code: InvoiceTypeCode) -> &mut Self {
        self.invoice_type_code = Some(invoice_type_code);
        self
    }

    pub fn set_invoice_nr<T: Into<&'invoice_builder str>>(&mut self, invoice_nr: T) -> &mut Self {
        self.invoice_nr = Some(invoice_nr.into());
        self
    }

    pub fn set_date_of_issue(&mut self, date_of_issue: NaiveDate) -> &mut Self {
        self.date_of_issue = Some(DateTimeString::new(date_of_issue));
        self
    }

    pub fn set_invoice_notes<S>(&mut self, notes: Vec<S>) -> &mut Self
    where
        S: AsRef<str>,
    {
        self.document_notes = Some(
            notes
                .into_iter()
                .map(|note| IncludedNote {
                    content: note.as_ref().to_string(),
                })
                .collect(),
        );
        self
    }

    /// Buyer reference
    ///
    /// An identifier assigned by the Buyer used for internal routing purposes.
    ///
    /// The identifier is defined by the Buyer (e.g. contact ID,
    /// department, office id, project code), but provided by
    /// the Seller in the Invoice.
    ///
    /// CHORUS PRO: for the public sector, it is the "Service Exécutant". It is mandatory for some buyers. It must
    /// belong to the Chorus Pro repository. It is limited to 100 characters.
    ///
    /// - /rsm:CrossIndustryInvoice
    /// - /rsm:SupplyChainTradeTransaction
    /// - /ram:ApplicableHeaderTradeAgreement
    /// - /ram:BuyerReference
    pub fn set_buyer_reference<T: Into<&'invoice_builder str>>(
        &mut self,
        buyer_reference: T,
    ) -> &mut Self {
        self.buyer_reference = Some(buyer_reference.into());
        self
    }

    pub fn set_sellers_name<T: Into<&'invoice_builder str>>(
        &mut self,
        sellers_name: T,
    ) -> &mut Self {
        self.sellers_name = Some(sellers_name.into());
        self
    }

    /// An identifier issued by an official registrar that identifies the Seller as a legal entity or person.
    ///
    /// BT-30
    ///
    /// ram:SpecifiedLegalOrganization -> ram:ID
    pub fn set_sellers_specified_legal_organization<T: Into<&'invoice_builder str>>(
        &mut self,
        sellers_specified_legal_organization: T,
    ) -> &mut Self {
        self.sellers_specified_legal_organization =
            Some(sellers_specified_legal_organization.into());
        self
    }

    /// Postal code, zip code or similar
    ///
    /// BT-38
    pub fn set_sellers_postal_trade_address_postcode_code<T: Into<&'invoice_builder str>>(
        &mut self,
        postcode_code: T,
    ) -> &mut Self {
        self.sellers_postal_trade_address.postcode_code = Some(postcode_code.into());
        self
    }

    /// Usually streetname and house number or post-office box number
    ///
    /// BT-35
    pub fn set_sellers_postal_trade_address_line_one<T: Into<&'invoice_builder str>>(
        &mut self,
        line: T,
    ) -> &mut Self {
        self.sellers_postal_trade_address.line_one = Some(line.into());
        self
    }

    /// Additional row to specify details or additions to line one
    ///
    /// BT-36
    pub fn set_sellers_postal_trade_address_line_two<T: Into<&'invoice_builder str>>(
        &mut self,
        line: T,
    ) -> &mut Self {
        self.sellers_postal_trade_address.line_two = Some(line.into());
        self
    }

    /// Additional row to specify details or additions to line one
    ///
    /// BT-162
    pub fn set_sellers_postal_trade_address_line_three<T: Into<&'invoice_builder str>>(
        &mut self,
        line: T,
    ) -> &mut Self {
        self.sellers_postal_trade_address.line_three = Some(line.into());
        self
    }

    /// Name of the city or community where the address is located
    ///
    /// BT-37
    pub fn set_sellers_postal_trade_address_city_name<T: Into<&'invoice_builder str>>(
        &mut self,
        city_name: T,
    ) -> &mut Self {
        self.sellers_postal_trade_address.city_name = Some(city_name.into());
        self
    }

    /// If there is no tax agent defined, the country where the sales tax is applied
    ///
    /// BT-40
    pub fn set_sellers_postal_trade_address_country_code(
        &mut self,
        country_code: CountryCode,
    ) -> &mut Self {
        self.sellers_postal_trade_address.country_id = country_code;
        self
    }

    pub fn set_sellers_specified_tax_registration<T: Into<&'invoice_builder str>>(
        &mut self,
        sellers_specified_tax_registration: T,
    ) -> &mut Self {
        self.sellers_specified_tax_registration = Some(sellers_specified_tax_registration.into());
        self
    }

    pub fn set_buyers_name<T: Into<&'invoice_builder str>>(&mut self, buyers_name: T) -> &mut Self {
        self.buyers_name = Some(buyers_name.into());
        self
    }

    /// An identifier issued by an official registrar that identifies the Buyer as a legal entity or person.
    ///
    /// BT-47
    ///
    /// ram:SpecifiedLegalOrganization -> ram:ID
    pub fn set_buyers_specified_legal_organization<T: Into<&'invoice_builder str>>(
        &mut self,
        buyers_specified_legal_organization: T,
    ) -> &mut Self {
        self.buyers_specified_legal_organization = Some(buyers_specified_legal_organization.into());
        self
    }

    /// Postal code, zip code or similar
    ///
    /// BT-53
    pub fn set_buyers_postal_trade_address_postcode_code<T: Into<&'invoice_builder str>>(
        &mut self,
        postcode_code: T,
    ) -> &mut Self {
        self.buyers_postal_trade_address.postcode_code = Some(postcode_code.into());
        self
    }

    /// Usually streetname and house number or post-office box number
    ///
    /// BT-50
    pub fn set_buyers_postal_trade_address_line_one<T: Into<&'invoice_builder str>>(
        &mut self,
        line: T,
    ) -> &mut Self {
        self.buyers_postal_trade_address.line_one = Some(line.into());
        self
    }

    /// Additional row to specify details or additions to line one
    ///
    /// BT-51
    pub fn set_buyers_postal_trade_address_line_two<T: Into<&'invoice_builder str>>(
        &mut self,
        line: T,
    ) -> &mut Self {
        self.buyers_postal_trade_address.line_two = Some(line.into());
        self
    }

    /// Additional row to specify details or additions to line one
    ///
    /// BT-163
    pub fn set_buyers_postal_trade_address_line_three<T: Into<&'invoice_builder str>>(
        &mut self,
        line: T,
    ) -> &mut Self {
        self.buyers_postal_trade_address.line_three = Some(line.into());
        self
    }

    /// Name of the city or community where the address is located
    ///
    /// BT-52
    pub fn set_buyers_postal_trade_address_city_name<T: Into<&'invoice_builder str>>(
        &mut self,
        city_name: T,
    ) -> &mut Self {
        self.buyers_postal_trade_address.city_name = Some(city_name.into());
        self
    }

    /// An identifier issued by an official registrar that identifies the Buyer as a legal entity or person.
    ///
    /// BT-55
    ///
    /// ram:SpecifiedLegalOrganization -> ram:ID
    pub fn set_buyers_postal_trade_address_country_code(
        &mut self,
        country_code: CountryCode,
    ) -> &mut Self {
        self.buyers_postal_trade_address.country_id = country_code;
        self
    }

    /// ## Buyer Order Referenced Document
    /// An identifier of a referenced purchase order, issued by the Buyer.
    ///
    /// /ram:BuyerOrderReferencedDocument
    pub fn set_buyers_order_specified_document<T: Into<&'invoice_builder str>>(
        &mut self,
        buyers_order_specified_document: T,
    ) -> &mut Self {
        self.buyers_order_specified_document = Some(buyers_order_specified_document.into());
        self
    }

    /// Service date, date when the service was delivered
    ///
    /// BT-72
    pub fn set_occurrence_date(&mut self, date: NaiveDate) -> &mut Self {
        self.occurrence_date = Some(DateTimeString::new(date));
        self
    }

    /// Tax amount that needs to be paid.
    /// Calculated by multiplying the net total by the tax percentage
    ///
    /// BT-117
    pub fn set_applicable_trade_tax_calculated_amount(&mut self, amount: f64) -> &mut Self {
        if let Some(applicable_trade_tax) = self.applicable_trade_tax.as_mut() {
            applicable_trade_tax.calculated_amount = Some(amount);
        } else {
            let mut new_struct = ApplicableTradeTax::default();
            new_struct.calculated_amount = Some(amount);
            self.applicable_trade_tax = Some(new_struct);
        }

        self
    }
    /// Sum of all net amounts of a single tax category
    ///
    /// BT-116
    pub fn set_applicable_trade_tax_basis_amount(&mut self, amount: f64) -> &mut Self {
        if let Some(applicable_trade_tax) = self.applicable_trade_tax.as_mut() {
            applicable_trade_tax.basis_amount = Some(amount);
        } else {
            let mut new_struct = ApplicableTradeTax::default();
            new_struct.basis_amount = Some(amount);
            self.applicable_trade_tax = Some(new_struct);
        }

        self
    }

    /// Category of the VAT that is applied. Choose from [`VATCategoryCode`]
    ///
    /// BT-118
    pub fn set_applicable_trade_tax_category_code(&mut self, code: VATCategoryCode) -> &mut Self {
        if let Some(applicable_trade_tax) = self.applicable_trade_tax.as_mut() {
            applicable_trade_tax.category_code = code;
        } else {
            let mut new_struct = ApplicableTradeTax::default();
            new_struct.category_code = code;
            self.applicable_trade_tax = Some(new_struct);
        }

        self
    }
    /// VAT Percentage for the given [`VATCategoryCode`]
    ///
    /// BT-119
    pub fn set_applicable_trade_tax_rate_applicable_percent(&mut self, amount: f64) -> &mut Self {
        if let Some(applicable_trade_tax) = self.applicable_trade_tax.as_mut() {
            applicable_trade_tax.rate_applicable_percent = Some(amount);
        } else {
            let mut new_struct = ApplicableTradeTax::default();
            new_struct.rate_applicable_percent = Some(amount);
            self.applicable_trade_tax = Some(new_struct);
        }

        self
    }

    pub fn set_specified_trade_payment_terms_due_date(&mut self, date: NaiveDate) -> &mut Self {
        let due_date_time = DueDateDateTime {
            payment_due_date: DateTimeString::new(date),
        };
        if let Some(specified_trade_payment_terms) = self.specified_trade_payment_terms.as_mut() {
            specified_trade_payment_terms.due_date_time = Some(due_date_time);
        } else {
            let new_struct = SpecifiedTradePaymentTerms {
                description: None,
                due_date_time: Some(due_date_time),
            };

            self.specified_trade_payment_terms = Some(new_struct);
        }

        self
    }

    pub fn set_invoice_currency_code(&mut self, invoice_currency_code: CurrencyCode) -> &mut Self {
        self.invoice_currency_code = Some(invoice_currency_code);
        self
    }

    /// Sum of all net amounts
    ///
    /// BT-106
    pub fn set_monetary_summation_line_total_amount(&mut self, amount: f64) -> &mut Self {
        self.monetary_summation.line_total_amount = Some(amount);
        self
    }

    /// Surcharge on document level. Surcharges on item level are contained in their net amounts
    ///
    /// BT-108
    pub fn set_monetary_summation_charge_total_amount(&mut self, amount: f64) -> &mut Self {
        self.monetary_summation.charge_total_amount = Some(amount);
        self
    }
    /// Deductions on document level. Deductions on item level are contained in their net amounts
    ///
    /// BT-107
    pub fn set_monetary_summation_allowance_total_amount(&mut self, amount: f64) -> &mut Self {
        self.monetary_summation.allowance_total_amount = Some(amount);
        self
    }

    /// Invoice amoiunt without VAT. Calculated by summing the net amounts
    /// minus document surcharge [Self::set_monetary_summation_charge_total_amount]
    /// plus document deductions [Self::set_monetary_summation_allowance_total_amount]
    ///
    /// BT-109
    pub fn set_monetary_summation_tax_basis_total_amount(&mut self, amount: f64) -> &mut Self {
        self.monetary_summation.tax_basis_total_amount = Some(amount);
        self
    }

    /// Tax amount for the full invoice in the country of the seller, calculated from tax percentage and net sum
    ///
    /// BT-110
    pub fn set_monetary_summation_tax_total_amount(&mut self, amount: f64) -> &mut Self {
        self.monetary_summation.tax_total_amount = Some(TaxTotalAmount::new(
            self.invoice_currency_code
                .expect("Invoice currency not set."),
            amount,
        ));
        self
    }

    /// Gross invoice amount
    ///
    /// [Self::set_monetary_summation_tax_basis_total_amount] + [Self::set_monetary_summation_tax_total_amount]
    ///
    /// BT-112
    pub fn set_monetary_summation_grand_total_amount(&mut self, amount: f64) -> &mut Self {
        self.monetary_summation.grand_total_amount = Some(amount);
        self
    }

    /// Outstanding amount that we ask for with this invoice. Gross invoice amount reduced by any previous payments
    ///
    /// BT-112
    pub fn set_monetary_summation_due_payable_amount(&mut self, amount: f64) -> &mut Self {
        self.monetary_summation.due_payable_amount = Some(amount);
        self
    }

    pub fn add_supply_chain_trade_line_item(
        &mut self,
        line_item: IncludedSupplyChainTradeLineItem<'invoice_builder>,
    ) -> &mut Self {
        self.included_supply_chain_trade_line_items.push(line_item);
        self
    }

    // What the whole crate is actually about
    pub fn to_xml_string(
        mut self,
        specification_level: SpecificationLevel,
    ) -> Result<String, String> {
        let built_invoice = self.build(specification_level)?;

        built_invoice.to_xml_string()
    }

    //Build itself
    pub fn build(
        &mut self,
        specification_level: SpecificationLevel,
    ) -> Result<Invoice<'invoice_builder>, String> {
        //Check if none of the fields is empty
        self.all_fields_are_set(specification_level)?;

        // Build the invoice structure
        Ok(Invoice::new(
            DocumentContext {
                business_process: self.business_process.map(|id| BusinessProcess { id }),
                guideline: Guideline {
                    id: specification_level,
                },
            },
            Document {
                id: self.invoice_nr.unwrap(),
                type_code: self.invoice_type_code.unwrap(),
                issue_date_time: IssueDateTime {
                    date_time_string: self.date_of_issue.clone().unwrap(),
                },
                included_note: Some(self.document_notes.clone().unwrap_or_default()),
            },
            SupplyChainTradeTransaction {
                included_supply_chain_trade_line_items: self
                    .included_supply_chain_trade_line_items
                    .clone(),
                applicable_header_trade_agreement: ApplicableHeaderTradeAgreement {
                    buyer_reference: self.buyer_reference,
                    seller_trade_party: SellerTradeParty {
                        id: Vec::new(),
                        global_id: Vec::new(),
                        name: self.sellers_name.unwrap(),
                        specified_legal_organization: self
                            .sellers_specified_legal_organization
                            .map(|v| SpecifiedLegalOrganization {
                                id: LegalOrganizationID::new(v),
                            }),
                        postal_trade_address: PostalTradeAddress {
                            country_id: self.sellers_postal_trade_address.country_id,
                            postcode_code: self.sellers_postal_trade_address.postcode_code,
                            line_one: self.sellers_postal_trade_address.line_one,
                            line_two: self.sellers_postal_trade_address.line_two,
                            line_three: self.sellers_postal_trade_address.line_three,
                            city_name: self.sellers_postal_trade_address.city_name,
                        },
                        uri_universal_communication: None,
                        specified_tax_registration: vec![SpecifiedTaxRegistration {
                            id: SpecifiedTaxRegistrationID::new(
                                self.sellers_specified_tax_registration.unwrap(),
                            ),
                        }],
                    },
                    buyer_trade_party: BuyerTradeParty {
                        name: self.buyers_name.unwrap(),
                        specified_legal_organization: self
                            .sellers_specified_legal_organization
                            .map(|v| SpecifiedLegalOrganization {
                                id: LegalOrganizationID::new(v),
                            }),
                        postal_trade_address: PostalTradeAddress {
                            country_id: self.buyers_postal_trade_address.country_id,
                            postcode_code: self.buyers_postal_trade_address.postcode_code,
                            line_one: self.buyers_postal_trade_address.line_one,
                            line_two: self.buyers_postal_trade_address.line_two,
                            line_three: self.buyers_postal_trade_address.line_three,
                            city_name: self.buyers_postal_trade_address.city_name,
                        },
                    },
                    buyer_order_referenced_document: self.buyers_order_specified_document.map(
                        |v| BuyerOrderReferencedDocument {
                            issuer_assigned_id: v,
                        },
                    ),
                },
                applicable_header_trade_delivery: ApplicableHeaderTradeDelivery {
                    actual_delivery_supply_chain_event: Some(ActualDeliverySupplyChainEvent {
                        occurrence_date_time: self.occurrence_date.clone().map(
                            |actual_delivery_date| OccurrenceDateTime {
                                actual_delivery_date,
                            },
                        ),
                    }),
                },
                applicable_header_trade_settlement: ApplicableHeaderTradeSettlement {
                    invoice_currency_code: self.invoice_currency_code.clone().unwrap(),
                    specified_trade_settlement_payment_means: Vec::new(),
                    applicable_trade_tax: self.applicable_trade_tax,
                    specified_trade_allowance_charge: Vec::new(),
                    specified_trade_payment_terms: self.specified_trade_payment_terms.clone(),
                    specified_trade_settlement_header_monetary_summation: self
                        .monetary_summation
                        .clone(),
                },
            },
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    /// Tests if a minimum invoice can be built without specifying the legal organization
    fn test_all_fields_are_set_legal_organization() {
        let specification_level = SpecificationLevel::Minimum;

        let sum_net: f64 = 100.0;
        let tax: f64 = sum_net * 19.0 / 100.0;
        let sum_gross: f64 = sum_net + tax;
        let customer_paid_already: f64 = 50.0;

        let mut invoice_builder = InvoiceBuilder::new();

        invoice_builder
            .set_business_process("process1")
            .set_invoice_type_code(InvoiceTypeCode::CommercialInvoice)
            .set_invoice_nr("INV-123456")
            .set_date_of_issue(chrono::NaiveDate::from_ymd_opt(2024, 08, 10).unwrap())
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
}
