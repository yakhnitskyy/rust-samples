//! Standard collections trade ordering, lookup speed, and access patterns.

use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

fn heading(number: u8, title: &str) {
    println!("\n=== {number:02}: {title} ===");
}

fn main() {
    example_01_hash_map();
    example_02_entry_api_word_count();
    example_03_grouping_values();
    example_04_hash_set();
    example_05_set_operations();
    example_06_vec_deque();
    example_07_binary_heap();
    example_08_choosing_a_collection();
}

fn example_01_hash_map() {
    heading(1, "HashMap");

    let mut scores = HashMap::new();
    scores.insert("blue", 10);
    scores.insert("red", 7);
    println!(
        "blue={:?}; missing={:?}",
        scores.get("blue"),
        scores.get("green")
    );
    for (team, score) in &scores {
        println!("{team}: {score}");
    }
}

fn example_02_entry_api_word_count() {
    heading(2, "entry API and word counts");

    let mut counts = HashMap::new();
    for word in "red blue red green blue red".split_whitespace() {
        *counts.entry(word).or_insert(0) += 1;
    }
    println!("{counts:?}");
}

fn example_03_grouping_values() {
    heading(3, "grouping values");

    let records = [("fruit", "apple"), ("tool", "hammer"), ("fruit", "pear")];
    let mut grouped: HashMap<&str, Vec<&str>> = HashMap::new();
    for (category, item) in records {
        grouped.entry(category).or_default().push(item);
    }
    println!("{grouped:?}");
}

fn example_04_hash_set() {
    heading(4, "HashSet");

    let tags: HashSet<_> = ["rust", "systems", "rust"].into_iter().collect();
    println!("unique={tags:?}; contains rust={}", tags.contains("rust"));
}

fn example_05_set_operations() {
    heading(5, "set operations");

    let left: HashSet<_> = [1, 2, 3].into_iter().collect();
    let right: HashSet<_> = [3, 4, 5].into_iter().collect();
    println!(
        "union={:?}",
        left.union(&right).copied().collect::<Vec<_>>()
    );
    println!(
        "intersection={:?}",
        left.intersection(&right).copied().collect::<Vec<_>>()
    );
}

fn example_06_vec_deque() {
    heading(6, "VecDeque");

    let mut queue = VecDeque::from(["first", "second"]);
    queue.push_back("third");
    queue.push_front("urgent");
    while let Some(item) = queue.pop_front() {
        print!("{item} ");
    }
    println!();
}

fn example_07_binary_heap() {
    heading(7, "BinaryHeap");

    let mut priorities = BinaryHeap::from([2, 9, 4, 1]);
    while let Some(next) = priorities.pop() {
        print!("{next} "); // `BinaryHeap` is a max-heap.
    }
    println!();
}

fn example_08_choosing_a_collection() {
    heading(8, "choosing a collection");

    println!("Vec: ordered indexed sequence");
    println!("VecDeque: efficient work at both ends");
    println!("HashMap: key-to-value lookup");
    println!("HashSet: uniqueness and membership");
    println!("BinaryHeap: repeatedly retrieve the highest priority");
}
