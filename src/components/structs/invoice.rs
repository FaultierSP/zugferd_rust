use super::agreement::{
    AgreementView, ObjectReference, PostalAddressDetails, ProjectReference, SupportingDocument,
    TaxRepresentative, TradeContact,
};
use super::context_document::Document;
use super::context_document::DocumentContext;
use super::delivery::{DeliveryView, ShipTo};
use super::line_item::InvoiceLineDetails;
use super::line_item::LineView;
use super::line_item::SupplyChainTradeTransaction;
use super::settlement::{
    SettlementView, TaxPoint, TaxTotalAmount, TradeAllowanceCharge, TradeAllowanceChargeReasonCode,
};
use super::xml_cleanup::remove_empty_elements;
use crate::components::enums::country_code::CountryCode;
use serde::{Serialize, Serializer};

//Specifications
#[derive(Debug)]
pub struct Invoice<'invoice> {
    //Namespaces
    xmlns_xsi: &'static str,
    xmlns_qdt: &'static str,
    xmlns_udt: &'static str,
    xmlns_rsm: &'static str,
    xmlns_ram: &'static str,

    //Document
    pub context: DocumentContext<'invoice>,
    pub document: Document<'invoice>,

    //Supply chain trade transaction
    pub supply_chain_trade_transaction: SupplyChainTradeTransaction<'invoice>,
    additional_details: Option<AdditionalInvoiceDetails<'invoice>>,
}

impl<'invoice> Invoice<'invoice> {
    pub fn new(
        context: DocumentContext<'invoice>,
        document: Document<'invoice>,
        supply_chain_trade_transaction: SupplyChainTradeTransaction<'invoice>,
    ) -> Self {
        Self {
            xmlns_xsi: "http://www.w3.org/2001/XMLSchema-instance",
            xmlns_qdt: "urn:un:unece:uncefact:data:standard:QualifiedDataType:100",
            xmlns_udt: "urn:un:unece:uncefact:data:standard:UnqualifiedDataType:100",
            xmlns_rsm: "urn:un:unece:uncefact:data:standard:CrossIndustryInvoice:100",
            xmlns_ram:
                "urn:un:unece:uncefact:data:standard:ReusableAggregateBusinessInformationEntity:100",
            context,
            document,
            supply_chain_trade_transaction,
            additional_details: None,
        }
    }

    pub(crate) fn with_additional_details(
        self,
        details: AdditionalInvoiceDetails<'invoice>,
    ) -> Result<Self, String> {
        let extended = InvoiceWithDetails::new(self, details)?;
        let mut invoice = extended.invoice;
        invoice.additional_details = Some(extended.details);
        Ok(invoice)
    }

    pub fn to_xml_string(&self) -> Result<String, String> {
        match quick_xml::se::to_string(&self) {
            Ok(xml_string) => Ok(format!(
                "<?xml version='1.0' encoding='UTF-8'?>{}",
                remove_empty_elements(xml_string)
            )),
            Err(e) => Err(e.to_string()),
        }
    }
}

#[derive(Serialize)]
#[serde(rename = "rsm:CrossIndustryInvoice")]
struct PlainInvoiceView<'s, 'a> {
    #[serde(rename = "@xmlns:xsi")]
    xsi: &'static str,
    #[serde(rename = "@xmlns:qdt")]
    qdt: &'static str,
    #[serde(rename = "@xmlns:udt")]
    udt: &'static str,
    #[serde(rename = "@xmlns:rsm")]
    rsm: &'static str,
    #[serde(rename = "@xmlns:ram")]
    ram: &'static str,
    #[serde(rename = "rsm:ExchangedDocumentContext")]
    context: &'s DocumentContext<'a>,
    #[serde(rename = "rsm:ExchangedDocument")]
    document: &'s Document<'a>,
    #[serde(rename = "rsm:SupplyChainTradeTransaction")]
    transaction: &'s SupplyChainTradeTransaction<'a>,
}

impl Serialize for Invoice<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if let Some(details) = &self.additional_details {
            InvoiceWithDetails::validate(self, details).map_err(serde::ser::Error::custom)?;
            InvoiceView::new(self, details).serialize(serializer)
        } else {
            PlainInvoiceView {
                xsi: self.xmlns_xsi,
                qdt: self.xmlns_qdt,
                udt: self.xmlns_udt,
                rsm: self.xmlns_rsm,
                ram: self.xmlns_ram,
                context: &self.context,
                document: &self.document,
                transaction: &self.supply_chain_trade_transaction,
            }
            .serialize(serializer)
        }
    }
}

/// Additional data for an EN 16931 invoice.
///
/// `line_details`, when nonempty, must contain one entry per invoice line, in
/// the same order. Existing invoice types and struct literals remain unchanged.
#[derive(Clone, Debug, Default)]
pub struct AdditionalInvoiceDetails<'a> {
    pub seller_description: Option<&'a str>,
    pub seller_contact: Option<TradeContact<'a>>,
    pub buyer_contact: Option<TradeContact<'a>>,
    pub seller_region: Option<&'a str>,
    pub buyer_region: Option<&'a str>,
    pub tax_representative: Option<TaxRepresentative<'a>>,
    pub ship_to: Option<ShipTo<'a>>,
    pub seller_order_reference: Option<&'a str>,
    pub tender_reference: Option<&'a str>,
    pub object_reference: Option<ObjectReference<'a>>,
    pub supporting_documents: Vec<SupportingDocument<'a>>,
    pub project: Option<ProjectReference<'a>>,
    pub receiving_advice_reference: Option<&'a str>,
    pub accounting_currency_tax: Option<TaxTotalAmount>,
    pub tax_point: Option<TaxPoint<'a>>,
    pub document_charges: Vec<TradeAllowanceCharge<'a>>,
    pub line_details: Vec<InvoiceLineDetails<'a>>,
}

/// An invoice with the additional EN 16931 fields, preserving the existing invoice model.
#[derive(Debug)]
pub struct InvoiceWithDetails<'a> {
    invoice: Invoice<'a>,
    details: AdditionalInvoiceDetails<'a>,
}

impl<'a> InvoiceWithDetails<'a> {
    pub(crate) fn new(
        invoice: Invoice<'a>,
        details: AdditionalInvoiceDetails<'a>,
    ) -> Result<Self, String> {
        Self::validate(&invoice, &details)?;
        Ok(Self { invoice, details })
    }

    fn validate(
        invoice: &Invoice<'_>,
        details: &AdditionalInvoiceDetails<'_>,
    ) -> Result<(), String> {
        if !matches!(
            invoice.context.guideline.id,
            crate::components::enums::specification_level::SpecificationLevel::En16931
                | crate::components::enums::specification_level::SpecificationLevel::XRechnung
        ) {
            return Err("EN 16931 fields require the EN 16931 or XRechnung profile".into());
        }
        let transaction = &invoice.supply_chain_trade_transaction;
        if !details.line_details.is_empty()
            && details.line_details.len()
                != transaction.included_supply_chain_trade_line_items.len()
        {
            return Err("EN 16931 line details must have one entry per invoice line".into());
        }
        if let Some(tax) = &details.accounting_currency_tax {
            if tax.currency_id.as_str()
                == transaction
                    .applicable_header_trade_settlement
                    .invoice_currency_code
                    .as_str()
            {
                return Err("VAT accounting currency must differ from invoice currency".into());
            }
            if !tax.amount.is_finite() {
                return Err("VAT accounting currency amount must be finite".into());
            }
        }
        for (name, value) in [
            ("seller description", details.seller_description),
            ("seller region", details.seller_region),
            ("buyer region", details.buyer_region),
            ("seller order reference", details.seller_order_reference),
            ("tender reference", details.tender_reference),
            (
                "receiving advice reference",
                details.receiving_advice_reference,
            ),
        ] {
            validate_optional_text(name, value)?;
        }
        if let Some(TaxPoint::DateCode(code)) = &details.tax_point {
            if !matches!(*code, "5" | "29" | "72") {
                return Err("VAT point date code must be 5, 29, or 72".into());
            }
        }
        if let Some(project) = &details.project {
            require_text("project ID", project.id)?;
            require_text("project name", project.name)?;
        }
        if let Some(reference) = &details.object_reference {
            validate_object_reference(reference)?;
        }
        for document in &details.supporting_documents {
            require_text("supporting document ID", document.id)?;
            validate_optional_text("supporting document description", document.description)?;
            validate_optional_text("supporting document URI", document.uri)?;
            if let Some(attachment) = &document.attachment {
                if !matches!(
                    attachment.mime_code,
                    "application/pdf"
                        | "application/vnd.oasis.opendocument.spreadsheet"
                        | "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
                        | "image/jpeg"
                        | "image/png"
                        | "text/csv"
                ) {
                    return Err("Attachment MIME code is not permitted by EN 16931".into());
                }
                require_text("attachment filename", attachment.filename)?;
                if attachment.content.is_empty() {
                    return Err("Attachment content must not be empty".into());
                }
            }
        }
        if let Some(rep) = &details.tax_representative {
            require_text("tax representative name", rep.name)?;
            require_text("tax representative VAT ID", rep.vat_id)?;
            validate_address(&rep.address)?;
        }
        if let Some(ship) = &details.ship_to {
            validate_address(&ship.address)?;
        }
        for contact in [&details.seller_contact, &details.buyer_contact]
            .into_iter()
            .flatten()
        {
            if contact.person_name.is_some() && contact.department_name.is_some() {
                return Err("A contact cannot have both a person and department name".into());
            }
            if contact.person_name.is_none()
                && contact.department_name.is_none()
                && contact.telephone.is_none()
                && contact.email.is_none()
            {
                return Err("Contact must contain at least one field".into());
            }
            for (name, value) in [
                ("contact person", contact.person_name),
                ("contact department", contact.department_name),
                ("contact telephone", contact.telephone),
                ("contact email", contact.email),
            ] {
                validate_optional_text(name, value)?;
            }
        }
        for charge in &details.document_charges {
            validate_charge(charge, true)?;
        }
        for line in &details.line_details {
            for (name, value) in [
                ("seller item ID", line.seller_assigned_id),
                ("buyer item ID", line.buyer_assigned_id),
                ("buyer order line ID", line.buyer_order_line_id),
                (
                    "buyer accounting reference",
                    line.buyer_accounting_reference,
                ),
            ] {
                validate_optional_text(name, value)?;
            }
            if let Some(reference) = &line.object_reference {
                validate_object_reference(reference)?;
            }
            if line.origin_country == Some(CountryCode::NotSet) {
                return Err("Item origin country must be set".into());
            }
            for characteristic in &line.characteristics {
                require_text("item characteristic name", characteristic.name)?;
                require_text("item characteristic value", characteristic.value)?;
            }
            for classification in &line.classifications {
                require_text("item classification code", classification.code)?;
                require_text("item classification list ID", classification.list_id)?;
                validate_optional_text(
                    "item classification list version",
                    classification.list_version_id,
                )?;
            }
            if matches!((line.period_start, line.period_end), (Some(start), Some(end)) if start > end)
            {
                return Err("Invoice line period start must not be after its end".into());
            }
            for charge in &line.charges {
                validate_charge(charge, false)?;
            }
        }
        Ok(())
    }

    pub fn to_xml_string(&self) -> Result<String, String> {
        quick_xml::se::to_string(self)
            .map(|xml| {
                format!(
                    "<?xml version='1.0' encoding='UTF-8'?>{}",
                    remove_empty_elements(xml)
                )
            })
            .map_err(|error| error.to_string())
    }
}

impl Serialize for InvoiceWithDetails<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        InvoiceView::new(&self.invoice, &self.details).serialize(serializer)
    }
}

fn require_text(name: &str, value: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        return Err(format!("{name} must not be empty"));
    }
    Ok(())
}

fn validate_optional_text(name: &str, value: Option<&str>) -> Result<(), String> {
    if let Some(value) = value {
        require_text(name, value)?;
    }
    Ok(())
}

fn validate_object_reference(reference: &ObjectReference<'_>) -> Result<(), String> {
    require_text("object reference ID", reference.id)?;
    validate_optional_text("object reference scheme", reference.scheme_id)
}

fn validate_address(address: &PostalAddressDetails<'_>) -> Result<(), String> {
    if address.address.country_id == CountryCode::NotSet {
        return Err("Address country must be set".into());
    }
    for (name, value) in [
        ("address postcode", address.address.postcode_code),
        ("address line one", address.address.line_one),
        ("address line two", address.address.line_two),
        ("address line three", address.address.line_three),
        ("address city", address.address.city_name),
        ("address region", address.region),
    ] {
        validate_optional_text(name, value)?;
    }
    Ok(())
}

fn validate_charge(charge: &TradeAllowanceCharge<'_>, document: bool) -> Result<(), String> {
    if charge.category_trade_tax.is_some() != document {
        return Err(
            "A document charge requires a VAT category; a line charge must not have one".into(),
        );
    }
    if let Some(tax) = charge.category_trade_tax {
        if tax.type_code != "VAT" {
            return Err("Document allowance or charge tax type must be VAT".into());
        }
        if tax
            .rate_applicable_percent
            .is_some_and(|rate| !rate.is_finite())
        {
            return Err("Document allowance or charge VAT rate must be finite".into());
        }
    }
    if matches!(
        (&charge.reason_code, charge.charge),
        (Some(TradeAllowanceChargeReasonCode::Allowance(_)), true)
            | (Some(TradeAllowanceChargeReasonCode::Charge(_)), false)
    ) {
        return Err("Allowance or charge reason code does not match the charge indicator".into());
    }
    if charge.reason_code.is_none() && charge.reason.is_none() {
        return Err("An allowance or charge requires a reason or reason code".into());
    }
    validate_optional_text("allowance or charge reason", charge.reason)?;
    if !charge.actual_amount.is_finite()
        || charge
            .calculation_percent
            .is_some_and(|value| !value.is_finite())
        || charge.basis_amount.is_some_and(|value| !value.is_finite())
    {
        return Err("Allowance or charge amounts and percentages must be finite".into());
    }
    Ok(())
}

#[derive(Serialize)]
#[serde(rename = "rsm:CrossIndustryInvoice")]
struct InvoiceView<'s, 'a> {
    #[serde(rename = "@xmlns:xsi")]
    xsi: &'static str,
    #[serde(rename = "@xmlns:qdt")]
    qdt: &'static str,
    #[serde(rename = "@xmlns:udt")]
    udt: &'static str,
    #[serde(rename = "@xmlns:rsm")]
    rsm: &'static str,
    #[serde(rename = "@xmlns:ram")]
    ram: &'static str,
    #[serde(rename = "rsm:ExchangedDocumentContext")]
    context: &'s DocumentContext<'a>,
    #[serde(rename = "rsm:ExchangedDocument")]
    document: &'s Document<'a>,
    #[serde(rename = "rsm:SupplyChainTradeTransaction")]
    transaction: TransactionView<'s, 'a>,
}

impl<'s, 'a> InvoiceView<'s, 'a> {
    fn new(invoice: &'s Invoice<'a>, details: &'s AdditionalInvoiceDetails<'a>) -> Self {
        let transaction = &invoice.supply_chain_trade_transaction;
        Self {
            xsi: "http://www.w3.org/2001/XMLSchema-instance",
            qdt: "urn:un:unece:uncefact:data:standard:QualifiedDataType:100",
            udt: "urn:un:unece:uncefact:data:standard:UnqualifiedDataType:100",
            rsm: "urn:un:unece:uncefact:data:standard:CrossIndustryInvoice:100",
            ram:
                "urn:un:unece:uncefact:data:standard:ReusableAggregateBusinessInformationEntity:100",
            context: &invoice.context,
            document: &invoice.document,
            transaction: TransactionView::new(transaction, details),
        }
    }
}

#[derive(Serialize)]
struct TransactionView<'s, 'a> {
    #[serde(rename = "ram:IncludedSupplyChainTradeLineItem")]
    lines: Vec<LineView<'s, 'a>>,
    #[serde(rename = "ram:ApplicableHeaderTradeAgreement")]
    agreement: AgreementView<'s, 'a>,
    #[serde(rename = "ram:ApplicableHeaderTradeDelivery")]
    delivery: DeliveryView<'s, 'a>,
    #[serde(rename = "ram:ApplicableHeaderTradeSettlement")]
    settlement: SettlementView<'s, 'a>,
}

impl<'s, 'a> TransactionView<'s, 'a> {
    fn new(
        base: &'s SupplyChainTradeTransaction<'a>,
        details: &'s AdditionalInvoiceDetails<'a>,
    ) -> Self {
        Self {
            lines: base
                .included_supply_chain_trade_line_items
                .iter()
                .enumerate()
                .map(|(index, line)| LineView::new(line, details.line_details.get(index)))
                .collect(),
            agreement: AgreementView::new(&base.applicable_header_trade_agreement, details),
            delivery: DeliveryView::new(&base.applicable_header_trade_delivery, details),
            settlement: SettlementView::new(&base.applicable_header_trade_settlement, details),
        }
    }
}
