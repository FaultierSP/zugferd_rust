//! Validation of business rules

use crate::Invoice;

pub struct BusinessRuleViolation {
    pub rule: String,
    pub message: String,
}

/// List of business rules to validate
const BUSINESS_RULES: &[fn(&Invoice) -> Result<(), BusinessRuleViolation>] = &[
    br_co_14,
    br_co_17,
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