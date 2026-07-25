//! Ownership is Rust's central model for memory safety.

fn heading(number: u8, title: &str) {
    println!("\n=== {number:02}: {title} ===");
}

fn main() {
    example_01_stack_and_heap();
    example_02_moves_transfer_ownership();
    example_03_copy_and_clone();
    example_04_shared_borrows();
    example_05_mutable_borrows();
    example_06_slices_borrow_part_of_a_value();
    example_07_explicit_and_elided_lifetimes();
    example_08_struct_and_multiple_lifetimes();
    example_09_static_lifetime();
}

fn example_01_stack_and_heap() {
    heading(1, "stack and heap values");

    let stack_number = 42;
    let heap_text = String::from("owned text");
    println!("stack={stack_number}, heap={heap_text}");
} // Both values are dropped here; String also releases its heap allocation.

fn consume(text: String) -> usize {
    text.len()
}

fn example_02_moves_transfer_ownership() {
    heading(2, "moves transfer ownership");

    let first_owner = String::from("move me");
    let second_owner = first_owner;
    // `first_owner` can no longer be used: only `second_owner` owns the String.
    println!("new owner: {second_owner}");

    let length = consume(second_owner);
    println!("the function owned and measured {length} bytes");
}

fn example_03_copy_and_clone() {
    heading(3, "Copy and Clone");

    let original_number = 7;
    let copied_number = original_number; // Integers implement `Copy`.
    println!("both integers work: {original_number}, {copied_number}");

    let original_text = String::from("deep data");
    let cloned_text = original_text.clone(); // Explicitly duplicate heap data.
    println!("both strings work: {original_text}, {cloned_text}");
}

fn text_length(text: &str) -> usize {
    text.len()
}

fn example_04_shared_borrows() {
    heading(4, "shared borrowing");

    let text = String::from("borrowed, not moved");
    let first = &text;
    let second = &text; // Many immutable references may coexist.
    println!(
        "{} bytes; first={first}; second={second}",
        text_length(&text)
    );
}

fn append_period(text: &mut String) {
    text.push('.');
}

fn example_05_mutable_borrows() {
    heading(5, "mutable borrowing");

    let mut text = String::from("change me");
    {
        let editable = &mut text; // The exclusive borrow ends at its last use.
        editable.make_ascii_uppercase();
    }
    append_period(&mut text);
    println!("{text}");
}

fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

fn example_06_slices_borrow_part_of_a_value() {
    heading(6, "slices borrow part of a value");

    let sentence = String::from("safe slices track their source");
    let word = first_word(&sentence);
    println!("first word={word}; full sentence={sentence}");
}

fn longer<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() {
        left
    } else {
        right
    }
}

// Lifetime elision lets the compiler infer that the output borrows from `input`.
fn identity(input: &str) -> &str {
    input
}

fn example_07_explicit_and_elided_lifetimes() {
    heading(7, "explicit and elided lifetimes");

    let left = String::from("short");
    let right = String::from("a little longer");
    println!(
        "longer={}, identity={}",
        longer(&left, &right),
        identity(&left)
    );
}

#[derive(Debug)]
struct Highlight<'a> {
    source: &'a str,
    excerpt: &'a str,
}

fn choose_with_label<'a, 'b>(value: &'a str, label: &'b str) -> (&'a str, &'b str) {
    (value, label)
}

fn example_08_struct_and_multiple_lifetimes() {
    heading(8, "struct and multiple lifetimes");

    let article = String::from("Rust makes borrowing relationships explicit.");
    let highlight = Highlight {
        source: &article,
        excerpt: &article[..4],
    };
    let label = String::from("topic");
    let (value, borrowed_label) = choose_with_label(highlight.excerpt, &label);
    println!(
        "{highlight:?}; source-bytes={}; chosen={value}; label={borrowed_label}",
        highlight.source.len()
    );
}

static MOTTO: &str = "learn by running";

fn example_09_static_lifetime() {
    heading(9, "'static data");

    // String literals are embedded in the binary and live for the full program.
    let also_static: &'static str = "I live for the whole program";
    println!("{MOTTO}: {also_static}");
}
