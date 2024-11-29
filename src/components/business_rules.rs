//! Validation of business rules

use crate::Invoice;

pub struct BusinessRuleViolation {
    pub rule: String,
    pub message: String,
}

/// List of business rules to validate
const BUSINESS_RULES: &[fn(&Invoice) -> Result<(), BusinessRuleViolation>] = &[
    // br_01,
    // br_02,
    // br_03,
    // br_04,
    // br_05,
    // br_06,
    // br_07,
    // br_08,
    // br_09,
    // br_10,
    // br_11,
    // br_12,
    // br_13,
    // br_14,
    // br_15,
    // br_16,
    // br_17,
    // br_18,
    // br_19,
    // br_20,
    // br_21,
    // br_22,
    // br_23,
    // br_24,
    // br_25,
    // br_26,
    // br_27,
    // br_28,
    // br_29,
    // br_30,
    // br_31,
    // br_32,
    // br_33,
    // br_36,
    // br_37,
    // br_38,
    // br_41,
    // br_42,
    // br_43,
    // br_44,
    // br_45,
    // br_46,
    // br_47,
    // br_48,
    // br_49,
    // br_50,
    // br_51,
    // br_52,
    // br_53,
    // br_54,
    // br_55,
    // br_56,
    // br_57,
    // br_61,
    // br_62,
    // br_63,
    // br_64,
    // br_65,
    // br_co_03,
    // br_co_04,
    // br_co_05,
    // br_co_06,
    // br_co_07,
    // br_co_08,
    // br_co_09,
    // br_co_10,
    // br_co_11,
    // br_co_12,
    // br_co_13,
    br_co_14,
    // br_co_15,
    // br_co_16,
    br_co_17,
    // br_co_18,
    // br_co_19,
    // br_co_20,
    // br_co_21,
    // br_co_22,
    // br_co_23,
    // br_co_24,
    // br_co_25,
    // br_co_26,
];

/// Validate the business rules of a ZUGFeRD invoice
pub fn validate(invoice: &Invoice) -> Vec<BusinessRuleViolation> {
    BUSINESS_RULES.iter()
        .filter_map(|rule| rule(invoice).err())
        .collect()
}

/// Helper macro, checks if two floats are equal (within a margin of 0.01)
/// 
/// If they are not equal, returns a [`BusinessRuleViolation`]
macro_rules! check_float_eq {
    ($rule:expr; $a:expr, $b:expr) => {
        if ($a - $b).abs() > 0.01 {
            let str_a = stringify!($a); //.to_ascii_uppercase().replace('_', "-");
            let str_b = stringify!($b); //.to_ascii_uppercase().replace('_', "-");
            return Err(BusinessRuleViolation {
                rule: $rule.to_string(),
                message: format!("{} = {:.2} != {:.2} = {}", str_a, $a, $b, str_b),
            });
        } else {
            // this allows us to use this as the final statement in a function
            Ok(()) as Result<(), BusinessRuleViolation>
        }
    };
}


/// BR-CO-14: Invoice total VAT amount (BT-110) = ∑ VAT category tax amount (BT-117)
fn br_co_14(invoice: &Invoice) -> Result<(), BusinessRuleViolation> {
    let rule = "BR-CO-14";
    let bt_110 = invoice.supply_chain_trade_transaction.applicable_header_trade_settlement.specified_trade_settlement_header_monetary_summation.tax_total_amount.clone().check(rule)?.amount;
    let bt_117_sum = invoice.supply_chain_trade_transaction.applicable_header_trade_settlement.applicable_trade_tax.iter()
        .filter_map(|bg_23| bg_23.calculated_amount)
        .sum::<f64>();
    check_float_eq!(rule; bt_110, bt_117_sum)
}

/// BR-CO-17: VAT category tax amount (BT-117) = VAT category taxable amount (BT-116) x (VAT category rate (BT-119) / 100), rounded to two decimals
fn br_co_17(invoice: &Invoice) -> Result<(), BusinessRuleViolation> {
    let rule = "BR-CO-17";
    let bg_23 = invoice.supply_chain_trade_transaction.applicable_header_trade_settlement.applicable_trade_tax.check(rule)?;
    let bt_117 = bg_23.calculated_amount.check(rule)?;
    let bt_116 = bg_23.basis_amount.check(rule)?;
    let bt_119 = bg_23.rate_applicable_percent.check(rule)?;
    check_float_eq!(rule; bt_117, bt_116 * (bt_119 / 100.0))
}


/// Shortcut to handle possibly missing values
trait OptionExt<T> {
    /// Check if the Option is Some, otherwise return a BusinessRuleViolation
    fn check(self, rule: &str) -> Result<T, BusinessRuleViolation>;
}
impl <T> OptionExt<T> for Option<T> {
    fn check(self, rule: &str) -> Result<T, BusinessRuleViolation> {
        self.ok_or_else(|| BusinessRuleViolation {
            rule: rule.to_string(),
            message: "Value is None".to_string(),
        })
    }
}