use super::InvoiceBuilder;
use crate::components::enums::currency_code::CurrencyCode;

impl<'invoice_builder> InvoiceBuilder<'invoice_builder> {
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
        self.tax_total_amount = Some(amount);
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
}
