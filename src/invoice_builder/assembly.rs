use super::{InvoiceBuilder, InvoiceBuilderError};
use crate::components::enums::payment_means_code::PaymentMeansCode;
use crate::components::enums::specification_level::SpecificationLevel;
use crate::components::structs::*;

/// Treats empty or whitespace-only strings as absent so they are omitted from the XML.
fn non_empty(value: Option<&str>) -> Option<&str> {
    value.filter(|v| !v.trim().is_empty())
}

impl<'invoice_builder> InvoiceBuilder<'invoice_builder> {
    /// Builds an EN 16931 invoice with additional fields, without changing the
    /// existing invoice and line-item struct APIs.
    pub fn build_with_details(
        &mut self,
        details: AdditionalInvoiceDetails<'invoice_builder>,
    ) -> Result<InvoiceWithDetails<'invoice_builder>, String> {
        let details = self.collect_additional_details(details)?;
        InvoiceWithDetails::new(self.build_base(SpecificationLevel::En16931)?, details)
    }

    fn collect_additional_details(
        &self,
        mut details: AdditionalInvoiceDetails<'invoice_builder>,
    ) -> Result<AdditionalInvoiceDetails<'invoice_builder>, String> {
        macro_rules! merge_field {
            ($field:ident) => {
                if details.$field.is_some() && self.$field.is_some() {
                    return Err(format!("{} was supplied twice", stringify!($field)));
                }
                if details.$field.is_none() {
                    details.$field = self.$field.clone();
                }
            };
        }
        merge_field!(seller_region);
        merge_field!(buyer_region);
        merge_field!(tax_representative);
        merge_field!(ship_to);
        merge_field!(seller_order_reference);
        merge_field!(tender_reference);
        merge_field!(object_reference);
        merge_field!(project);
        merge_field!(receiving_advice_reference);
        merge_field!(accounting_currency_tax);
        merge_field!(tax_point);
        if !self.line_details.is_empty() && !details.line_details.is_empty() {
            return Err("Line details were supplied twice".into());
        }
        if details.line_details.is_empty() {
            details.line_details = self.line_details.clone();
        }
        let mut documents = self.supporting_documents.clone();
        documents.extend(details.supporting_documents);
        details.supporting_documents = documents;
        let mut charges = self.document_charges.clone();
        charges.extend(details.document_charges);
        details.document_charges = charges;

        if details.seller_description.is_some() && self.sellers_description.is_some() {
            return Err("Seller description was supplied twice".into());
        }
        details.seller_description = details.seller_description.or(self.sellers_description);
        for (detail, existing) in [
            (
                &mut details.seller_contact,
                &self.sellers_defined_trade_contact,
            ),
            (
                &mut details.buyer_contact,
                &self.buyers_defined_trade_contact,
            ),
        ] {
            if let Some(existing) = existing {
                let contact = detail.get_or_insert_with(TradeContact::default);
                for (field, value) in [
                    (&mut contact.person_name, existing.person_name),
                    (&mut contact.department_name, existing.department_name),
                    (&mut contact.telephone, existing.telephone),
                    (&mut contact.email, existing.email),
                ] {
                    if field.is_some() && value.is_some() {
                        return Err(
                            "Contact field was supplied both through the builder and EN 16931 details"
                                .into(),
                        );
                    }
                    *field = field.or(value);
                }
            }
        }
        Ok(details)
    }

    fn has_additional_fields(&self) -> bool {
        self.sellers_defined_trade_contact
            .as_ref()
            .and_then(|contact| contact.email)
            .is_some()
            || self
                .buyers_defined_trade_contact
                .as_ref()
                .and_then(|contact| contact.email)
                .is_some()
            || self.seller_region.is_some()
            || self.buyer_region.is_some()
            || self.tax_representative.is_some()
            || self.ship_to.is_some()
            || self.seller_order_reference.is_some()
            || self.tender_reference.is_some()
            || self.object_reference.is_some()
            || !self.supporting_documents.is_empty()
            || self.project.is_some()
            || self.receiving_advice_reference.is_some()
            || self.accounting_currency_tax.is_some()
            || self.tax_point.is_some()
            || !self.document_charges.is_empty()
            || !self.line_details.is_empty()
    }

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
        let supported = matches!(
            specification_level,
            SpecificationLevel::En16931 | SpecificationLevel::XRechnung
        );
        if self.has_additional_fields() && !supported {
            return Err("EN 16931 fields require the EN 16931 or XRechnung profile".into());
        }
        let has_contact_or_description = self.sellers_description.is_some()
            || self.sellers_defined_trade_contact.is_some()
            || self.buyers_defined_trade_contact.is_some();
        let details = if supported && (self.has_additional_fields() || has_contact_or_description) {
            Some(self.collect_additional_details(AdditionalInvoiceDetails::default())?)
        } else {
            None
        };
        let invoice = self.build_base(specification_level)?;
        match details {
            Some(details) => invoice.with_additional_details(details),
            None => Ok(invoice),
        }
    }

    fn build_base(
        &mut self,
        specification_level: SpecificationLevel,
    ) -> Result<Invoice<'invoice_builder>, String> {
        //Check if none of the fields is empty
        self.all_fields_are_set(specification_level)?;

        if self.payer_financial_account_ibanid.is_some()
            || self.payee_account_ibanid.is_some()
            || self
                .payee_financial_account_proprietary_id
                .is_some()
            || self
                .payee_party_creditor_financial_account_info
                .is_some()
            || self
                .specified_trade_settlement_payment_means_type_code
                .is_some()
            || self
                .specified_trade_settlement_payment_means_information
                .is_some()
            || self.applicable_settlement_financial_card.is_some()
        {
            if self.specified_trade_settlement_payment_means.is_empty() {
                self.specified_trade_settlement_payment_means
                    .push(SpecifiedTradeSettlementPaymentMeans {
                        type_code: self
                            .specified_trade_settlement_payment_means_type_code
                            .map(PaymentMeansCode::MutuallyDefined)
                            .unwrap_or(PaymentMeansCode::CreditTransfer),
                        information: non_empty(
                            self.specified_trade_settlement_payment_means_information,
                        ),
                        applicable_trade_settlement_financial_card: None,
                        payer_party_debtor_financial_account: None,
                        payee_party_creditor_financial_account: None,
                        payee_specified_creditor_financial_institution: None,
                    });
            }
            let payment_means = &mut self.specified_trade_settlement_payment_means[0];
            payment_means.type_code = self
                .specified_trade_settlement_payment_means_type_code
                .map(PaymentMeansCode::MutuallyDefined)
                .unwrap_or(PaymentMeansCode::CreditTransfer);
            payment_means.information =
                non_empty(self.specified_trade_settlement_payment_means_information);
            if let Some(card) = self.applicable_settlement_financial_card {
                payment_means.applicable_trade_settlement_financial_card = Some(card);
            }
            if let Some(ibanid) = self.payer_financial_account_ibanid {
                payment_means.payer_party_debtor_financial_account =
                    Some(PayerPartyDebtorFinancialAccount { ibanid });
            }
            if let Some(info) = self.payee_party_creditor_financial_account_info {
                payment_means.payee_party_creditor_financial_account = Some(info.account);
                payment_means.payee_specified_creditor_financial_institution = info.institution;
            }
        }

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
                included_note: self.document_notes.clone().filter(|notes| !notes.is_empty()),
            },
            SupplyChainTradeTransaction {
                included_supply_chain_trade_line_items: self
                    .included_supply_chain_trade_line_items
                    .clone(),
                applicable_header_trade_agreement: ApplicableHeaderTradeAgreement {
                    buyer_reference: non_empty(self.buyer_reference),
                    seller_trade_party: SellerTradeParty {
                        id: Vec::new(),
                        global_id: Vec::new(),
                        name: self
                            .sellers_name
                            .ok_or_else(|| {
                                InvoiceBuilderError::MissingField("sellers_name".to_string())
                            })
                            .map_err(|error| error.to_string())?,
                        specified_legal_organization: non_empty(self.sellers_specified_legal_organization)
                            .map(|v| SpecifiedLegalOrganization {
                                id: LegalOrganizationID::new(v),
                            }),
                        postal_trade_address: PostalTradeAddress {
                            country_id: self.sellers_postal_trade_address.country_id,
                            postcode_code: non_empty(self.sellers_postal_trade_address.postcode_code),
                            line_one: non_empty(self.sellers_postal_trade_address.line_one),
                            line_two: non_empty(self.sellers_postal_trade_address.line_two),
                            line_three: non_empty(self.sellers_postal_trade_address.line_three),
                            city_name: non_empty(self.sellers_postal_trade_address.city_name),
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
                        specified_legal_organization: non_empty(self.buyers_specified_legal_organization).map(
                            |v| SpecifiedLegalOrganization {
                                id: LegalOrganizationID::new(v),
                            },
                        ),
                        postal_trade_address: PostalTradeAddress {
                            country_id: self.buyers_postal_trade_address.country_id,
                            postcode_code: non_empty(self.buyers_postal_trade_address.postcode_code),
                            line_one: non_empty(self.buyers_postal_trade_address.line_one),
                            line_two: non_empty(self.buyers_postal_trade_address.line_two),
                            line_three: non_empty(self.buyers_postal_trade_address.line_three),
                            city_name: non_empty(self.buyers_postal_trade_address.city_name),
                        },
                    },
                    buyer_order_referenced_document: non_empty(self.buyers_order_specified_document).map(
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
                    specified_trade_settlement_payment_means: self
                        .specified_trade_settlement_payment_means
                        .clone(),
                    applicable_trade_tax: self.applicable_trade_tax.clone(),
                    specified_trade_allowance_charge: self.specified_trade_allowance_charge.clone(),
                    specified_trade_payment_terms: self.specified_trade_payment_terms.clone(),
                    specified_trade_settlement_header_monetary_summation: monetary_summation,
                },
            },
        ))
    }
}
