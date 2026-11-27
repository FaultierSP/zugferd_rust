use super::InvoiceBuilder;
use crate::components::structs::{InvoiceLineDetails, IncludedSupplyChainTradeLineItem};

impl<'invoice_builder> InvoiceBuilder<'invoice_builder> {
    /// Sets one EN 16931 extension per invoice line, in invoice-line order.
    pub fn set_line_details(
        &mut self,
        details: Vec<InvoiceLineDetails<'invoice_builder>>,
    ) -> &mut Self {
        self.line_details = details;
        self
    }

    pub fn add_supply_chain_trade_line_item(
        &mut self,
        line_item: IncludedSupplyChainTradeLineItem<'invoice_builder>,
    ) -> &mut Self {
        self.included_supply_chain_trade_line_items.push(line_item);
        self
    }
}
