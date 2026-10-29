use super::InvoiceBuilder;
use crate::components::structs::IncludedSupplyChainTradeLineItem;

impl<'invoice_builder> InvoiceBuilder<'invoice_builder> {
    pub fn add_supply_chain_trade_line_item(
        &mut self,
        line_item: IncludedSupplyChainTradeLineItem<'invoice_builder>,
    ) -> &mut Self {
        self.included_supply_chain_trade_line_items.push(line_item);
        self
    }
}
