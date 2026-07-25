//! Declarative macros transform matched syntax into new Rust syntax.

macro_rules! squared {
    ($value:expr) => {{
        let value = $value;
        value * value
    }};
}

macro_rules! describe {
    (empty) => {
        "nothing"
    };
    (number $value:expr) => {
        if $value == 1 { "one" } else { "many" }
    };
    (text $value:expr) => {
        $value
    };
}

macro_rules! string_list {
    ($($value:expr),* $(,)?) => {
        vec![$($value.to_string()),*]
    };
}

macro_rules! make_constant_function {
    ($name:ident => $value:expr) => {
        fn $name() -> i32 {
            $value
        }
    };
}

make_constant_function!(answer => 42);

#[macro_export]
macro_rules! surround {
    ($value:expr) => {
        format!("[{}]", $value)
    };
}

macro_rules! hygienic_double {
    ($value:expr) => {{
        // This local cannot accidentally capture a `temporary` at the call site.
        let temporary = $value;
        temporary + temporary
    }};
}

macro_rules! map_of {
    ($($key:expr => $value:expr),* $(,)?) => {{
        let mut map = std::collections::HashMap::new();
        $(map.insert($key, $value);)*
        map
    }};
}

macro_rules! assert_approx_eq {
    ($left:expr, $right:expr, $tolerance:expr) => {{
        let left = $left;
        let right = $right;
        let tolerance = $tolerance;
        assert!(
            (left - right).abs() <= tolerance,
            "{left} differs from {right} by more than {tolerance}"
        );
    }};
}

fn heading(number: u8, title: &str) {
    println!("\n=== {number:02}: {title} ===");
}

fn main() {
    example_01_standard_macros();
    example_02_expression_macro();
    example_03_multiple_match_arms();
    example_04_repetition();
    example_05_item_generation();
    example_06_exported_macro();
    example_07_hygiene();
    example_08_practical_macros();
}

fn example_01_standard_macros() {
    heading(1, "standard-library macros");

    let values = vec![1, 2, 3];
    println!("values={values:?}; source expression={}", stringify!(1 + 2));
    assert_eq!(values.len(), 3);
}

fn example_02_expression_macro() {
    heading(2, "expression macro");

    let mut calls = 0;
    let value = squared!({
        calls += 1;
        5
    });
    println!("square={value}; argument evaluated {calls} time");
}

fn example_03_multiple_match_arms() {
    heading(3, "multiple match arms");

    println!(
        "{}, {}, {}",
        describe!(empty),
        describe!(number 2),
        describe!(text "matched text")
    );
}

fn example_04_repetition() {
    heading(4, "repetition");

    let values = string_list!("Rust", 2024, true);
    println!("{values:?}");
}

fn example_05_item_generation() {
    heading(5, "item generation");
    println!("generated answer()={}", answer());
}

fn example_06_exported_macro() {
    heading(6, "exported macro");
    println!("{}", surround!("publicly exported"));
}

fn example_07_hygiene() {
    heading(7, "macro hygiene");

    let temporary = 100;
    let doubled = hygienic_double!(6);
    println!("call-site temporary={temporary}; macro result={doubled}");
}

fn example_08_practical_macros() {
    heading(8, "practical map and assertion macros");

    let scores = map_of!("red" => 3, "blue" => 7);
    println!("{scores:?}");
    assert_approx_eq!(0.1_f64 + 0.2, 0.3, 1e-10);
    println!("approximate floating-point assertion passed");
}
