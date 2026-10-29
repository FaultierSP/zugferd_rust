use super::InvoiceBuilder;
use crate::components::enums::vat_category_code::VATCategoryCode;
use crate::components::structs::{
    ApplicableTradeTax, DateTimeString, DueDateDateTime, SpecifiedTradePaymentTerms,
};
use chrono::NaiveDate;

impl<'invoice_builder> InvoiceBuilder<'invoice_builder> {
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
            self.applicable_trade_tax = Some(ApplicableTradeTax {
                calculated_amount: Some(amount),
                ..Default::default()
            });
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
            self.applicable_trade_tax = Some(ApplicableTradeTax {
                basis_amount: Some(amount),
                ..Default::default()
            });
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
            self.applicable_trade_tax = Some(ApplicableTradeTax {
                category_code: code,
                ..Default::default()
            });
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
            self.applicable_trade_tax = Some(ApplicableTradeTax {
                rate_applicable_percent: Some(amount),
                ..Default::default()
            });
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
}
