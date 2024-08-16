use serde_xml_rs;
use chrono::NaiveDate;
use serde::Serialize;

pub mod components;

pub use components::structs::{
    Invoice,
    DocumentContext,
    BusinessProcess,
    Guideline,
    Document,
    IssueDateTime,
    DateTimeString,
    SupplyChainTradeTransaction,
    ApplicableHeaderTradeAgreement,
    ApplicableHeaderTradeDelivery,
    ApplicableHeaderTradeSettlement,
    SellerTradeParty,
    BuyerTradeParty,
    BuyerOrderReferencedDocument,
    SpecifiedLegalOrganization,
    LegalOrganizationID,
    PostalTradeAddress,
    SpecifiedTaxRegistration,
    SpecifiedTaxRegistrationID,
    SpecifiedTradeSettlementHeaderMonetarySummation,
    TaxTotalAmount,
};
pub use crate::components::enums::{
    specification_level::SpecificationLevel,
    invoice_type_code::InvoiceTypeCode,
    country_code::CountryCode,
    currency_code::CurrencyCode,
};

#[derive(Serialize, Clone)]
pub struct InvoiceBuilder<'invoice_builder> {
    //Minimal specification
    business_process: Option<&'invoice_builder str>,
    invoice_type_code: Option<InvoiceTypeCode>,
    invoice_nr: Option<&'invoice_builder str>,
    date_of_issue: Option<DateTimeString<'invoice_builder>>,
    buyer_reference: Option<&'invoice_builder str>,
    sellers_name: Option<&'invoice_builder str>,
    sellers_specified_legal_organization: Option<&'invoice_builder str>,
    sellers_postal_trade_address_country_code: Option<CountryCode>,
    sellers_specified_tax_registration: Option<&'invoice_builder str>,
    buyers_name: Option<&'invoice_builder str>,
    buyers_specified_legal_organization: Option<&'invoice_builder str>,
    buyers_order_specified_document: Option<&'invoice_builder str>,
    invoice_currency_code: Option<CurrencyCode>,
    tax_basis_total_amount: Option<f64>,
    tax_basis_total_amount_str: &'invoice_builder str,
    tax_total_amount: Option<f64>,
    grand_total_amount: Option<f64>,
    due_payable_amount: Option<f64>,
}

impl<'invoice_builder> InvoiceBuilder <'invoice_builder> {
    pub fn new() -> Self {
        Self {
            business_process: None,
            invoice_type_code: None,
            invoice_nr: None,
            date_of_issue: None,
            buyer_reference: None,
            sellers_name: None,
            sellers_specified_legal_organization: None,
            sellers_postal_trade_address_country_code: None,
            sellers_specified_tax_registration: None,
            buyers_name: None,
            buyers_specified_legal_organization: None,
            buyers_order_specified_document: None,
            invoice_currency_code: None,
            tax_basis_total_amount: None,
            tax_basis_total_amount_str: "",
            tax_total_amount: None,
            grand_total_amount: None,
            due_payable_amount: None,
        }
    }

    pub fn all_fields_are_set(&self, specification_level: SpecificationLevel) -> Result<(), String> {
        let mut error_text: String = String::new();

        // Check fields required for minimum specification
        if self.business_process.is_none() {
            error_text += "Business process variable not set\n";
        }
        if self.invoice_type_code.is_none() {
            error_text += "Invoice type code not set\n";
        }
        if self.invoice_nr.is_none() {
            error_text += "Invoice number not set\n";
        }
        if self.date_of_issue.is_none() {
            error_text += "Date of issue not set\n";
        }
        if self.buyer_reference.is_none() {
            error_text += "Buyer reference not set\n";
        }
        if self.sellers_name.is_none() {
            error_text += "Seller's name not set\n";
        }
        if self.sellers_specified_legal_organization.is_none() {
            error_text += "Seller's specified legal organization not set\n";
        }
        if self.sellers_postal_trade_address_country_code.is_none() {
            error_text += "Seller's postal trade address country code not set\n";
        }
        if self.sellers_specified_tax_registration.is_none() {
            error_text += "Seller's specified tax registration not set\n";
        }
        if self.buyers_name.is_none() {
            error_text += "Buyer's name not set\n";
        }
        if self.buyers_specified_legal_organization.is_none() {
            error_text += "Buyer's specified legal organization not set\n";
        }
        if self.buyers_order_specified_document.is_none() {
            error_text += "Buyer's order specified document not set\n";
        }
        if self.invoice_currency_code.is_none() {
            error_text += "Invoice currency code not set\n";
        }
        if self.tax_basis_total_amount.is_none() {
            error_text += "Tax basis total amount not set\n";
        }
        if self.tax_total_amount.is_none() {
            error_text += "Tax total amount not set\n";
        }
        if self.grand_total_amount.is_none() {
            error_text += "Grand total amount not set\n";
        }
        if self.due_payable_amount.is_none() {
            error_text += "Due payable amount not set\n";
        }

        // Additional checks for different specification levels will be added here
        match specification_level {
            SpecificationLevel::Minimum => {
                // Minimum specification checks are already included above
            },
            SpecificationLevel::Basic => {

            },
            SpecificationLevel::BasicWithLess => {

            },
            SpecificationLevel::En16931 => {

            },
            SpecificationLevel::Extended => {

            }
        }

        if !error_text.is_empty() {
            return Err(format!("Errors for specification level {:?}:\n{}", specification_level, error_text));
        }

        Ok(())
    }

    // Set functions
    pub fn set_business_process <T: Into<&'invoice_builder str>> (&mut self, business_process: T) -> &mut Self {
        self.business_process = Some(business_process.into());
        self
    }

    pub fn set_invoice_type_code(&mut self, invoice_type_code: InvoiceTypeCode) -> &mut Self {
        self.invoice_type_code = Some(invoice_type_code);
        self
    }

    pub fn set_invoice_nr <T: Into<&'invoice_builder str>> (&mut self, invoice_nr: T) -> &mut Self {
        self.invoice_nr = Some(invoice_nr.into());
        self
    }

    pub fn set_date_of_issue (&mut self, date_of_issue: NaiveDate) -> &mut Self {
        self.date_of_issue = Some(DateTimeString::new(date_of_issue));
        self
    }

    pub fn set_buyer_reference <T: Into<&'invoice_builder str>> (&mut self, buyer_reference: T) -> &mut Self {
        self.buyer_reference = Some(buyer_reference.into());
        self
    }

    pub fn set_sellers_name <T: Into<&'invoice_builder str>> (&mut self, sellers_name: T) -> &mut Self {
        self.sellers_name = Some(sellers_name.into());
        self
    }

    pub fn set_sellers_specified_legal_organization <T: Into<&'invoice_builder str>> (&mut self, sellers_specified_legal_organization: T) -> &mut Self {
        self.sellers_specified_legal_organization = Some(sellers_specified_legal_organization.into());
        self
    }

    pub fn set_sellers_postal_trade_address_country_code (&mut self, sellers_postal_trade_address_country_code: CountryCode) -> &mut Self {
        self.sellers_postal_trade_address_country_code = Some(sellers_postal_trade_address_country_code);
        self
    }

    pub fn set_sellers_specified_tax_registration <T: Into<&'invoice_builder str>> (&mut self, sellers_specified_tax_registration: T) -> &mut Self {
        self.sellers_specified_tax_registration = Some(sellers_specified_tax_registration.into());
        self
    }

    pub fn set_buyers_name <T: Into<&'invoice_builder str>> (&mut self, buyers_name: T) -> &mut Self {
        self.buyers_name = Some(buyers_name.into());
        self
    }

    pub fn set_buyers_specified_legal_organization <T: Into<&'invoice_builder str>> (&mut self, buyers_specified_legal_organization: T) -> &mut Self {
        self.buyers_specified_legal_organization = Some(buyers_specified_legal_organization.into());
        self
    }

    pub fn set_buyers_order_specified_document <T: Into<&'invoice_builder str>> (&mut self, buyers_order_specified_document: T) -> &mut Self {
        self.buyers_order_specified_document = Some(buyers_order_specified_document.into());
        self
    }

    pub fn set_invoice_currency_code(&mut self, invoice_currency_code: CurrencyCode) -> &mut Self {
        self.invoice_currency_code = Some(invoice_currency_code);
        self
    }

    pub fn set_tax_basis_total_amount(&mut self, tax_basis_total_amount: f64) -> &mut Self {
        self.tax_basis_total_amount = Some(tax_basis_total_amount);
        self
    }

    pub fn set_tax_total_amount(&mut self, tax_total_amount: f64) -> &mut Self {
        self.tax_total_amount = Some(tax_total_amount);
        self
    }

    pub fn set_grand_total_amount(&mut self, grand_total_amount: f64) -> &mut Self {
        self.grand_total_amount = Some(grand_total_amount);
        self
    }

    pub fn set_due_payable_amount(&mut self, due_payable_amount: f64) -> &mut Self {
        self.due_payable_amount = Some(due_payable_amount);
        self
    }

    pub fn to_xml_string(mut self,specification_level: SpecificationLevel) -> Result<String,String> {
        let built_invoice = self.build(specification_level)?;

        match serde_xml_rs::to_string(&built_invoice) {
          Ok(xml_string) => Ok(xml_string),
          Err(e) => Err(e.to_string()),  
        }
    }

    //Build itself
    pub fn build(&mut self, specification_level: SpecificationLevel) -> Result<Invoice<'invoice_builder>, String> {
        //Check if none of the fields is empty
        self.all_fields_are_set(specification_level)?;

        // Build the invoice structure
        Ok(Invoice::new(
            DocumentContext {
                business_process: BusinessProcess { id: self.business_process.unwrap() },
                guideline: Guideline { id: specification_level },
            },
            Document {
                id: self.invoice_nr.unwrap(),
                type_code: self.invoice_type_code.unwrap(),
                issue_date_time: IssueDateTime {
                    date_time_string: self.date_of_issue.clone().unwrap(),
                },
            },
            SupplyChainTradeTransaction {
                applicable_header_trade_agreement: ApplicableHeaderTradeAgreement {
                    buyer_reference: self.buyer_reference.unwrap(),
                    seller_trade_party: SellerTradeParty {
                        name: self.sellers_name.unwrap(),
                        specified_legal_organization: SpecifiedLegalOrganization {
                            id: LegalOrganizationID::new(self.sellers_specified_legal_organization.unwrap()),
                        },
                        postal_trade_address: PostalTradeAddress {
                            country_id: self.sellers_postal_trade_address_country_code.unwrap(),
                        },
                        specified_tax_registration: SpecifiedTaxRegistration {
                            id: SpecifiedTaxRegistrationID::new(self.sellers_specified_tax_registration.unwrap()),
                        },
                    },
                    buyer_trade_party: BuyerTradeParty {
                        name: self.buyers_name.unwrap(),
                        specified_legal_organization: SpecifiedLegalOrganization {
                            id: LegalOrganizationID::new(self.buyers_specified_legal_organization.unwrap()),
                        },
                    },
                    buyer_order_referenced_document: BuyerOrderReferencedDocument {
                        issuer_assigned_id: self.buyers_order_specified_document.unwrap(),
                    },
                },
                applicable_header_trade_delivery: ApplicableHeaderTradeDelivery {},
                applicable_header_trade_settlement: ApplicableHeaderTradeSettlement {
                    invoice_currency_code: self.invoice_currency_code.clone().unwrap(),
                    specified_trade_settlement_header_monetary_summation: SpecifiedTradeSettlementHeaderMonetarySummation {
                        tax_basis_total_amount: self.tax_basis_total_amount.unwrap(),
                        tax_total_amount: TaxTotalAmount::new(
                            self.invoice_currency_code.unwrap(),
                            self.tax_total_amount.unwrap(),
                        ),
                        grand_total_amount: self.grand_total_amount.unwrap(),
                        due_payable_amount: self.due_payable_amount.unwrap(),
                    },
                },
            },
        ))
    }
}