use super::line_item::GlobalID;
use crate::components::enums::country_code::CountryCode;
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct ApplicableHeaderTradeAgreement<'invoice> {
    #[serde(rename = "ram:BuyerReference", skip_serializing_if = "Option::is_none")]
    pub buyer_reference: Option<&'invoice str>,
    #[serde(rename = "ram:SellerTradeParty")]
    pub seller_trade_party: SellerTradeParty<'invoice>,
    #[serde(rename = "ram:BuyerTradeParty")]
    pub buyer_trade_party: BuyerTradeParty<'invoice>,
    #[serde(
        rename = "ram:BuyerOrderReferencedDocument",
        skip_serializing_if = "Option::is_none"
    )]
    pub buyer_order_referenced_document: Option<BuyerOrderReferencedDocument<'invoice>>,
}

#[derive(Serialize, Clone, Debug)]
pub struct SellerTradeParty<'invoice> {
    #[serde(rename = "ram:ID")]
    pub id: Vec<&'invoice str>,
    #[serde(rename = "ram:GlobalID")]
    pub global_id: Vec<GlobalID<'invoice>>,
    #[serde(rename = "ram:Name")]
    pub name: &'invoice str,
    #[serde(
        rename = "ram:SpecifiedLegalOrganization",
        skip_serializing_if = "Option::is_none"
    )]
    pub specified_legal_organization: Option<SpecifiedLegalOrganization<'invoice>>,
    #[serde(rename = "ram:PostalTradeAddress")]
    pub postal_trade_address: PostalTradeAddress<'invoice>,
    #[serde(
        rename = "ram:URIUniversalCommunication",
        skip_serializing_if = "Option::is_none"
    )]
    /// BT-34-00
    pub uri_universal_communication: Option<URIUniversalCommunication<'invoice>>,
    #[serde(rename = "ram:SpecifiedTaxRegistration")]
    pub specified_tax_registration: Vec<SpecifiedTaxRegistration<'invoice>>,
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
    #[serde(rename = "ram:ID")]
    pub id: LegalOrganizationID<'invoice>,
}

#[derive(Serialize, Clone, Debug)]
pub struct PostalTradeAddress<'invoice> {
    #[serde(rename = "ram:PostcodeCode", skip_serializing_if = "Option::is_none")]
    pub postcode_code: Option<&'invoice str>,
    #[serde(rename = "ram:LineOne", skip_serializing_if = "Option::is_none")]
    pub line_one: Option<&'invoice str>,
    #[serde(rename = "ram:LineTwo", skip_serializing_if = "Option::is_none")]
    pub line_two: Option<&'invoice str>,
    #[serde(rename = "ram:LineThree", skip_serializing_if = "Option::is_none")]
    pub line_three: Option<&'invoice str>,
    #[serde(rename = "ram:CityName", skip_serializing_if = "Option::is_none")]
    pub city_name: Option<&'invoice str>,

    #[serde(rename = "ram:CountryID")]
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
    pub fn new_fc(value: &'invoice str) -> Self {
        Self {
            scheme_id: "FC",
            value,
        }
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct SpecifiedTaxRegistration<'invoice> {
    #[serde(rename = "ram:ID")]
    pub id: SpecifiedTaxRegistrationID<'invoice>,
}

#[derive(Serialize, Clone, Debug)]
pub struct URIUniversalCommunication<'invoice> {
    #[serde(rename = "ram:URIID")]
    pub uriid: UriId<'invoice>,
}

#[derive(Serialize, Clone, Debug)]
pub struct UriId<'invoice> {
    #[serde(rename = "@schemeID")]
    pub scheme_id: &'invoice str,
    #[serde(rename = "$value")]
    pub value: &'invoice str,
}

#[derive(Serialize, Clone, Debug)]
pub struct BuyerTradeParty<'invoice> {
    #[serde(rename = "ram:Name")]
    pub name: &'invoice str,
    #[serde(
        rename = "ram:SpecifiedLegalOrganization",
        skip_serializing_if = "Option::is_none"
    )]
    pub specified_legal_organization: Option<SpecifiedLegalOrganization<'invoice>>,
    #[serde(rename = "ram:PostalTradeAddress")]
    pub postal_trade_address: PostalTradeAddress<'invoice>,
}

#[derive(Serialize, Clone, Debug)]
pub struct BuyerOrderReferencedDocument<'invoice> {
    #[serde(rename = "ram:IssuerAssignedID")]
    pub issuer_assigned_id: &'invoice str,
}
