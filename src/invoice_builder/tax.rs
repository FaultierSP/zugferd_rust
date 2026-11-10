use super::InvoiceBuilder;
use crate::components::enums::vat_category_code::VATCategoryCode;
use crate::components::structs::{
    ApplicableTradeTax, DateTimeString, DueDateDateTime, SpecifiedTradePaymentTerms,
};
use chrono::NaiveDate;

impl<'invoice_builder> InvoiceBuilder<'invoice_builder> {
    pub fn add_applicable_trade_tax(
        &mut self,
        applicable_trade_tax: ApplicableTradeTax<'invoice_builder>,
    ) -> &mut Self {
        self.applicable_trade_tax.push(applicable_trade_tax);
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
        self.first_applicable_trade_tax_mut().calculated_amount = Some(amount);
        self
    }

    /// Sum of all net amounts of a single tax category
    ///
    /// BT-116
    pub fn set_applicable_trade_tax_basis_amount(&mut self, amount: f64) -> &mut Self {
        self.first_applicable_trade_tax_mut().basis_amount = Some(amount);
        self
    }

    /// Category of the VAT that is applied. Choose from [`VATCategoryCode`]
    ///
    /// BT-118
    pub fn set_applicable_trade_tax_category_code(&mut self, code: VATCategoryCode) -> &mut Self {
        self.first_applicable_trade_tax_mut().category_code = code;
        self
    }

    /// VAT Percentage for the given [`VATCategoryCode`]
    ///
    /// BT-119
    pub fn set_applicable_trade_tax_rate_applicable_percent(&mut self, amount: f64) -> &mut Self {
        self.first_applicable_trade_tax_mut().rate_applicable_percent = Some(amount);
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

    fn first_applicable_trade_tax_mut(&mut self) -> &mut ApplicableTradeTax<'invoice_builder> {
        if self.applicable_trade_tax.is_empty() {
            self.applicable_trade_tax
                .push(ApplicableTradeTax::default());
        }
        &mut self.applicable_trade_tax[0]
    }
}
