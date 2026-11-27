use super::InvoiceBuilder;
use crate::components::enums::currency_code::CurrencyCode;
use crate::components::structs::{
    ApplicableTradeSettlementFinancialCard, PayeePartyCreditorFinancialAccount,
    PayeePartyCreditorFinancialAccountInfo, PayeeSpecifiedCreditorFinancialInstitution,
    TradeAllowanceCharge, TaxPoint, SpecifiedTradeAllowanceCharge, TaxTotalAmount,
};

impl<'invoice_builder> InvoiceBuilder<'invoice_builder> {
    pub fn set_accounting_currency_tax(&mut self, tax: TaxTotalAmount) -> &mut Self {
        self.accounting_currency_tax = Some(tax);
        self
    }

    pub fn set_tax_point(&mut self, tax_point: TaxPoint<'invoice_builder>) -> &mut Self {
        self.tax_point = Some(tax_point);
        self
    }

    pub fn add_document_charge(&mut self, charge: TradeAllowanceCharge<'invoice_builder>) -> &mut Self {
        self.document_charges.push(charge);
        self
    }

    pub fn set_invoice_currency_code(&mut self, invoice_currency_code: CurrencyCode) -> &mut Self {
        self.invoice_currency_code = Some(invoice_currency_code);
        self
    }

    /// Adds a document-level allowance or charge (BG-20 / BG-21).
    pub fn add_specified_trade_allowance_charge(
        &mut self,
        allowance_charge: SpecifiedTradeAllowanceCharge<'invoice_builder>,
    ) -> &mut Self {
        self.specified_trade_allowance_charge.push(allowance_charge);
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
        let card = self.applicable_settlement_financial_card.get_or_insert(
            ApplicableTradeSettlementFinancialCard {
                id: card_id,
                cardholder_name: None,
            },
        );
        card.id = card_id;
        self
    }

    /// Sets the optional cardholder name; the card identifier is still required.
    pub fn set_applicable_settlement_financial_card_cardholder_name(
        &mut self,
        cardholder_name: &'invoice_builder str,
    ) -> &mut Self {
        let card = self.applicable_settlement_financial_card.get_or_insert(
            ApplicableTradeSettlementFinancialCard {
                id: "",
                cardholder_name: None,
            },
        );
        card.cardholder_name = Some(cardholder_name);
        self
    }       

    pub fn set_payer_financial_account_ibanid(
        &mut self,
        ibanid: &'invoice_builder str,
    ) -> &mut Self {
        self.payer_financial_account_ibanid = Some(ibanid);
        self
    }

    pub fn set_payee_account_ibanid(
        &mut self,
        ibanid: &'invoice_builder str,
    ) -> &mut Self {
        let info = self
            .payee_party_creditor_financial_account_info
            .get_or_insert(PayeePartyCreditorFinancialAccountInfo {
                account: PayeePartyCreditorFinancialAccount {
                    ibanid: None,
                    account_name: None,
                    proprietary_id: None,
                },
                institution: None,
            });
        info.account.ibanid = Some(ibanid);
        self
    }

    pub fn set_payee_creditor_account_name(
        &mut self,
        account_name: &'invoice_builder str,
    ) -> &mut Self {
        let info = self
            .payee_party_creditor_financial_account_info
            .get_or_insert(PayeePartyCreditorFinancialAccountInfo {
                account: PayeePartyCreditorFinancialAccount {
                    ibanid: None,
                    account_name: None,
                    proprietary_id: None,
                },
                institution: None,
            });
        info.account.account_name = Some(account_name);
        self
    }

    pub fn set_payee_financial_account_proprietary_id(
        &mut self,
        proprietary_id: &'invoice_builder str,
    ) -> &mut Self {
        let info = self
            .payee_party_creditor_financial_account_info
            .get_or_insert(PayeePartyCreditorFinancialAccountInfo {
                account: PayeePartyCreditorFinancialAccount {
                    ibanid: None,
                    account_name: None,
                    proprietary_id: None,
                },
                institution: None,
            });
        info.account.proprietary_id = Some(proprietary_id);
        self
    }

    pub fn set_payee_creditor_bic_id(
        &mut self,
        bicid: &'invoice_builder str,
    ) -> &mut Self {
        let info = self
            .payee_party_creditor_financial_account_info
            .get_or_insert(PayeePartyCreditorFinancialAccountInfo {
                account: PayeePartyCreditorFinancialAccount {
                    ibanid: None,
                    account_name: None,
                    proprietary_id: None,
                },
                institution: None,
            });
        info.institution = Some(PayeeSpecifiedCreditorFinancialInstitution { bicid });
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
