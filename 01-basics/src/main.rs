//! Small, executable examples of Rust's basic syntax.

fn heading(number: u8, title: &str) {
    println!("\n=== {number:02}: {title} ===");
}

fn main() {
    // example_01_variables_mutability_and_constants();
    // example_02_shadowing_and_scalars();
    // example_03_tuples_and_destructuring();
    example_04_functions_and_expressions();
    // example_05_conditionals_are_expressions();
    // example_06_loops_can_return_values();
    // example_07_ranges_and_patterns();
}

fn example_01_variables_mutability_and_constants() {
    heading(1, "variables, mutability, and constants");

    let language = "Rust"; // Immutable unless `mut` is written explicitly.
    let mut lessons_completed = 0;
    lessons_completed += 1;
    const MINUTES_PER_HOUR: u32 = 60;

    println!("{language}: lesson {lessons_completed}, {MINUTES_PER_HOUR} min/hour");
}

fn example_02_shadowing_and_scalars() {
    heading(2, "shadowing and scalar types");

    // Shadowing creates a new binding and may change the value's type.
    let spaces = "   ";
    let spaces = spaces.len();

    let signed: i32 = -42;
    let decimal: f64 = 3.5;
    let ready: bool = true;
    let crab: char = '🦀'; // `char` is a Unicode scalar value.
    println!("spaces={spaces}, signed={signed}, decimal={decimal}, ready={ready}, {crab}");
}

fn example_03_tuples_and_destructuring() {
    heading(3, "tuples and destructuring");

    let record = ("Ada", 36, true);
    let (name, age, active) = record;
    println!(
        "{name} is {age}; active={active}; tuple index 0={}",
        record.0
    );
}

fn square(value: i32) -> i32 {
    value * value // No semicolon: this expression is the return value.
}

fn example_04_functions_and_expressions() {
    heading(4, "functions and expressions");

    let offset = {
        let base = 10;
        base + 5
    };
    println!("square(6)={}, block value={offset}", square(6));
}

fn example_05_conditionals_are_expressions() {
    heading(5, "conditional expressions");

    let score = 82;
    let grade = if score >= 90 {
        "excellent"
    } else if score >= 70 {
        "good"
    } else {
        "keep practicing"
    };
    println!("score {score}: {grade}");
}

fn example_06_loops_can_return_values() {
    heading(6, "loops can return values");

    let mut counter = 0;
    let doubled = loop {
        counter += 1;
        if counter == 5 {
            break counter * 2;
        }
    };

    let mut countdown = 3;
    while countdown > 0 {
        print!("{countdown} ");
        countdown -= 1;
    }
    println!("go! loop result={doubled}");
}

fn example_07_ranges_and_patterns() {
    heading(7, "ranges and pattern destructuring");

    let sum: i32 = (1..=5).sum();
    for (index, letter) in ['a', 'b', 'c'].into_iter().enumerate() {
        println!("{index}: {letter}");
    }

    let point = (3, 7);
    let (x, y) = point;
    println!("sum 1..=5={sum}; point=({x}, {y})");
}
