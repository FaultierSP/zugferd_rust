use serde::{Serialize,Serializer};
use chrono::NaiveDate;

use crate::components::enums::{
    specification_level::SpecificationLevel,
    invoice_type_code::InvoiceTypeCode,
    country_code::CountryCode,
    currency_code::CurrencyCode,
    vat_category_code::VATCategoryCode,
    identifier_scheme_code::IdentifierSchemeCode,
    unit_code::UnitCode,
};

use crate::components::constants;

//Formatting and serializing functions
fn f64_format <S> (value: &f64, serializer: S) -> Result<S::Ok, S::Error> where S:Serializer {
    let formatted = format!("{:.2}",value);
    serializer.serialize_str(&formatted)
}

fn f64_format_with_precision_4 <S> (value: &f64, serializer: S) -> Result<S::Ok, S::Error> where S:Serializer {
    let formatted = format!("{:.4}",value);
    serializer.serialize_str(&formatted)
}

fn format_f64_option <S> (option: &Option<f64>, serializer: S) -> Result<S::Ok, S::Error>
where S:Serializer
{
    if option.is_some() {
        let formatted = format!("{:.2}",option.unwrap());
        return serializer.serialize_str(&formatted);
    }
    else {
        return serializer.serialize_none();
        //return Err(serde::ser::Error::custom("Expected a value, got None"));
    }
}

fn vector_is_empty <S> (vector: &Vec<S>) -> bool {
    vector.is_empty()
}

//Specifications
#[derive(Serialize, Debug)]
#[serde(rename="rsm:CrossIndustryInvoice")]
pub struct Invoice<'invoice> {
    //Namespaces
    #[serde(rename="@xmlns:xsi")]
    xmlns_xsi: &'static str,
    #[serde(rename="@xmlns:qdt")]
    xmlns_qdt: &'static str,
    #[serde(rename="@xmlns:udt")]
    xmlns_udt: &'static str,
    #[serde(rename="@xmlns:rsm")]
    xmlns_rsm: &'static str,
    #[serde(rename="@xmlns:ram")]
    xmlns_ram: &'static str,
    
    //Document
    #[serde(rename="rsm:ExchangedDocumentContext")]
    pub context: DocumentContext<'invoice>,
    #[serde(rename="rsm:ExchangedDocument")]
    pub document: Document<'invoice>,

    //Supply chain trade transaction
    #[serde(rename="rsm:SupplyChainTradeTransaction")]
    pub supply_chain_trade_transaction: SupplyChainTradeTransaction<'invoice>,
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

#[derive(Serialize, Debug)]
pub struct DocumentContext<'invoice> {
    #[serde(rename="ram:BusinessProcessSpecifiedDocumentContextParameter", skip_serializing_if = "Option::is_none")]
    pub business_process: Option<BusinessProcess<'invoice>>,
    #[serde(rename="ram:GuidelineSpecifiedDocumentContextParameter")]
    pub guideline: Guideline,
}

#[derive(Serialize, Debug)]
pub struct Document<'invoice> {
    #[serde(rename="ram:ID")]
    pub id: &'invoice str,
    #[serde(rename="ram:TypeCode")]
    pub type_code: InvoiceTypeCode,
    #[serde(rename="ram:IssueDateTime")]
    pub issue_date_time: IssueDateTime<'invoice>,
    #[serde(rename="ram:IncludedNote", skip_serializing_if = "Option::is_none")]
    pub included_note: Option<Vec<IncludedNote>>,
}

#[derive(Serialize, Clone, Debug)]
pub struct BusinessProcess<'invoice> {
    #[serde(rename="ram:ID")]
    pub id: &'invoice str,
}

#[derive(Serialize, Clone, Debug)]
pub struct Guideline {
    #[serde(rename="ram:ID")]
    pub id: SpecificationLevel,
}

#[derive(Serialize, Clone, Debug)]
pub struct IssueDateTime<'invoice> {
    #[serde(rename="udt:DateTimeString")]
    pub date_time_string: DateTimeString<'invoice>,
}

#[derive(Serialize, Clone, Debug)]
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

#[derive(Serialize, Clone, Debug)]
pub struct IncludedNote {
    #[serde(rename="ram:Content")]
    pub content: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct SupplyChainTradeTransaction<'invoice> {
    #[serde(rename="ram:IncludedSupplyChainTradeLineItem", skip_serializing_if = "vector_is_empty")]
    pub included_supply_chain_trade_line_items: Vec<IncludedSupplyChainTradeLineItem<'invoice>>,
    #[serde(rename="ram:ApplicableHeaderTradeAgreement")]
    pub applicable_header_trade_agreement: ApplicableHeaderTradeAgreement<'invoice>,
    #[serde(rename="ram:ApplicableHeaderTradeDelivery")]
    pub applicable_header_trade_delivery: ApplicableHeaderTradeDelivery<'invoice>,
    #[serde(rename="ram:ApplicableHeaderTradeSettlement")]
    pub applicable_header_trade_settlement: ApplicableHeaderTradeSettlement<'invoice>,
}

#[derive(Serialize, Clone, Debug)]
pub struct IncludedSupplyChainTradeLineItem<'invoice> {
    #[serde(rename="ram:AssociatedDocumentLineDocument")]
    pub associated_document_line_document: AssociatedDocumentLineDocument<'invoice>,
    #[serde(rename="ram:SpecifiedTradeProduct")]
    pub specified_trade_product: SpecifiedTradeProduct<'invoice>,
    #[serde(rename="ram:SpecifiedLineTradeAgreement")]
    pub specified_line_trade_agreement: SpecifiedLineTradeAgreement,
    #[serde(rename="ram:SpecifiedLineTradeDelivery")]
    pub specified_line_trade_delivery: SpecifiedLineTradeDelivery,
    #[serde(rename="ram:SpecifiedLineTradeSettlement")]
    pub specified_line_trade_settlement: SpecifiedLineTradeSettlement<'invoice>,
}

#[derive(Serialize, Clone, Debug)]
pub struct AssociatedDocumentLineDocument<'invoice> {
    #[serde(rename="ram:LineID")]
    pub line_id: &'invoice str,
    #[serde(rename="ram:IncludedNote", skip_serializing_if = "Option::is_none")]
    /// BT-127-00
    pub included_note: Option<&'invoice str>,
}

#[derive(Serialize, Clone, Debug)]
pub struct SpecifiedTradeProduct<'invoice> {
    #[serde(rename="ram:GlobalID", skip_serializing_if = "Option::is_none")]
    /// BT-157
    pub global_id: Option<GlobalID<'invoice>>,
    #[serde(rename="ram:Name")]
    pub name: &'invoice str,
    //#[serde(rename="ram:Description")]
    //pub description: &'invoice str,
}

#[derive(Serialize, Clone, Debug)]
pub struct GlobalID<'invoice> {
    #[serde(rename="@schemeID")]
    pub scheme_id: IdentifierSchemeCode,
    #[serde(rename="$value")]
    pub value: &'invoice str,
}

impl <'invoice> GlobalID<'invoice> {
    pub fn new(scheme_id: IdentifierSchemeCode, value: &'invoice str) -> Self {
        Self {
            scheme_id,
            value,
        }
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct SpecifiedLineTradeAgreement {
    #[serde(rename="ram:GrossPriceProductTradePrice", skip_serializing_if = "Option::is_none")]
    pub gross_price_product_trade_price: Option<GrossPriceProductTradePrice>,
    #[serde(rename="ram:NetPriceProductTradePrice")]
    pub net_price_product_trade_price: NetPriceProductTradePrice,
}

#[derive(Serialize, Clone, Debug)]
pub struct GrossPriceProductTradePrice {
    #[serde(rename="ram:ChargeAmount",serialize_with="f64_format")]
    pub charge_amount: f64,
}

#[derive(Serialize, Clone, Debug)]
pub struct NetPriceProductTradePrice {
    #[serde(rename="ram:ChargeAmount",serialize_with="f64_format")]
    pub charge_amount: f64,
}

#[derive(Serialize, Clone, Debug)]
pub struct SpecifiedLineTradeDelivery {
    #[serde(rename="ram:BilledQuantity")]
    pub billed_quantity: BilledQuantity,
}

#[derive(Serialize, Clone, Debug)]
pub struct BilledQuantity {
    #[serde(rename="@unitCode")]
    pub unit_code: UnitCode,
    #[serde(rename="$value",serialize_with="f64_format_with_precision_4")]
    pub value: f64,
}

impl BilledQuantity {
    pub fn new(unit_code: UnitCode, value: f64) -> Self {
        Self {
            unit_code,
            value,
        }
    }
    
}

#[derive(Serialize, Clone, Debug)]
pub struct SpecifiedLineTradeSettlement<'invoice> {
    #[serde(rename="ram:ApplicableTradeTax")]
    pub applicable_trade_tax: ApplicableTradeTax<'invoice>,
    #[serde(rename="ram:SpecifiedTradeSettlementLineMonetarySummation")]
    pub specified_trade_settlement_line_monetary_summation: SpecifiedTradeSettlementLineMonetarySummation,
}

#[derive(Serialize, Clone, Debug)]
pub struct SpecifiedTradeSettlementLineMonetarySummation {
    #[serde(rename="ram:LineTotalAmount", serialize_with="f64_format")]
    pub line_total_amount: f64,
}

#[derive(Serialize, Clone, Debug)]
pub struct ApplicableHeaderTradeAgreement<'invoice> {
    #[serde(rename="ram:BuyerReference", skip_serializing_if = "Option::is_none")]
    pub buyer_reference: Option<&'invoice str>,
    #[serde(rename="ram:SellerTradeParty")]
    pub seller_trade_party: SellerTradeParty<'invoice>,
    #[serde(rename="ram:BuyerTradeParty")]
    pub buyer_trade_party: BuyerTradeParty<'invoice>,
    #[serde(rename="ram:BuyerOrderReferencedDocument")]
    pub buyer_order_referenced_document: BuyerOrderReferencedDocument<'invoice>,
}

#[derive(Serialize, Clone, Debug)]
pub struct SellerTradeParty<'invoice> {
    #[serde(rename="ram:Name")]
    pub name: &'invoice str,
    #[serde(rename="ram:SpecifiedLegalOrganization")]
    pub specified_legal_organization: SpecifiedLegalOrganization<'invoice>,
    #[serde(rename="ram:PostalTradeAddress")]
    pub postal_trade_address: PostalTradeAddress<'invoice>,
    #[serde(rename="ram:SpecifiedTaxRegistration")]
    pub specified_tax_registration: SpecifiedTaxRegistration<'invoice>,
}

#[derive(Serialize, Clone, Debug)]
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

#[derive(Serialize, Clone, Debug)]
pub struct SpecifiedLegalOrganization<'invoice> {
    #[serde(rename="ram:ID")]
    pub id: LegalOrganizationID<'invoice>,
}

#[derive(Serialize, Clone, Debug)]
pub struct PostalTradeAddress <'invoice> {

    #[serde(rename="ram:PostcodeCode", skip_serializing_if = "Option::is_none")]
    pub postcode_code: Option<&'invoice str>,
    #[serde(rename="ram:LineOne", skip_serializing_if = "Option::is_none")]
    pub line_one: Option<&'invoice str>,
    #[serde(rename="ram:LineTwo", skip_serializing_if = "Option::is_none")]
    pub line_two: Option<&'invoice str>,
    #[serde(rename="ram:LineThree", skip_serializing_if = "Option::is_none")]
    pub line_three: Option<&'invoice str>,
    #[serde(rename="ram:CityName", skip_serializing_if = "Option::is_none")]
    pub city_name: Option<&'invoice str>,

    #[serde(rename="ram:CountryID")]
    pub country_id: CountryCode,
}

impl<'invoice> Default for PostalTradeAddress<'invoice> {
    fn default() -> Self {
        Self {
            postcode_code: None,
            line_one: None,
            line_two: None,
            line_three: None,
            city_name: None,
            country_id: CountryCode::NotSet,
        }
    }
}

#[derive(Serialize, Clone, Debug)]
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

#[derive(Serialize, Clone, Debug)]
pub struct SpecifiedTaxRegistration<'invoice> {
    #[serde(rename="ram:ID")]
    pub id: SpecifiedTaxRegistrationID<'invoice>,
}

#[derive(Serialize, Clone, Debug)]
pub struct BuyerTradeParty<'invoice> {
    #[serde(rename="ram:Name")]
    pub name: &'invoice str,
    #[serde(rename="ram:PostalTradeAddress")]
    pub postal_trade_address: PostalTradeAddress<'invoice>,
    #[serde(rename="ram:SpecifiedLegalOrganization", skip_serializing_if = "Option::is_none")]
    pub specified_legal_organization: Option<SpecifiedLegalOrganization<'invoice>>,
}

#[derive(Serialize, Clone, Debug)]
pub struct BuyerOrderReferencedDocument<'invoice> {
    #[serde(rename="ram:IssuerAssignedID")]
    pub issuer_assigned_id: &'invoice str,
}

#[derive(Serialize, Clone, Debug)]
pub struct ApplicableHeaderTradeDelivery<'invoice> {
    #[serde(rename="ram:ActualDeliverySupplyChainEvent", skip_serializing_if = "Option::is_none")]
    pub actual_delivery_supply_chain_event: Option<ActualDeliverySupplyChainEvent<'invoice>>,
}

#[derive(Serialize, Clone, Debug)]
pub struct ActualDeliverySupplyChainEvent<'invoice> {
    #[serde(rename="ram:OccurrenceDateTime", skip_serializing_if = "Option::is_none")]
    pub occurrence_date_time: Option<OccurrenceDateTime<'invoice>>,
}

#[derive(Serialize, Clone, Debug)]
pub struct OccurrenceDateTime<'invoice> {
    #[serde(rename="udt:DateTimeString")]
    pub actual_delivery_date: DateTimeString<'invoice>,
}

impl<'invoice> ApplicableHeaderTradeDelivery<'invoice> {
    pub fn new_with_event(occurrence_date_time: Option<DateTimeString<'invoice>>) -> Self {
        Self {
            actual_delivery_supply_chain_event: if occurrence_date_time.is_some() {
                Some(ActualDeliverySupplyChainEvent {
                    occurrence_date_time: occurrence_date_time.map(|actual_delivery_date| OccurrenceDateTime {
                        actual_delivery_date,
                    })
                })
            } else {
                None
            }
        }
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct ApplicableHeaderTradeSettlement <'invoice>{
    #[serde(rename="ram:InvoiceCurrencyCode")]
    pub invoice_currency_code: CurrencyCode,
    #[serde(rename="ram:ApplicableTradeTax", skip_serializing_if = "Option::is_none")]
    pub applicable_trade_tax: Option<ApplicableTradeTax<'invoice>>,
    #[serde(rename="ram:SpecifiedTradePaymentTerms", skip_serializing_if = "Option::is_none")]
    pub specified_trade_payment_terms: Option<SpecifiedTradePaymentTerms<'invoice>>,
    #[serde(rename="ram:SpecifiedTradeSettlementHeaderMonetarySummation")]
    pub specified_trade_settlement_header_monetary_summation: SpecifiedTradeSettlementHeaderMonetarySummation,
}   

#[derive(Serialize, Clone, Copy, Debug)]
pub struct ApplicableTradeTax <'invoice> {
    #[serde(rename="ram:CalculatedAmount",serialize_with="format_f64_option", skip_serializing_if = "Option::is_none")]
    pub calculated_amount: Option<f64>,
    #[serde(rename="ram:TypeCode")]
    pub type_code: &'invoice str,
    #[serde(rename="ram:BasisAmount",serialize_with="format_f64_option", skip_serializing_if = "Option::is_none")]
    pub basis_amount: Option<f64>,
    #[serde(rename="ram:CategoryCode")]
    pub category_code: VATCategoryCode,
    #[serde(rename="ram:RateApplicablePercent",serialize_with="format_f64_option", skip_serializing_if = "Option::is_none")]
    pub rate_applicable_percent: Option<f64>,

}

impl<'invoice> Default for ApplicableTradeTax<'invoice> {
    fn default() -> Self {
        Self {
            calculated_amount: None,
            type_code: "VAT",
            basis_amount: None,
            category_code: VATCategoryCode::StandardRate,
            rate_applicable_percent: None,
        }
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct SpecifiedTradePaymentTerms <'invoice> {
    /// `BT-20`: A textual description of the payment terms that apply to the amount due for payment (Including description of possible penalties).
    #[serde(rename="ram:Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<&'invoice str>,
    /// `BT-9-00`: The date when the payment is due.
    #[serde(rename="ram:DueDateDateTime", skip_serializing_if = "Option::is_none")]
    pub due_date_time: Option<DueDateDateTime<'invoice>>,
}

#[derive(Serialize, Clone, Debug)]
pub struct DueDateDateTime<'invoice> {
    /// `BT-9`: The date when the payment is due.
    #[serde(rename="udt:DateTimeString")]
    pub payment_due_date: DateTimeString<'invoice>,
}

/// `BG-22`: A group of business terms providing the monetary totals for the Invoice.
#[derive(Serialize, Clone, Debug)]
pub struct SpecifiedTradeSettlementHeaderMonetarySummation {
    /// `BT-106`: Sum of all Invoice line net amounts in the Invoice.
    #[serde(rename="ram:LineTotalAmount", serialize_with="format_f64_option", skip_serializing_if = "Option::is_none")]
    pub line_total_amount: Option<f64>,
    /// `BT-108`: Sum of all charges on document level in the Invoice.
    #[serde(rename="ram:ChargeTotalAmount", serialize_with="format_f64_option", skip_serializing_if = "Option::is_none")]
    pub charge_total_amount: Option<f64>,
    /// `BT-107`: Sum of all allowances on document level in the Invoice.
    #[serde(rename="ram:AllowanceTotalAmount", serialize_with="format_f64_option", skip_serializing_if = "Option::is_none")]
    pub allowance_total_amount: Option<f64>,
    

    //Required for minimum specification
    
    /// `BT-109`: The total amount of the Invoice without VAT.
    #[serde(rename="ram:TaxBasisTotalAmount",serialize_with="format_f64_option")]
    pub tax_basis_total_amount: Option<f64>,
    /// `BT-110`: The total VAT amount for the Invoice.
    #[serde(rename="ram:TaxTotalAmount", skip_serializing_if = "Option::is_none")]
    pub tax_total_amount: Option<TaxTotalAmount>,
    /// `BT-112`: The total amount of the Invoice with VAT.
    #[serde(rename="ram:GrandTotalAmount",serialize_with="format_f64_option")]
    pub grand_total_amount: Option<f64>,
    /// `BT-115`: The outstanding amount that is requested to be paid.
    #[serde(rename="ram:DuePayableAmount",serialize_with="format_f64_option")]
    pub due_payable_amount: Option<f64>,
}

impl Default for SpecifiedTradeSettlementHeaderMonetarySummation {
    fn default() -> Self {
        Self {
            line_total_amount: None,
            charge_total_amount: None,
            allowance_total_amount: None,
            tax_basis_total_amount: None,
            tax_total_amount: None,
            grand_total_amount: None,
            due_payable_amount: None,
        }
    }
}

#[derive(Serialize, Clone, Debug)]
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