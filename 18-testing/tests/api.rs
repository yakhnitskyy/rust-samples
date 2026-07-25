mod common;

use rust_testing::{parse_nonnegative, slugify};

#[test]
fn public_calculator_api_works_from_an_external_crate() {
    let calculator = common::configured_calculator();
    assert_eq!(calculator.value(), 10);
    assert_eq!(calculator.add_to_value(5), 15);
    assert_eq!(calculator.checked_multiply(4), Some(40));
}

#[test]
fn public_parsing_api_reports_invalid_input() {
    assert_eq!(parse_nonnegative("12").unwrap(), 12);
    assert!(parse_nonnegative("-12").is_err());
}

#[test]
fn public_slug_api_handles_extra_whitespace() {
    assert_eq!(slugify("  Learn   Rust  "), "learn-rust");
}
