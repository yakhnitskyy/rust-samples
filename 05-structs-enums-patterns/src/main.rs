//! Structs combine fields; enums represent one of several possible variants.

fn heading(number: u8, title: &str) {
    println!("\n=== {number:02}: {title} ===");
}

fn main() {
    example_01_named_structs();
    example_02_tuple_and_unit_structs();
    example_03_methods_and_associated_functions();
    example_04_data_carrying_enums();
    example_05_option_and_exhaustive_match();
    example_06_if_let_and_let_else();
    example_07_guards_and_destructuring();
    example_08_state_modeling();
}

#[derive(Debug)]
struct User {
    name: String,
    active: bool,
}

fn example_01_named_structs() {
    heading(1, "named structs");

    let name = String::from("Ferris");
    let user = User { name, active: true };
    println!("{} active={}", user.name, user.active);
}

#[derive(Debug)]
struct Color(u8, u8, u8);

#[derive(Debug)]
struct Marker;

fn example_02_tuple_and_unit_structs() {
    heading(2, "tuple and unit structs");

    let orange = Color(255, 128, 0);
    let Color(red, green, blue) = orange;
    println!("rgb=({red}, {green}, {blue}); marker={:?}", Marker);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
}

fn example_03_methods_and_associated_functions() {
    heading(3, "methods and associated functions");

    let mut rectangle = Rectangle::new(3, 4);
    rectangle.scale(2);
    println!("{rectangle:?}; area={}", rectangle.area());
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

fn describe_message(message: Message) -> String {
    match message {
        Message::Quit => "quit".to_owned(),
        Message::Move { x, y } => format!("move to ({x}, {y})"),
        Message::Write(text) => format!("write {text:?}"),
        Message::ChangeColor(r, g, b) => format!("rgb({r}, {g}, {b})"),
    }
}

fn example_04_data_carrying_enums() {
    heading(4, "data-carrying enums");

    let messages = [
        Message::Quit,
        Message::Move { x: 3, y: -1 },
        Message::Write("hello".into()),
        Message::ChangeColor(10, 20, 30),
    ];
    for message in messages {
        println!("{}", describe_message(message));
    }
}

fn example_05_option_and_exhaustive_match() {
    heading(5, "Option and exhaustive match");

    let values = [Some(4), None];
    for value in values {
        match value {
            Some(number) if number % 2 == 0 => println!("even value: {number}"),
            Some(number) => println!("odd value: {number}"),
            None => println!("no value"),
        }
    }
}

fn print_if_present(value: Option<&str>) {
    if let Some(text) = value {
        println!("if let found {text}");
    }
}

fn first_character(text: &str) {
    let Some(character) = text.chars().next() else {
        println!("the text is empty");
        return;
    };
    println!("let-else found {character}");
}

fn example_06_if_let_and_let_else() {
    heading(6, "if let and let else");

    print_if_present(Some("Rust"));
    first_character("");
    first_character("borrow");
}

fn example_07_guards_and_destructuring() {
    heading(7, "guards and destructuring");

    let point = (0, 7);
    match point {
        (0, y) if y > 0 => println!("positive y-axis at {y}"),
        (x, 0) => println!("x-axis at {x}"),
        (x, y) => println!("general point ({x}, {y})"),
    }
}

#[derive(Debug)]
enum OrderState {
    Created,
    Paid { receipt: u32 },
    Shipped { tracking: String },
}

fn status(state: &OrderState) -> String {
    match state {
        OrderState::Created => "waiting for payment".into(),
        OrderState::Paid { receipt } => format!("paid with receipt {receipt}"),
        OrderState::Shipped { tracking } => format!("shipped as {tracking}"),
    }
}

fn example_08_state_modeling() {
    heading(8, "state modeling");

    let states = [
        OrderState::Created,
        OrderState::Paid { receipt: 42 },
        OrderState::Shipped {
            tracking: "RS123".into(),
        },
    ];
    for state in states {
        println!("{state:?}: {}", status(&state));
    }
}
