use super::agreement::{AddressView, PostalAddressDetails, ReferenceView};
use super::context_document::DateTimeString;
use super::invoice::AdditionalInvoiceDetails;
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct ApplicableHeaderTradeDelivery<'invoice> {
    #[serde(
        rename = "ram:ActualDeliverySupplyChainEvent",
        skip_serializing_if = "Option::is_none"
    )]
    pub actual_delivery_supply_chain_event: Option<ActualDeliverySupplyChainEvent<'invoice>>,
}

#[derive(Serialize, Clone, Debug)]
pub struct ActualDeliverySupplyChainEvent<'invoice> {
    #[serde(
        rename = "ram:OccurrenceDateTime",
        skip_serializing_if = "Option::is_none"
    )]
    pub occurrence_date_time: Option<OccurrenceDateTime<'invoice>>,
}

#[derive(Serialize, Clone, Debug)]
pub struct OccurrenceDateTime<'invoice> {
    #[serde(rename = "udt:DateTimeString")]
    pub actual_delivery_date: DateTimeString<'invoice>,
}

impl<'invoice> ApplicableHeaderTradeDelivery<'invoice> {
    pub fn new_with_event(occurrence_date_time: Option<DateTimeString<'invoice>>) -> Self {
        Self {
            actual_delivery_supply_chain_event: if occurrence_date_time.is_some() {
                Some(ActualDeliverySupplyChainEvent {
                    occurrence_date_time: occurrence_date_time.map(|actual_delivery_date| {
                        OccurrenceDateTime {
                            actual_delivery_date,
                        }
                    }),
                })
            } else {
                None
            },
        }
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct ShipTo<'a> {
    pub address: PostalAddressDetails<'a>,
}

#[derive(Serialize)]
struct ShipToView<'s, 'a> {
    #[serde(rename = "ram:PostalTradeAddress")]
    address: AddressView<'s, 'a>,
}

#[derive(Serialize)]
pub(super) struct DeliveryView<'s, 'a> {
    #[serde(
        rename = "ram:ShipToTradeParty",
        skip_serializing_if = "Option::is_none"
    )]
    ship_to: Option<ShipToView<'s, 'a>>,
    #[serde(
        rename = "ram:ActualDeliverySupplyChainEvent",
        skip_serializing_if = "Option::is_none"
    )]
    event: &'s Option<ActualDeliverySupplyChainEvent<'a>>,
    #[serde(
        rename = "ram:ReceivingAdviceReferencedDocument",
        skip_serializing_if = "Option::is_none"
    )]
    receiving_advice: Option<ReferenceView<'a>>,
}

impl<'s, 'a> DeliveryView<'s, 'a> {
    pub(super) fn new(
        base: &'s ApplicableHeaderTradeDelivery<'a>,
        details: &'s AdditionalInvoiceDetails<'a>,
    ) -> Self {
        Self {
            ship_to: details.ship_to.as_ref().map(|ship| ShipToView {
                address: AddressView::new(&ship.address.address, ship.address.region),
            }),
            event: &base.actual_delivery_supply_chain_event,
            receiving_advice: details
                .receiving_advice_reference
                .map(|id| ReferenceView::id(id, None, None)),
        }
    }
}
