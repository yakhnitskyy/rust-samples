//! Generics abstract over types; traits describe behavior those types provide.

use std::fmt::{self, Display};
use std::ops::Add;

fn heading(number: u8, title: &str) {
    println!("\n=== {number:02}: {title} ===");
}

fn main() {
    example_01_generic_functions();
    example_02_generic_structs_and_methods();
    example_03_traits_and_defaults();
    example_04_trait_bounds_and_where();
    example_05_impl_trait();
    example_06_trait_objects();
    example_07_associated_types();
    example_08_display_and_add();
}

fn largest<T: Ord>(values: &[T]) -> Option<&T> {
    values.iter().max()
}

fn example_01_generic_functions() {
    heading(1, "generic functions");

    println!("largest number={:?}", largest(&[4, 9, 2]));
    println!("largest word={:?}", largest(&["ant", "zebra", "cat"]));
}

#[derive(Debug)]
struct Pair<T> {
    left: T,
    right: T,
}

impl<T> Pair<T> {
    fn new(left: T, right: T) -> Self {
        Self { left, right }
    }
}

impl<T: Add<Output = T> + Copy> Pair<T> {
    fn total(&self) -> T {
        self.left + self.right
    }
}

fn example_02_generic_structs_and_methods() {
    heading(2, "generic structs and methods");

    let pair = Pair::new(20, 22);
    println!("{pair:?}; total={}", pair.total());
}

trait Summary {
    fn title(&self) -> &str;

    fn summarize(&self) -> String {
        format!("Read: {}", self.title())
    }
}

struct Note {
    title: String,
    body: String,
}

impl Summary for Note {
    fn title(&self) -> &str {
        &self.title
    }

    fn summarize(&self) -> String {
        format!("{} — {} chars", self.title, self.body.len())
    }
}

fn example_03_traits_and_defaults() {
    heading(3, "traits and default methods");

    let note = Note {
        title: "Traits".into(),
        body: "Shared behavior".into(),
    };
    println!("{}", note.summarize());
}

fn compare_and_print<T, U>(left: &T, right: &T, label: U)
where
    T: PartialOrd + Display,
    U: Display,
{
    let winner = if left >= right { left } else { right };
    println!("{label}: {winner}");
}

fn example_04_trait_bounds_and_where() {
    heading(4, "trait bounds and where clauses");

    compare_and_print(&7, &11, "larger value");
}

fn make_summary(title: &str) -> impl Summary {
    Note {
        title: title.into(),
        body: "returned as an opaque implementing type".into(),
    }
}

fn example_05_impl_trait() {
    heading(5, "impl Trait");

    println!("{}", make_summary("Opaque return types").summarize());
}

trait Draw {
    fn draw(&self) -> String;
}

struct Circle(u32);
struct Label(&'static str);

impl Draw for Circle {
    fn draw(&self) -> String {
        format!("circle radius {}", self.0)
    }
}

impl Draw for Label {
    fn draw(&self) -> String {
        format!("label {:?}", self.0)
    }
}

fn example_06_trait_objects() {
    heading(6, "trait objects");

    let widgets: Vec<Box<dyn Draw>> = vec![Box::new(Circle(3)), Box::new(Label("OK"))];
    for widget in widgets {
        println!("{}", widget.draw()); // Dynamic dispatch at runtime.
    }
}

trait Sequence {
    type Item;
    fn next_item(&mut self) -> Option<Self::Item>;
}

struct Countdown(u8);

impl Sequence for Countdown {
    type Item = u8;

    fn next_item(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let current = self.0;
            self.0 -= 1;
            Some(current)
        }
    }
}

fn example_07_associated_types() {
    heading(7, "associated types");

    let mut countdown = Countdown(3);
    while let Some(value) = countdown.next_item() {
        print!("{value} ");
    }
    println!();
}

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Display for Point {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "({}, {})", self.x, self.y)
    }
}

fn example_08_display_and_add() {
    heading(8, "Display and Add implementations");

    let sum = Point { x: 1, y: 2 } + Point { x: 3, y: 4 };
    println!("point sum={sum}");
}
