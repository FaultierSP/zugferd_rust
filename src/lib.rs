use quick_xml;
use chrono::NaiveDate;
use serde::Serialize;

pub mod components;

pub use components::structs::*;
pub use crate::components::enums::{
    specification_level::SpecificationLevel,
    invoice_type_code::InvoiceTypeCode,
    country_code::CountryCode,
    currency_code::CurrencyCode,
    vat_category_code::VATCategoryCode,
};
pub use components::business_rules::validate as validate_business_rules;

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

impl<'invoice_builder> InvoiceBuilder <'invoice_builder> {
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
        if self.sellers_postal_trade_address.country_id == CountryCode::NotSet {
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
        if self.monetary_summation.tax_basis_total_amount.is_none() {
            error_text += "Specified trade settlement monetary summation: Tax basis total amount not set\n";
        }
        if self.monetary_summation.tax_total_amount.is_none() {
            error_text += "Specified trade settlement monetary summation: Tax total amount not set\n";
        }
        if self.monetary_summation.grand_total_amount.is_none() {
            error_text += "Specified trade settlement monetary summation: Grand total amount not set\n";
        }
        if self.monetary_summation.due_payable_amount.is_none() {
            error_text += "Specified trade settlement monetary summation: Due payable amount not set\n";
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

                if applicable_trade_tax_checker.rate_applicable_percent.is_none() {
                    error_text += "Applicable trade tax: Applicable percent rate not set\n";
                }

            }
            else {
                error_text += "Applicable trade tax not set\n";
            }

            if self.specified_trade_payment_terms.is_none() {
                error_text += "Specified trade payment terms: Due date time not set\n";
            }

            if self.monetary_summation.line_total_amount.is_none() {
                error_text += "Specified trade settlement monetary summation: Line total amount not set\n";
            }

            if self.monetary_summation.charge_total_amount.is_none() {
                error_text += "Specified trade settlement monetary summation: Charge total amount not set\n";
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

    pub fn set_invoice_notes <S> (&mut self, notes: Vec<S>) -> &mut Self
    where S: AsRef<str>
    {
        self.document_notes = Some(
            notes.into_iter().map(|note| IncludedNote {
                content: note.as_ref().to_string(),
            }).collect()
        );
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

    pub fn set_sellers_postal_trade_address_postcode_code <T:Into<&'invoice_builder str>> (&mut self, postcode_code:T) -> &mut Self {
        self.sellers_postal_trade_address.postcode_code = Some(postcode_code.into());
        self
    }

    pub fn set_sellers_postal_trade_address_line_one <T:Into<&'invoice_builder str>> (&mut self, line:T) -> &mut Self {
        self.sellers_postal_trade_address.line_one = Some(line.into());
        self
    }

    pub fn set_sellers_postal_trade_address_line_two <T:Into<&'invoice_builder str>> (&mut self, line:T) -> &mut Self {
        self.sellers_postal_trade_address.line_two = Some(line.into());
        self
    }

    pub fn set_sellers_postal_trade_address_line_three <T:Into<&'invoice_builder str>> (&mut self, line:T) -> &mut Self {
        self.sellers_postal_trade_address.line_three = Some(line.into());
        self
    }

    pub fn set_sellers_postal_trade_address_city_name <T:Into<&'invoice_builder str>> (&mut self, city_name:T) -> &mut Self {
        self.sellers_postal_trade_address.city_name = Some(city_name.into());
        self
    }

    pub fn set_sellers_postal_trade_address_country_code (&mut self, country_code: CountryCode) -> &mut Self {
        self.sellers_postal_trade_address.country_id = country_code;
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

    pub fn set_buyers_postal_trade_address_postcode_code <T:Into<&'invoice_builder str>> (&mut self, postcode_code:T) -> &mut Self {
        self.buyers_postal_trade_address.postcode_code = Some(postcode_code.into());
        self
    }

    pub fn set_buyers_postal_trade_address_line_one <T:Into<&'invoice_builder str>> (&mut self, line:T) -> &mut Self {
        self.buyers_postal_trade_address.line_one = Some(line.into());
        self
    }

    pub fn set_buyers_postal_trade_address_line_two <T:Into<&'invoice_builder str>> (&mut self, line:T) -> &mut Self {
        self.buyers_postal_trade_address.line_two = Some(line.into());
        self
    }

    pub fn set_buyers_postal_trade_address_line_three <T:Into<&'invoice_builder str>> (&mut self, line:T) -> &mut Self {
        self.buyers_postal_trade_address.line_three = Some(line.into());
        self
    }

    pub fn set_buyers_postal_trade_address_city_name <T:Into<&'invoice_builder str>> (&mut self, city_name:T) -> &mut Self {
        self.buyers_postal_trade_address.city_name = Some(city_name.into());
        self
    }

    pub fn set_buyers_postal_trade_address_country_code (&mut self, country_code: CountryCode) -> &mut Self {
        self.buyers_postal_trade_address.country_id = country_code;
        self
    }

    pub fn set_buyers_order_specified_document <T: Into<&'invoice_builder str>> (&mut self, buyers_order_specified_document: T) -> &mut Self {
        self.buyers_order_specified_document = Some(buyers_order_specified_document.into());
        self
    }

    pub fn set_occurrence_date (&mut self, date: NaiveDate) -> &mut Self {
        self.occurrence_date = Some(DateTimeString::new(date));
        self
    }

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
        if let Some(specified_trade_payment_terms) = self.specified_trade_payment_terms.as_mut() {
            specified_trade_payment_terms.due_date_time = DateTimeString::new(date);
        }
        else {
            let new_struct = SpecifiedTradePaymentTerms{
                due_date_time: DateTimeString::new(date),
            };

            self.specified_trade_payment_terms = Some(new_struct);
        }
        
        self
    }

    pub fn set_invoice_currency_code(&mut self, invoice_currency_code: CurrencyCode) -> &mut Self {
        self.invoice_currency_code = Some(invoice_currency_code);
        self
    }

    pub fn set_monetary_summation_line_total_amount(&mut self, amount: f64) -> &mut Self {
        self.monetary_summation.line_total_amount = Some(amount);
        self
    }

    pub fn set_monetary_summation_charge_total_amount(&mut self, amount: f64) -> &mut Self {
        self.monetary_summation.charge_total_amount = Some(amount);
        self
    }

    pub fn set_monetary_summation_allowance_total_amount(&mut self, amount: f64) -> &mut Self {
        self.monetary_summation.allowance_total_amount = Some(amount);
        self
    }

    pub fn set_monetary_summation_tax_basis_total_amount(&mut self, amount: f64) -> &mut Self {
        self.monetary_summation.tax_basis_total_amount = Some(amount);
        self
    }

    pub fn set_monetary_summation_tax_total_amount(&mut self, amount: f64) -> &mut Self {
        self.monetary_summation.tax_total_amount = Some(TaxTotalAmount::new(self.invoice_currency_code.expect("Invoice currency not set."), amount));
        self
    }

    pub fn set_monetary_summation_grand_total_amount(&mut self, amount: f64) -> &mut Self {
        self.monetary_summation.grand_total_amount = Some(amount);
        self
    }

    pub fn set_monetary_summation_due_payable_amount(&mut self, amount: f64) -> &mut Self {
        self.monetary_summation.due_payable_amount = Some(amount);
        self
    }

    pub fn add_supply_chain_trade_line_item(&mut self, line_item: IncludedSupplyChainTradeLineItem<'invoice_builder>) -> &mut Self {
        self.included_supply_chain_trade_line_items.push(line_item);
        self
    }

    // What the whole crate is actually about
    pub fn to_xml_string(mut self,specification_level: SpecificationLevel) -> Result<String,String> {
        let built_invoice = self.build(specification_level)?;

        match quick_xml::se::to_string(&built_invoice) {
          Ok(xml_string) => Ok(
            format!(
              "<?xml version='1.0' encoding='UTF-8'?>{}",
              xml_string
            )
          ),
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
                included_note: Some(self.document_notes.clone().unwrap_or_default()),
            },
            SupplyChainTradeTransaction {
                included_supply_chain_trade_line_items: self.included_supply_chain_trade_line_items.clone(),
                applicable_header_trade_agreement: ApplicableHeaderTradeAgreement {
                    buyer_reference: self.buyer_reference,
                    seller_trade_party: SellerTradeParty {
                        name: self.sellers_name.unwrap(),
                        specified_legal_organization: SpecifiedLegalOrganization {
                            id: LegalOrganizationID::new(self.sellers_specified_legal_organization.unwrap()),
                        },
                        postal_trade_address: PostalTradeAddress {
                            country_id: self.sellers_postal_trade_address.country_id,
                            postcode_code: self.sellers_postal_trade_address.postcode_code,
                            line_one: self.sellers_postal_trade_address.line_one,
                            line_two: self.sellers_postal_trade_address.line_two,
                            line_three: self.sellers_postal_trade_address.line_three,
                            city_name: self.sellers_postal_trade_address.city_name,
                        },
                        specified_tax_registration: SpecifiedTaxRegistration {
                            id: SpecifiedTaxRegistrationID::new(self.sellers_specified_tax_registration.unwrap()),
                        },
                    },
                    buyer_trade_party: BuyerTradeParty {
                        name: self.buyers_name.unwrap(),
                        specified_legal_organization: Some(SpecifiedLegalOrganization {
                            id: LegalOrganizationID::new(self.buyers_specified_legal_organization.unwrap()),
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
                    buyer_order_referenced_document: BuyerOrderReferencedDocument {
                        issuer_assigned_id: self.buyers_order_specified_document.unwrap(),
                    },
                },
                applicable_header_trade_delivery: ApplicableHeaderTradeDelivery {
                    actual_delivery_supply_chain_event: Some(ActualDeliverySupplyChainEvent {
                        occurrence_date_time: self.occurrence_date.clone().map(|actual_delivery_date|
                            OccurrenceDateTime {
                                actual_delivery_date,
                            }
                        ),
                    }),
                },
                applicable_header_trade_settlement: ApplicableHeaderTradeSettlement {
                    invoice_currency_code: self.invoice_currency_code.clone().unwrap(),
                    applicable_trade_tax: self.applicable_trade_tax,
                    specified_trade_payment_terms: self.specified_trade_payment_terms.clone(),
                    specified_trade_settlement_header_monetary_summation: self.monetary_summation.clone(),
                },
            },
        ))
    }
}