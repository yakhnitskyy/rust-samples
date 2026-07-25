//! Closures carry behavior; iterators build lazy, composable data pipelines.

use std::collections::HashMap;

fn heading(number: u8, title: &str) {
    println!("\n=== {number:02}: {title} ===");
}

fn main() {
    example_01_immutable_capture();
    example_02_mutable_and_move_capture();
    example_03_fn_trait_family();
    example_04_iterator_adapters();
    example_05_iterator_consumers();
    example_06_custom_iterator();
    example_07_lazy_evaluation();
    example_08_chaining_and_grouping();
}

fn example_01_immutable_capture() {
    heading(1, "immutable closure capture");

    let threshold = 10;
    let is_large = |value: i32| value > threshold;
    println!("8 large? {}; 12 large? {}", is_large(8), is_large(12));
}

fn example_02_mutable_and_move_capture() {
    heading(2, "mutable and move capture");

    let mut total = 0;
    let mut add = |value| total += value;
    add(2);
    add(3);
    println!("mutably captured total={total}");

    let owned = String::from("moved into closure");
    let consume_later = move || println!("{owned}");
    consume_later();
}

fn call_twice<F: Fn(i32) -> i32>(function: F, value: i32) -> (i32, i32) {
    (function(value), function(value))
}

fn mutate_once<F: FnMut()>(mut function: F) {
    function();
}

fn consume_once<F: FnOnce() -> String>(function: F) -> String {
    function()
}

fn example_03_fn_trait_family() {
    heading(3, "Fn, FnMut, and FnOnce");

    println!("Fn={:?}", call_twice(|value| value * 2, 4));
    let mut count = 0;
    mutate_once(|| count += 1);
    println!("FnMut count={count}");

    let text = String::from("consumed");
    println!("FnOnce={}", consume_once(|| text));
}

fn example_04_iterator_adapters() {
    heading(4, "iterator adapters");

    let transformed: Vec<_> = (1..=10)
        .filter(|value| value % 2 == 0)
        .map(|value| value * value)
        .take(3)
        .collect();
    println!("{transformed:?}");
}

fn example_05_iterator_consumers() {
    heading(5, "iterator consumers");

    let values = [3, 8, 2, 9];
    println!(
        "sum={}; max={:?}",
        values.iter().sum::<i32>(),
        values.iter().max()
    );
    println!(
        "any > 8={}; position of 2={:?}",
        values.iter().any(|value| *value > 8),
        values.iter().position(|value| *value == 2)
    );
}

struct Counter {
    next: u32,
    end: u32,
}

impl Counter {
    fn new(end: u32) -> Self {
        Self { next: 1, end }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next > self.end {
            None
        } else {
            let current = self.next;
            self.next += 1;
            Some(current)
        }
    }
}

fn example_06_custom_iterator() {
    heading(6, "custom iterator");

    println!("{:?}", Counter::new(5).collect::<Vec<_>>());
}

fn example_07_lazy_evaluation() {
    heading(7, "lazy evaluation");

    let pipeline = (1..=4).inspect(|value| println!("processing {value}"));
    println!("nothing was processed until collection");
    let doubled: Vec<_> = pipeline.map(|value| value * 2).collect();
    println!("{doubled:?}");
}

fn example_08_chaining_and_grouping() {
    heading(8, "chaining and grouping");

    let flattened: Vec<_> = [vec![1, 2], vec![3, 4]]
        .into_iter()
        .flatten()
        .chain([5, 6])
        .collect();
    println!("flattened={flattened:?}");

    let mut by_length: HashMap<usize, Vec<&str>> = HashMap::new();
    for word in ["a", "to", "rust", "be", "safe"] {
        by_length.entry(word.len()).or_default().push(word);
    }
    println!("grouped={by_length:?}");
}
