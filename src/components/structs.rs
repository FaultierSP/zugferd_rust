use serde::{Serialize,Serializer};
use chrono::NaiveDate;

use crate::components::enums::{
    specification_level::SpecificationLevel,
    invoice_type_code::InvoiceTypeCode,
    country_code::CountryCode,
    currency_code::CurrencyCode,
};

use crate::components::constants;

//Formatting and serializing functions
fn f64_format <S> (value: &f64, serializer: S) -> Result<S::Ok, S::Error> where S:Serializer {
    let formatted = format!("{:.2}",value);
    serializer.serialize_str(&formatted)
}

//Specifications
#[derive(Serialize)]
#[serde(rename="rsm:CrossIndustryInvoice")]
pub struct Invoice<'invoice> {
    //Namespaces
    #[serde(rename="xmlns:xsi")]
    xmlns_xsi: &'static str,
    #[serde(rename="xmlns:qdt")]
    xmlns_qdt: &'static str,
    #[serde(rename="xmlns:udt")]
    xmlns_udt: &'static str,
    #[serde(rename="xmlns:rsm")]
    xmlns_rsm: &'static str,
    #[serde(rename="xmlns:ram")]
    xmlns_ram: &'static str,
    
    //Document
    #[serde(rename="rsm:ExchangedDocumentContext")]
    pub context: DocumentContext<'invoice>,
    #[serde(rename="rsm:ExchangedDocument")]
    pub document: Document<'invoice>,

    //Supply chain trade transaction
    #[serde(rename="rsm:SupplyChainTradeTransaction")]
    supply_chain_trade_transaction: SupplyChainTradeTransaction<'invoice>,
}

impl<'invoice> Invoice<'invoice> {
    pub fn new(
        context: DocumentContext<'invoice>,
        document: Document<'invoice>,
        supply_chain_trade_transaction: SupplyChainTradeTransaction<'invoice>
    ) -> Self {
        Self {
            xmlns_xsi:"http://www.w3.org/2001/XMLSchema-instance",
            xmlns_qdt:"urn:un:unece:uncefact:data:standard:QualifiedDataType:100",
            xmlns_udt:"urn:un:unece:uncefact:data:standard:UnqualifiedDataType:100",
            xmlns_rsm:"urn:un:unece:uncefact:data:standard:CrossIndustryInvoice:100",
            xmlns_ram:"urn:un:unece:uncefact:data:standard:ReusableAggregateBusinessInformationEntity:100",
            context,
            document,
            supply_chain_trade_transaction,
        }
    }
}

#[derive(Serialize)]
pub struct DocumentContext<'invoice> {
    #[serde(rename="ram:BusinessProcessSpecifiedDocumentContextParameter")]
    pub business_process: BusinessProcess<'invoice>,
    #[serde(rename="ram:GuidelineSpecifiedDocumentContextParameter")]
    pub guideline: Guideline,
}

#[derive(Serialize)]
pub struct Document<'invoice> {
    #[serde(rename="ram:ID")]
    pub id: &'invoice str,
    #[serde(rename="ram:TypeCode")]
    pub type_code: InvoiceTypeCode,
    #[serde(rename="ram:IssueDateTime")]
    pub issue_date_time: IssueDateTime<'invoice>,
}

#[derive(Serialize)]
pub struct BusinessProcess<'invoice> {
    #[serde(rename="ram:ID")]
    pub id: &'invoice str,
}

#[derive(Serialize)]
pub struct Guideline {
    #[serde(rename="ram:ID")]
    pub id: SpecificationLevel,
}

#[derive(Serialize,Clone)]
pub struct DateTimeString<'invoice> {
    #[serde(rename="@format")]
    format: &'invoice str,
    #[serde(rename="$value")]
    value: String,
}

impl<'invoice> DateTimeString<'invoice> {
    pub fn new(date: NaiveDate) -> Self {
        Self {
            format: "102",
            value: date.format(constants::DATE_TIME_FORMAT_102).to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct IssueDateTime<'invoice> {
    #[serde(rename="udt:DateTimeString")]
    pub date_time_string: DateTimeString<'invoice>,
}

#[derive(Serialize)]
pub struct SupplyChainTradeTransaction<'invoice> {
    #[serde(rename="ram:ApplicableHeaderTradeAgreement")]
    pub applicable_header_trade_agreement: ApplicableHeaderTradeAgreement<'invoice>,
    #[serde(rename="ram:ApplicableHeaderTradeDelivery")]
    pub applicable_header_trade_delivery: ApplicableHeaderTradeDelivery,
    #[serde(rename="ram:ApplicableHeaderTradeSettlement")]
    pub applicable_header_trade_settlement: ApplicableHeaderTradeSettlement,
}

#[derive(Serialize)]
pub struct ApplicableHeaderTradeAgreement<'invoice> {
    #[serde(rename="ram:BuyerReference")]
    pub buyer_reference: &'invoice str,
    #[serde(rename="ram:SellerTradeParty")]
    pub seller_trade_party: SellerTradeParty<'invoice>,
    #[serde(rename="ram:BuyerTradeParty")]
    pub buyer_trade_party: BuyerTradeParty<'invoice>,
    #[serde(rename="ram:BuyerOrderReferencedDocument")]
    pub buyer_order_referenced_document: BuyerOrderReferencedDocument<'invoice>,
}

#[derive(Serialize)]
pub struct SellerTradeParty<'invoice> {
    #[serde(rename="ram:Name")]
    pub name: &'invoice str,
    #[serde(rename="ram:SpecifiedLegalOrganization")]
    pub specified_legal_organization: SpecifiedLegalOrganization<'invoice>,
    #[serde(rename="ram:PostalTradeAddress")]
    pub postal_trade_address: PostalTradeAddress,
    #[serde(rename="ram:SpecifiedTaxRegistration")]
    pub specified_tax_registration: SpecifiedTaxRegistration<'invoice>,
}

#[derive(Serialize)]
pub struct LegalOrganizationID<'invoice> {
    #[serde(rename = "@schemeID")]
    scheme_id: &'static str,
    #[serde(rename = "$value")]
    value: &'invoice str,
}

impl<'invoice> LegalOrganizationID<'invoice> {
    pub fn new(value: &'invoice str) -> Self {
        Self {
            scheme_id: "0002",
            value,
        }
    }
}

#[derive(Serialize)]
pub struct SpecifiedLegalOrganization<'invoice> {
    #[serde(rename="ram:ID")]
    pub id: LegalOrganizationID<'invoice>,
}

#[derive(Serialize)]
pub struct PostalTradeAddress {
    #[serde(rename="ram:CountryID")]
    pub country_id: CountryCode,
}

#[derive(Serialize)]
pub struct SpecifiedTaxRegistrationID<'invoice> {
    #[serde(rename = "@schemeID")]
    scheme_id: &'static str,
    #[serde(rename = "$value")]
    value: &'invoice str,
}

impl<'invoice> SpecifiedTaxRegistrationID<'invoice> {
    pub fn new(value: &'invoice str) -> Self {
        Self {
            scheme_id: "VA",
            value,
        }
    }
}

#[derive(Serialize)]
pub struct SpecifiedTaxRegistration<'invoice> {
    #[serde(rename="ram:ID")]
    pub id: SpecifiedTaxRegistrationID<'invoice>,
}

#[derive(Serialize)]
pub struct BuyerTradeParty<'invoice> {
    #[serde(rename="ram:Name")]
    pub name: &'invoice str,
    #[serde(rename="ram:SpecifiedLegalOrganization")]
    pub specified_legal_organization: SpecifiedLegalOrganization<'invoice>,
}

#[derive(Serialize)]
pub struct BuyerOrderReferencedDocument<'invoice> {
    #[serde(rename="ram:IssuerAssignedID")]
    pub issuer_assigned_id: &'invoice str,
}

#[derive(Serialize)]
pub struct ApplicableHeaderTradeDelivery {

}

#[derive(Serialize)]
pub struct ApplicableHeaderTradeSettlement {
    #[serde(rename="ram:InvoiceCurrencyCode")]
    pub invoice_currency_code: CurrencyCode,
    #[serde(rename="ram:SpecifiedTradeSettlementHeaderMonetarySummation")]
    pub specified_trade_settlement_header_monetary_summation: SpecifiedTradeSettlementHeaderMonetarySummation,
    
}

#[derive(Serialize)]
pub struct SpecifiedTradeSettlementHeaderMonetarySummation {
    #[serde(rename="ram:TaxBasisTotalAmount",serialize_with="f64_format")]
    pub tax_basis_total_amount: f64,
    #[serde(rename="ram:TaxTotalAmount")]
    pub tax_total_amount: TaxTotalAmount,
    #[serde(rename="ram:GrandTotalAmount",serialize_with="f64_format")]
    pub grand_total_amount: f64,
    #[serde(rename="ram:DuePayableAmount",serialize_with="f64_format")]
    pub due_payable_amount: f64,
}

#[derive(Serialize)]
pub struct TaxTotalAmount {
    #[serde(rename="@currencyID")]
    pub currency_id: CurrencyCode,
    #[serde(rename="$value",serialize_with="f64_format")]
    pub amount: f64,
}

impl TaxTotalAmount {
    pub fn new(currency_id: CurrencyCode, amount: f64) -> Self {
        TaxTotalAmount {
            currency_id: currency_id,
            amount: amount,
        }
    }
}