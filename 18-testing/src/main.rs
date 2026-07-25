//! The executable points to real tests in `lib.rs` and `tests/`.

use rust_testing::{Calculator, parse_nonnegative, slugify};

fn heading(number: u8, title: &str) {
    println!("\n=== {number:02}: {title} ===");
}

fn main() {
    example_01_testable_public_api();
    example_02_basic_assertions();
    example_03_helpful_messages();
    example_04_result_tests();
    example_05_should_panic();
    example_06_unit_and_private_tests();
    example_07_integration_tests();
    example_08_filters_ignored_and_docs();
}

fn example_01_testable_public_api() {
    heading(1, "testable public API");
    let calculator = Calculator::new(10);
    println!("10 + 5 = {}", calculator.add_to_value(5));
}

fn example_02_basic_assertions() {
    heading(2, "basic assertions");
    println!("See addition_returns_expected_value in src/lib.rs");
    println!("Run: cargo test addition");
}

fn example_03_helpful_messages() {
    heading(3, "helpful assertion messages");
    println!("slug={}", slugify("Hello Rust"));
    println!("assert_eq! accepts a custom message explaining the intended behavior");
}

fn example_04_result_tests() {
    heading(4, "Result-returning tests");
    println!(
        "valid={:?}; invalid={:?}",
        parse_nonnegative("7"),
        parse_nonnegative("-1")
    );
}

fn example_05_should_panic() {
    heading(5, "should_panic");
    println!("#[should_panic(expected = \"...\")] verifies an intentional panic");
}

fn example_06_unit_and_private_tests() {
    heading(6, "unit and private-function tests");
    println!("The #[cfg(test)] child module can test private parent-module helpers");
}

fn example_07_integration_tests() {
    heading(7, "integration tests and shared setup");
    println!("tests/api.rs consumes only the library's public interface");
    println!("tests/common/mod.rs provides shared integration-test setup");
}

fn example_08_filters_ignored_and_docs() {
    heading(8, "filters, ignored tests, and documentation tests");
    println!("cargo test addition");
    println!("cargo test -- --ignored");
    println!("cargo test --doc");
}
