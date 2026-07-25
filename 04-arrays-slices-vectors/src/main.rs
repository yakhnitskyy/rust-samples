//! Arrays own a fixed number of values; slices borrow a sequence; vectors grow.

fn heading(number: u8, title: &str) {
    println!("\n=== {number:02}: {title} ===");
}

fn main() {
    example_01_fixed_arrays();
    example_02_safe_indexing();
    example_03_shared_slices();
    example_04_mutable_slices();
    example_05_vector_capacity_and_growth();
    example_06_insertion_and_removal();
    example_07_sorting_and_deduplication();
    example_08_nested_vectors_and_slice_apis();
}

fn example_01_fixed_arrays() {
    heading(1, "fixed arrays and repetition");

    let primes: [u8; 4] = [2, 3, 5, 7];
    let zeroes = [0; 5];
    println!(
        "primes={primes:?}; zeroes={zeroes:?}; length={}",
        primes.len()
    );
}

fn example_02_safe_indexing() {
    heading(2, "safe indexing");

    let values = [10, 20, 30];
    println!("index 1={}; get 99={:?}", values[1], values.get(99));
}

fn sum(values: &[i32]) -> i32 {
    values.iter().sum()
}

fn example_03_shared_slices() {
    heading(3, "shared slices");

    let values = [1, 2, 3, 4, 5];
    let middle = &values[1..4];
    println!("middle={middle:?}; sum={}", sum(middle));
}

fn double(values: &mut [i32]) {
    for value in values {
        *value *= 2;
    }
}

fn example_04_mutable_slices() {
    heading(4, "mutable slices");

    let mut values = [1, 2, 3, 4];
    double(&mut values[1..3]);
    println!("{values:?}");
}

fn example_05_vector_capacity_and_growth() {
    heading(5, "vector capacity and growth");

    let mut values = Vec::with_capacity(4);
    for value in 1..=5 {
        values.push(value * 10);
    }
    println!(
        "{values:?}; len={}; capacity={}",
        values.len(),
        values.capacity()
    );
}

fn example_06_insertion_and_removal() {
    heading(6, "insertion and removal");

    let mut tasks = vec!["read", "practice"];
    tasks.insert(1, "compile");
    let removed = tasks.remove(0);
    let last = tasks.pop();
    println!("removed={removed}; last={last:?}; remaining={tasks:?}");
}

fn example_07_sorting_and_deduplication() {
    heading(7, "sorting and deduplication");

    let mut numbers = vec![5, 1, 3, 1, 5, 2];
    numbers.sort_unstable();
    numbers.dedup(); // Adjacent duplicates are removed, so sort first.
    println!("{numbers:?}");
}

fn row_total(row: &[i32]) -> i32 {
    row.iter().sum()
}

fn example_08_nested_vectors_and_slice_apis() {
    heading(8, "nested vectors and slice APIs");

    let matrix = [vec![1, 2, 3], vec![4, 5, 6]];
    let totals: Vec<i32> = matrix.iter().map(|row| row_total(row)).collect();
    println!("matrix={matrix:?}; row totals={totals:?}");
}
