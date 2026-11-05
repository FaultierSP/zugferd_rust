use super::context_document::Document;
use super::context_document::DocumentContext;
use super::line_item::SupplyChainTradeTransaction;
use super::xml_cleanup::remove_empty_elements;
use serde::Serialize;

//Specifications
#[derive(Serialize, Debug)]
#[serde(rename = "rsm:CrossIndustryInvoice")]
pub struct Invoice<'invoice> {
    //Namespaces
    #[serde(rename = "@xmlns:xsi")]
    xmlns_xsi: &'static str,
    #[serde(rename = "@xmlns:qdt")]
    xmlns_qdt: &'static str,
    #[serde(rename = "@xmlns:udt")]
    xmlns_udt: &'static str,
    #[serde(rename = "@xmlns:rsm")]
    xmlns_rsm: &'static str,
    #[serde(rename = "@xmlns:ram")]
    xmlns_ram: &'static str,

    //Document
    #[serde(rename = "rsm:ExchangedDocumentContext")]
    pub context: DocumentContext<'invoice>,
    #[serde(rename = "rsm:ExchangedDocument")]
    pub document: Document<'invoice>,

    //Supply chain trade transaction
    #[serde(rename = "rsm:SupplyChainTradeTransaction")]
    pub supply_chain_trade_transaction: SupplyChainTradeTransaction<'invoice>,
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
        }
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
