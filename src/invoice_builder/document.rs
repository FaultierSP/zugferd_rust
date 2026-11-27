use super::InvoiceBuilder;
use crate::components::enums::invoice_type_code::InvoiceTypeCode;
use crate::components::structs::{
    DateTimeString, IncludedNote, ObjectReference, ProjectReference, SupportingDocument,
};
use chrono::NaiveDate;

impl<'invoice_builder> InvoiceBuilder<'invoice_builder> {
    pub fn set_seller_order_reference(&mut self, reference: &'invoice_builder str) -> &mut Self {
        self.seller_order_reference = Some(reference);
        self
    }

    pub fn set_tender_reference(&mut self, reference: &'invoice_builder str) -> &mut Self {
        self.tender_reference = Some(reference);
        self
    }

    pub fn set_object_reference(
        &mut self,
        reference: ObjectReference<'invoice_builder>,
    ) -> &mut Self {
        self.object_reference = Some(reference);
        self
    }

    pub fn add_supporting_document(
        &mut self,
        document: SupportingDocument<'invoice_builder>,
    ) -> &mut Self {
        self.supporting_documents.push(document);
        self
    }

    pub fn set_project(&mut self, project: ProjectReference<'invoice_builder>) -> &mut Self {
        self.project = Some(project);
        self
    }

    pub fn set_receiving_advice_reference(
        &mut self,
        reference: &'invoice_builder str,
    ) -> &mut Self {
        self.receiving_advice_reference = Some(reference);
        self
    }

    /// Identifies the business process context in which the transaction appears,
    /// to enable the Buyer to process the Invoice in an appropriate way.
    ///
    /// To be specified by the buyer
    ///
    /// The codes to be used are defined in the CHORUSPRO specifications:
    /// A1 (invoice deposit), A2 (prepaid invoice deposit), ...
    pub fn set_business_process<T: Into<&'invoice_builder str>>(
        &mut self,
        business_process: T,
    ) -> &mut Self {
        self.business_process = Some(business_process.into());
        self
    }

    pub fn set_invoice_type_code(&mut self, invoice_type_code: InvoiceTypeCode) -> &mut Self {
        self.invoice_type_code = Some(invoice_type_code);
        self
    }

    pub fn set_invoice_nr<T: Into<&'invoice_builder str>>(&mut self, invoice_nr: T) -> &mut Self {
        self.invoice_nr = Some(invoice_nr.into());
        self
    }

    pub fn set_date_of_issue(&mut self, date_of_issue: NaiveDate) -> &mut Self {
        self.date_of_issue = Some(DateTimeString::new(date_of_issue));
        self
    }

    pub fn set_invoice_notes<S>(&mut self, notes: Vec<S>) -> &mut Self
    where
        S: AsRef<str>,
    {
        self.document_notes = Some(
            notes
                .into_iter()
                .map(|note| IncludedNote {
                    content: note.as_ref().to_string(),
                })
                .collect(),
        );
        self
    }
}
