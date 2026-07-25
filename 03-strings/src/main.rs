//! Rust strings are UTF-8 and come in owned (`String`) and borrowed (`str`) forms.

use std::fmt::Write;

fn heading(number: u8, title: &str) {
    println!("\n=== {number:02}: {title} ===");
}

fn main() {
    example_01_string_and_str();
    example_02_conversions_and_ownership();
    example_03_mutation();
    example_04_concatenation_and_formatting();
    example_05_utf8_bytes_and_characters();
    example_06_safe_slicing();
    example_07_search_and_split();
    example_08_replace_and_parse();
    example_09_building_text_efficiently();
}

fn describe(text: &str) {
    println!("{text:?} occupies {} UTF-8 bytes", text.len());
}

fn example_01_string_and_str() {
    heading(1, "String and &str");

    let literal: &str = "borrowed literal";
    let owned: String = String::from("owned and growable");
    describe(literal);
    describe(&owned); // `&String` coerces to `&str`.
}

fn example_02_conversions_and_ownership() {
    heading(2, "conversions and ownership");

    let from_literal = "hello".to_owned();
    let another = String::from("world");
    let borrowed: &str = another.as_str();
    println!("{from_literal} {borrowed}");
}

fn example_03_mutation() {
    heading(3, "mutation");

    let mut message = String::with_capacity(32);
    message.push_str("Rust");
    message.push(' ');
    message.insert_str(0, "Learning ");
    message.retain(|character| character != ' ');
    println!("{message}; capacity={}", message.capacity());
}

fn example_04_concatenation_and_formatting() {
    heading(4, "concatenation and formatting");

    let left = String::from("hello");
    let right = " Rust";
    let combined = left + right; // Moves `left` and borrows `right`.
    let formatted = format!("{combined}, version {}!", 2024);
    println!("{formatted}");
}

fn example_05_utf8_bytes_and_characters() {
    heading(5, "UTF-8 bytes and characters");

    let text = "aé🦀";
    println!("bytes={}, chars={}", text.len(), text.chars().count());
    println!("byte values={:?}", text.bytes().collect::<Vec<_>>());
    println!("characters={:?}", text.chars().collect::<Vec<_>>());
}

fn example_06_safe_slicing() {
    heading(6, "safe slicing");

    let text = "Здравствуйте";
    // `get` returns None instead of panicking when bounds split a UTF-8 character.
    println!(
        "0..4={:?}; invalid 0..1={:?}",
        text.get(0..4),
        text.get(0..1)
    );
}

fn example_07_search_and_split() {
    heading(7, "search and split");

    let input = "red, green, blue";
    println!("contains green={}", input.contains("green"));
    for (index, part) in input.split(',').map(str::trim).enumerate() {
        println!("{index}: {part}");
    }
}

fn example_08_replace_and_parse() {
    heading(8, "replace and parse");

    let normalized = "2026-07-25".replace('-', "/");
    let parsed: Result<i32, _> = "42".parse();
    let invalid: Result<bool, _> = "perhaps".parse();
    println!("normalized={normalized}; parsed={parsed:?}; invalid={invalid:?}");
}

fn example_09_building_text_efficiently() {
    heading(9, "building text efficiently");

    let names = ["Ada", "Grace", "Linus"];
    println!("join: {}", names.join(", "));

    let mut report = String::new();
    for (index, name) in names.iter().enumerate() {
        // `write!` appends formatted text without allocating an intermediate String.
        writeln!(&mut report, "{}. {name}", index + 1).expect("writing to String cannot fail");
    }
    print!("{report}");
}
