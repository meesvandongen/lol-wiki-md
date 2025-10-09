use lol_wiki_md::parse::{evaluate_expression, ExprNumberFormat};
use proptest::prelude::*;

#[test]
fn basic() { assert_eq!(evaluate_expression("1+2*3", ExprNumberFormat::Float(2)).unwrap(), "7"); }

proptest! {
    #[test]
    fn no_panic_random_numbers(a in -1000f64..1000.0, b in -1000f64..1000.0) {
        let expr = format!("({a})+({b})");
        let _ = evaluate_expression(&expr, ExprNumberFormat::Float(2));
    }
}
