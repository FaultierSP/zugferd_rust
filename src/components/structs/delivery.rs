use super::context_document::DateTimeString;
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
