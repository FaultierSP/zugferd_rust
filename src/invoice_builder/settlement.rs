use super::InvoiceBuilder;
use crate::components::enums::currency_code::CurrencyCode;

impl<'invoice_builder> InvoiceBuilder<'invoice_builder> {
    pub fn set_invoice_currency_code(&mut self, invoice_currency_code: CurrencyCode) -> &mut Self {
        self.invoice_currency_code = Some(invoice_currency_code);
        self
    }

    // payments

    // type_code gemaess Codeliste UNTDID 4461
    pub fn set_specified_trade_settlement_payment_means_type_code(
        &mut self,
        type_code: &'invoice_builder str,
    ) -> &mut Self {
        self.specified_trade_settlement_payment_means_type_code = Some(type_code);
        self
    }
    
    pub fn set_specified_trade_settlement_payment_means_information(
        &mut self,
        information: &'invoice_builder str,
    ) -> &mut Self {
        self.specified_trade_settlement_payment_means_information = Some(information);
        self
    }

    /// Sets the required card identifier when card payment details are provided.
    pub fn set_applicable_settlement_financial_card_id(
        &mut self,
        card_id: &'invoice_builder str,
    ) -> &mut Self {
        self.applicable_settlement_financial_card_id = Some(card_id);
        self
    }

    /// Sets the optional cardholder name; the card identifier is still required.
    pub fn set_applicable_settlement_financial_card_cardholder_name(
        &mut self,
        cardholder_name: &'invoice_builder str,
    ) -> &mut Self {
        self.applicable_settlement_financial_card_cardholder_name = Some(cardholder_name);
        self
    }       


    pub fn set_payer_party_debtor_financial_account_ibanid(
        &mut self,
        ibanid: &'invoice_builder str,
    ) -> &mut Self {
        self.payer_party_debtor_financial_account_ibanid = Some(ibanid);
        self
    }

    pub fn set_payer_payee_creditor_financial_account_ibanid(
        &mut self,
        ibanid: &'invoice_builder str,
    ) -> &mut Self {
        self.payer_payee_creditor_financial_account_ibanid = Some(ibanid);
        self
    }

    pub fn set_payee_party_creditor_financial_account_name(
        &mut self,
        account_name: &'invoice_builder str,
    ) -> &mut Self {
        self.payee_party_creditor_financial_account_name = Some(account_name);
        self
    }

    pub fn set_payee_party_creditor_financial_account_proprietary_id(
        &mut self,
        proprietary_id: &'invoice_builder str,
    ) -> &mut Self {
        self.payee_party_creditor_financial_account_proprietary_id = Some(proprietary_id);
        self
    }

    pub fn set_payee_party_creditor_financial_account_payee_specified_creditor_financial_institution_bicid(
        &mut self,
        bicid: &'invoice_builder str,
    ) -> &mut Self {
        self.payee_specified_creditor_financial_institution_bicid = Some(bicid);
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
