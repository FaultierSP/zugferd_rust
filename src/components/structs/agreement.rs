use super::invoice::AdditionalInvoiceDetails;
use super::line_item::GlobalID;
use crate::components::enums::country_code::CountryCode;
use base64::{engine::general_purpose::STANDARD, Engine};
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

/// Composite struct for trade contact information (person, department, phone)
#[derive(Serialize, Clone, Copy, Debug)]
pub struct DefinedTradeContact<'invoice> {
    #[serde(rename = "ram:PersonName", skip_serializing_if = "Option::is_none")]
    pub person_name: Option<&'invoice str>,
    #[serde(rename = "ram:DepartmentName", skip_serializing_if = "Option::is_none")]
    pub department_name: Option<&'invoice str>,
    #[serde(rename = "ram:CompleteNumber", skip_serializing_if = "Option::is_none")]
    pub complete_number: Option<&'invoice str>,
}

#[derive(Serialize, Clone, Debug)]
pub struct BuyerOrderReferencedDocument<'invoice> {
    #[serde(rename = "ram:IssuerAssignedID")]
    pub issuer_assigned_id: &'invoice str,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct TradeContact<'a> {
    #[serde(rename = "ram:PersonName", skip_serializing_if = "Option::is_none")]
    pub person_name: Option<&'a str>,
    #[serde(rename = "ram:DepartmentName", skip_serializing_if = "Option::is_none")]
    pub department_name: Option<&'a str>,
    #[serde(rename = "ram:CompleteNumber", skip_serializing_if = "Option::is_none")]
    pub telephone: Option<&'a str>,
    #[serde(rename = "ram:Email", skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
}

#[derive(Serialize, Clone, Debug)]
pub struct PostalAddressDetails<'a> {
    pub address: PostalTradeAddress<'a>,
    pub region: Option<&'a str>,
}

#[derive(Serialize, Clone, Debug)]
pub struct TaxRepresentative<'a> {
    pub name: &'a str,
    pub vat_id: &'a str,
    pub address: PostalAddressDetails<'a>,
}

#[derive(Serialize, Clone, Debug)]
pub struct ProjectReference<'a> {
    pub id: &'a str,
    pub name: &'a str,
}

#[derive(Serialize, Clone, Debug)]
pub struct ObjectReference<'a> {
    pub id: &'a str,
    /// UNTDID 1153 reference type, serialized as `ram:ReferenceTypeCode`.
    pub scheme_id: Option<&'a str>,
}

#[derive(Serialize, Clone, Debug)]
pub struct DocumentAttachment<'a> {
    pub content: &'a [u8],
    /// One of the MIME types permitted by the EN 16931 profile.
    pub mime_code: &'a str,
    pub filename: &'a str,
}

#[derive(Serialize, Clone, Debug)]
pub struct SupportingDocument<'a> {
    pub id: &'a str,
    pub description: Option<&'a str>,
    pub uri: Option<&'a str>,
    pub attachment: Option<DocumentAttachment<'a>>,
}

#[derive(Serialize)]
pub(super) struct AddressView<'s, 'a> {
    #[serde(rename = "ram:PostcodeCode", skip_serializing_if = "Option::is_none")]
    postcode: Option<&'a str>,
    #[serde(rename = "ram:LineOne", skip_serializing_if = "Option::is_none")]
    line_one: Option<&'a str>,
    #[serde(rename = "ram:LineTwo", skip_serializing_if = "Option::is_none")]
    line_two: Option<&'a str>,
    #[serde(rename = "ram:LineThree", skip_serializing_if = "Option::is_none")]
    line_three: Option<&'a str>,
    #[serde(rename = "ram:CityName", skip_serializing_if = "Option::is_none")]
    city: Option<&'a str>,
    #[serde(rename = "ram:CountryID")]
    country: &'s CountryCode,
    #[serde(
        rename = "ram:CountrySubDivisionName",
        skip_serializing_if = "Option::is_none"
    )]
    region: Option<&'a str>,
}

impl<'s, 'a> AddressView<'s, 'a> {
    pub(super) fn new(address: &'s PostalTradeAddress<'a>, region: Option<&'a str>) -> Self {
        Self {
            postcode: address.postcode_code,
            line_one: address.line_one,
            line_two: address.line_two,
            line_three: address.line_three,
            city: address.city_name,
            country: &address.country_id,
            region,
        }
    }
}

#[derive(Serialize)]
struct Communication<'a> {
    #[serde(rename = "ram:CompleteNumber", skip_serializing_if = "Option::is_none")]
    telephone: Option<&'a str>,
    #[serde(rename = "ram:URIID", skip_serializing_if = "Option::is_none")]
    email: Option<&'a str>,
}

#[derive(Serialize)]
struct ContactView<'a> {
    #[serde(rename = "ram:PersonName", skip_serializing_if = "Option::is_none")]
    person_name: Option<&'a str>,
    #[serde(rename = "ram:DepartmentName", skip_serializing_if = "Option::is_none")]
    department_name: Option<&'a str>,
    #[serde(
        rename = "ram:TelephoneUniversalCommunication",
        skip_serializing_if = "Option::is_none"
    )]
    telephone: Option<Communication<'a>>,
    #[serde(
        rename = "ram:EmailURIUniversalCommunication",
        skip_serializing_if = "Option::is_none"
    )]
    email: Option<Communication<'a>>,
}

impl<'a> From<&TradeContact<'a>> for ContactView<'a> {
    fn from(contact: &TradeContact<'a>) -> Self {
        Self {
            person_name: contact.person_name,
            department_name: contact.department_name,
            telephone: contact.telephone.map(|value| Communication {
                telephone: Some(value),
                email: None,
            }),
            email: contact.email.map(|value| Communication {
                telephone: None,
                email: Some(value),
            }),
        }
    }
}

#[derive(Serialize)]
struct SellerView<'s, 'a> {
    #[serde(rename = "ram:ID")]
    id: &'s Vec<&'a str>,
    #[serde(rename = "ram:GlobalID")]
    global_id: &'s Vec<GlobalID<'a>>,
    #[serde(rename = "ram:Name")]
    name: &'a str,
    #[serde(rename = "ram:Description", skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(
        rename = "ram:SpecifiedLegalOrganization",
        skip_serializing_if = "Option::is_none"
    )]
    organization: &'s Option<SpecifiedLegalOrganization<'a>>,
    #[serde(
        rename = "ram:DefinedTradeContact",
        skip_serializing_if = "Option::is_none"
    )]
    contact: Option<ContactView<'a>>,
    #[serde(rename = "ram:PostalTradeAddress")]
    address: AddressView<'s, 'a>,
    #[serde(
        rename = "ram:URIUniversalCommunication",
        skip_serializing_if = "Option::is_none"
    )]
    communication: &'s Option<URIUniversalCommunication<'a>>,
    #[serde(rename = "ram:SpecifiedTaxRegistration")]
    tax_registration: &'s Vec<SpecifiedTaxRegistration<'a>>,
}

#[derive(Serialize)]
struct BuyerView<'s, 'a> {
    #[serde(rename = "ram:Name")]
    name: &'a str,
    #[serde(
        rename = "ram:SpecifiedLegalOrganization",
        skip_serializing_if = "Option::is_none"
    )]
    organization: &'s Option<SpecifiedLegalOrganization<'a>>,
    #[serde(
        rename = "ram:DefinedTradeContact",
        skip_serializing_if = "Option::is_none"
    )]
    contact: Option<ContactView<'a>>,
    #[serde(rename = "ram:PostalTradeAddress")]
    address: AddressView<'s, 'a>,
}

#[derive(Serialize)]
struct RepresentativeView<'s, 'a> {
    #[serde(rename = "ram:Name")]
    name: &'a str,
    #[serde(rename = "ram:PostalTradeAddress")]
    address: AddressView<'s, 'a>,
    #[serde(rename = "ram:SpecifiedTaxRegistration")]
    tax_registration: SpecifiedTaxRegistration<'a>,
}

#[derive(Serialize)]
struct Attachment<'a> {
    #[serde(rename = "@mimeCode")]
    mime: &'a str,
    #[serde(rename = "@filename")]
    filename: &'a str,
    #[serde(rename = "$value")]
    data: String,
}

#[derive(Serialize)]
pub(super) struct ReferenceView<'a> {
    #[serde(
        rename = "ram:IssuerAssignedID",
        skip_serializing_if = "Option::is_none"
    )]
    id: Option<&'a str>,
    #[serde(rename = "ram:URIID", skip_serializing_if = "Option::is_none")]
    uri: Option<&'a str>,
    #[serde(rename = "ram:LineID", skip_serializing_if = "Option::is_none")]
    line_id: Option<&'a str>,
    #[serde(rename = "ram:TypeCode", skip_serializing_if = "Option::is_none")]
    type_code: Option<&'static str>,
    #[serde(rename = "ram:Name", skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(
        rename = "ram:AttachmentBinaryObject",
        skip_serializing_if = "Option::is_none"
    )]
    attachment: Option<Attachment<'a>>,
    #[serde(
        rename = "ram:ReferenceTypeCode",
        skip_serializing_if = "Option::is_none"
    )]
    reference_type_code: Option<&'a str>,
}

impl<'a> ReferenceView<'a> {
    pub(super) fn id(
        id: &'a str,
        type_code: Option<&'static str>,
        scheme_id: Option<&'a str>,
    ) -> Self {
        Self {
            id: Some(id),
            uri: None,
            line_id: None,
            type_code,
            name: None,
            attachment: None,
            reference_type_code: scheme_id,
        }
    }
    pub(super) fn line(id: &'a str) -> Self {
        Self {
            id: None,
            uri: None,
            line_id: Some(id),
            type_code: None,
            name: None,
            attachment: None,
            reference_type_code: None,
        }
    }
    fn supporting(document: &SupportingDocument<'a>) -> Self {
        Self {
            id: Some(document.id),
            uri: document.uri,
            line_id: None,
            type_code: Some("916"),
            name: document.description,
            attachment: document.attachment.as_ref().map(|attachment| Attachment {
                mime: attachment.mime_code,
                filename: attachment.filename,
                data: STANDARD.encode(attachment.content),
            }),
            reference_type_code: None,
        }
    }
}

#[derive(Serialize)]
struct ProjectView<'a> {
    #[serde(rename = "ram:ID")]
    id: &'a str,
    #[serde(rename = "ram:Name")]
    name: &'a str,
}

#[derive(Serialize)]
pub(super) struct AgreementView<'s, 'a> {
    #[serde(rename = "ram:BuyerReference", skip_serializing_if = "Option::is_none")]
    buyer_reference: Option<&'a str>,
    #[serde(rename = "ram:SellerTradeParty")]
    seller: SellerView<'s, 'a>,
    #[serde(rename = "ram:BuyerTradeParty")]
    buyer: BuyerView<'s, 'a>,
    #[serde(
        rename = "ram:SellerTaxRepresentativeTradeParty",
        skip_serializing_if = "Option::is_none"
    )]
    representative: Option<RepresentativeView<'s, 'a>>,
    #[serde(
        rename = "ram:SellerOrderReferencedDocument",
        skip_serializing_if = "Option::is_none"
    )]
    seller_order: Option<ReferenceView<'a>>,
    #[serde(
        rename = "ram:BuyerOrderReferencedDocument",
        skip_serializing_if = "Option::is_none"
    )]
    buyer_order: &'s Option<BuyerOrderReferencedDocument<'a>>,
    #[serde(rename = "ram:AdditionalReferencedDocument")]
    references: Vec<ReferenceView<'a>>,
    #[serde(
        rename = "ram:SpecifiedProcuringProject",
        skip_serializing_if = "Option::is_none"
    )]
    project: Option<ProjectView<'a>>,
}

impl<'s, 'a> AgreementView<'s, 'a> {
    pub(super) fn new(
        base: &'s ApplicableHeaderTradeAgreement<'a>,
        details: &'s AdditionalInvoiceDetails<'a>,
    ) -> Self {
        let seller = &base.seller_trade_party;
        let buyer = &base.buyer_trade_party;
        let mut references = Vec::new();
        if let Some(id) = details.tender_reference {
            references.push(ReferenceView::id(id, Some("50"), None));
        }
        if let Some(object) = &details.object_reference {
            references.push(ReferenceView::id(object.id, Some("130"), object.scheme_id));
        }
        references.extend(
            details
                .supporting_documents
                .iter()
                .map(ReferenceView::supporting),
        );
        Self {
            buyer_reference: base.buyer_reference,
            seller: SellerView {
                id: &seller.id,
                global_id: &seller.global_id,
                name: seller.name,
                description: details.seller_description,
                organization: &seller.specified_legal_organization,
                contact: details.seller_contact.as_ref().map(ContactView::from),
                address: AddressView::new(&seller.postal_trade_address, details.seller_region),
                communication: &seller.uri_universal_communication,
                tax_registration: &seller.specified_tax_registration,
            },
            buyer: BuyerView {
                name: buyer.name,
                organization: &buyer.specified_legal_organization,
                contact: details.buyer_contact.as_ref().map(ContactView::from),
                address: AddressView::new(&buyer.postal_trade_address, details.buyer_region),
            },
            representative: details
                .tax_representative
                .as_ref()
                .map(|rep| RepresentativeView {
                    name: rep.name,
                    address: AddressView::new(&rep.address.address, rep.address.region),
                    tax_registration: SpecifiedTaxRegistration {
                        id: SpecifiedTaxRegistrationID::new(rep.vat_id),
                    },
                }),
            seller_order: details
                .seller_order_reference
                .map(|id| ReferenceView::id(id, None, None)),
            buyer_order: &base.buyer_order_referenced_document,
            references,
            project: details.project.as_ref().map(|p| ProjectView {
                id: p.id,
                name: p.name,
            }),
        }
    }
}
