use super::InvoiceBuilder;
use crate::components::enums::{
    country_code::CountryCode, specification_level::SpecificationLevel,
};

impl<'invoice_builder> InvoiceBuilder<'invoice_builder> {
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
        if self.tax_total_amount.is_none() {
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

        if specification_level >= SpecificationLevel::Basic
            && self.included_supply_chain_trade_line_items.is_empty()
        {
            error_text += "No included supply chain trade line items set\n";
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
}
