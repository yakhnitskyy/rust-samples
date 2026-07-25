//! A small public API used by the binary, unit tests, integration tests, and docs.
//!
//! ```
//! use rust_testing::Calculator;
//!
//! let calculator = Calculator::new(10);
//! assert_eq!(calculator.add_to_value(5), 15);
//! ```

use std::num::ParseIntError;

#[derive(Debug, Clone, Copy)]
pub struct Calculator {
    value: i32,
}

impl Calculator {
    pub fn new(value: i32) -> Self {
        Self { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }

    pub fn add_to_value(self, amount: i32) -> i32 {
        self.value + amount
    }

    pub fn checked_multiply(self, factor: i32) -> Option<i32> {
        self.value.checked_mul(factor)
    }
}

/// Parses a non-negative integer.
///
/// ```
/// assert_eq!(rust_testing::parse_nonnegative("42").unwrap(), 42);
/// assert!(rust_testing::parse_nonnegative("-1").is_err());
/// ```
pub fn parse_nonnegative(input: &str) -> Result<u32, ParseIntError> {
    input.parse()
}

pub fn slugify(input: &str) -> String {
    input
        .split_whitespace()
        .map(str::to_lowercase)
        .collect::<Vec<_>>()
        .join("-")
}

#[cfg(test)]
fn normalize_spaces(input: &str) -> String {
    input.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calculator() -> Calculator {
        Calculator::new(10)
    }

    #[test]
    fn addition_returns_expected_value() {
        assert_eq!(calculator().add_to_value(5), 15);
    }

    #[test]
    fn assertion_can_include_context() {
        let actual = slugify("Hello Rust");
        assert_eq!(
            actual, "hello-rust",
            "slug should normalize case and spaces"
        );
    }

    #[test]
    fn result_returning_test() -> Result<(), ParseIntError> {
        let value = parse_nonnegative("7")?;
        assert_eq!(value, 7);
        Ok(())
    }

    #[test]
    #[should_panic(expected = "demonstration panic")]
    fn expected_panic_is_testable() {
        panic!("demonstration panic");
    }

    #[test]
    fn private_helpers_are_visible_to_child_test_module() {
        assert_eq!(normalize_spaces("  too   many spaces "), "too many spaces");
    }

    #[test]
    #[ignore = "demonstrates an opt-in slow test"]
    fn deliberately_ignored_test() {
        std::thread::sleep(std::time::Duration::from_millis(10));
        assert_eq!(2 + 2, 4);
    }
}
