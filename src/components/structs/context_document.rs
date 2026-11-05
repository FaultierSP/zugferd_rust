use crate::components::constants;
use crate::components::enums::{
    invoice_type_code::InvoiceTypeCode, specification_level::SpecificationLevel,
};
use chrono::NaiveDate;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct DocumentContext<'invoice> {
    #[serde(
        rename = "ram:BusinessProcessSpecifiedDocumentContextParameter",
        skip_serializing_if = "Option::is_none"
    )]
    pub business_process: Option<BusinessProcess<'invoice>>,
    #[serde(rename = "ram:GuidelineSpecifiedDocumentContextParameter")]
    pub guideline: Guideline,
}

#[derive(Serialize, Debug)]
pub struct Document<'invoice> {
    #[serde(rename = "ram:ID")]
    pub id: &'invoice str,
    #[serde(rename = "ram:TypeCode")]
    pub type_code: InvoiceTypeCode,
    #[serde(rename = "ram:IssueDateTime")]
    pub issue_date_time: IssueDateTime<'invoice>,
    #[serde(rename = "ram:IncludedNote", skip_serializing_if = "Option::is_none")]
    pub included_note: Option<Vec<IncludedNote>>,
}

#[derive(Serialize, Clone, Debug)]
pub struct BusinessProcess<'invoice> {
    #[serde(rename = "ram:ID")]
    pub id: &'invoice str,
}

#[derive(Serialize, Clone, Debug)]
pub struct Guideline {
    #[serde(rename = "ram:ID")]
    pub id: SpecificationLevel,
}

#[derive(Serialize, Clone, Debug)]
pub struct IssueDateTime<'invoice> {
    #[serde(rename = "udt:DateTimeString")]
    pub date_time_string: DateTimeString<'invoice>,
}

#[derive(Serialize, Clone, Debug)]
pub struct DateTimeString<'invoice> {
    #[serde(rename = "@format")]
    format: &'invoice str,
    #[serde(rename = "$value")]
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
impl std::fmt::Display for DateTimeString<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let date = NaiveDate::parse_from_str(&self.value, constants::DATE_TIME_FORMAT_102).unwrap();
        write!(f, "{}", date.format("%Y-%m-%d"))?;
        Ok(())
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct IncludedNote {
    #[serde(rename = "ram:Content")]
    pub content: String,
}
