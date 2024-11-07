use serde::{Serialize, Serializer};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum InvoiceTypeCode {
    CommercialInvoice,
    CreditNote,
    CorrectedInvoice,
    SelfBilledInvoice,
    SelfBilledCreditNote,
    PrepaymentInvoice,
    InvoiceInformationForAccountingPurposes,
}

impl InvoiceTypeCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            InvoiceTypeCode::CommercialInvoice => "380",
            InvoiceTypeCode::CreditNote => "381",
            InvoiceTypeCode::CorrectedInvoice => "384",
            InvoiceTypeCode::SelfBilledInvoice => "389",
            InvoiceTypeCode::SelfBilledCreditNote => "261",
            InvoiceTypeCode::PrepaymentInvoice => "386",
            InvoiceTypeCode::InvoiceInformationForAccountingPurposes => "751",
        }
    }
}

impl Serialize for InvoiceTypeCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
