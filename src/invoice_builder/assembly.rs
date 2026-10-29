use super::{InvoiceBuilder, InvoiceBuilderError};
use crate::components::enums::specification_level::SpecificationLevel;
use crate::components::structs::*;

impl<'invoice_builder> InvoiceBuilder<'invoice_builder> {
    // What the whole crate is actually about
    pub fn to_xml_string(
        mut self,
        specification_level: SpecificationLevel,
    ) -> Result<String, String> {
        let built_invoice = self.build(specification_level).map_err(|e| e.to_string())?;

        built_invoice.to_xml_string()
    }

    //Build itself
    pub fn build(
        &mut self,
        specification_level: SpecificationLevel,
    ) -> Result<Invoice<'invoice_builder>, String> {
        //Check if none of the fields is empty
        self.all_fields_are_set(specification_level)?;

        let invoice_currency_code = self
            .invoice_currency_code
            .ok_or_else(|| InvoiceBuilderError::MissingField("invoice_currency_code".to_string()))
            .map_err(|error| error.to_string())?;
        let tax_total_amount = self
            .tax_total_amount
            .ok_or_else(|| InvoiceBuilderError::MissingField("tax_total_amount".to_string()))
            .map_err(|error| error.to_string())?;
        let mut monetary_summation = self.monetary_summation.clone();
        monetary_summation.tax_total_amount =
            Some(TaxTotalAmount::new(invoice_currency_code, tax_total_amount));

        // Build the invoice structure
        Ok(Invoice::new(
            DocumentContext {
                business_process: self.business_process.map(|id| BusinessProcess { id }),
                guideline: Guideline {
                    id: specification_level,
                },
            },
            Document {
                id: self
                    .invoice_nr
                    .ok_or_else(|| InvoiceBuilderError::MissingField("invoice_nr".to_string()))
                    .map_err(|error| error.to_string())?,
                type_code: self
                    .invoice_type_code
                    .ok_or_else(|| {
                        InvoiceBuilderError::MissingField("invoice_type_code".to_string())
                    })
                    .map_err(|error| error.to_string())?,
                issue_date_time: IssueDateTime {
                    date_time_string: self
                        .date_of_issue
                        .clone()
                        .ok_or_else(|| {
                            InvoiceBuilderError::MissingField("date_of_issue".to_string())
                        })
                        .map_err(|error| error.to_string())?,
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
                        name: self
                            .sellers_name
                            .ok_or_else(|| {
                                InvoiceBuilderError::MissingField("sellers_name".to_string())
                            })
                            .map_err(|error| error.to_string())?,
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
                                self.sellers_specified_tax_registration
                                    .ok_or_else(|| {
                                        InvoiceBuilderError::MissingField(
                                            "sellers_specified_tax_registration".to_string(),
                                        )
                                    })
                                    .map_err(|error| error.to_string())?,
                            ),
                        }],
                    },
                    buyer_trade_party: BuyerTradeParty {
                        name: self
                            .buyers_name
                            .ok_or_else(|| {
                                InvoiceBuilderError::MissingField("buyers_name".to_string())
                            })
                            .map_err(|error| error.to_string())?,
                        specified_legal_organization: self.buyers_specified_legal_organization.map(
                            |v| SpecifiedLegalOrganization {
                                id: LegalOrganizationID::new(v),
                            },
                        ),
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
                applicable_header_trade_delivery: ApplicableHeaderTradeDelivery::new_with_event(
                    self.occurrence_date.clone(),
                ),
                applicable_header_trade_settlement: ApplicableHeaderTradeSettlement {
                    invoice_currency_code,
                    specified_trade_settlement_payment_means: Vec::new(),
                    applicable_trade_tax: self.applicable_trade_tax,
                    specified_trade_allowance_charge: Vec::new(),
                    specified_trade_payment_terms: self.specified_trade_payment_terms.clone(),
                    specified_trade_settlement_header_monetary_summation: monetary_summation,
                },
            },
        ))
    }
}
