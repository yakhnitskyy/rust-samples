//! Recoverable failures are values: usually `Option<T>` or `Result<T, E>`.

use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

fn heading(number: u8, title: &str) {
    println!("\n=== {number:02}: {title} ===");
}

fn main() {
    example_01_option();
    example_02_matching_result();
    example_03_question_mark_operator();
    example_04_custom_errors();
    example_05_error_sources();
    example_06_from_conversions();
    example_07_recovery_and_propagation();
    example_08_boxed_application_errors();
}

fn first_even(values: &[i32]) -> Option<i32> {
    values.iter().copied().find(|value| value % 2 == 0)
}

fn example_01_option() {
    heading(1, "Option");

    let found = first_even(&[1, 3, 4, 7]);
    let missing = first_even(&[1, 3, 5]);
    println!("found={found:?}; default={}", missing.unwrap_or(0));
}

fn example_02_matching_result() {
    heading(2, "matching Result");

    for text in ["42", "forty-two"] {
        match text.parse::<i32>() {
            Ok(number) => println!("{text:?} -> {number}"),
            Err(error) => println!("{text:?} failed: {error}"),
        }
    }
}

fn parse_and_double(text: &str) -> Result<i32, ParseIntError> {
    let number = text.parse::<i32>()?; // Early-return the error when parsing fails.
    Ok(number * 2)
}

fn example_03_question_mark_operator() {
    heading(3, "the ? operator");

    println!("21 -> {:?}", parse_and_double("21"));
    println!("nope -> {:?}", parse_and_double("nope"));
}

#[derive(Debug)]
enum ScoreError {
    NotANumber(ParseIntError),
    Negative(i32),
}

impl fmt::Display for ScoreError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotANumber(_) => write!(formatter, "score is not an integer"),
            Self::Negative(value) => write!(formatter, "score cannot be negative: {value}"),
        }
    }
}

impl Error for ScoreError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NotANumber(source) => Some(source),
            Self::Negative(_) => None,
        }
    }
}

fn parse_score(text: &str) -> Result<u32, ScoreError> {
    let value = text.parse::<i32>().map_err(ScoreError::NotANumber)?;
    if value < 0 {
        return Err(ScoreError::Negative(value));
    }
    Ok(value as u32)
}

fn example_04_custom_errors() {
    heading(4, "custom error enums");

    for input in ["90", "-2", "high"] {
        println!("{input:?} -> {:?}", parse_score(input));
    }
}

fn example_05_error_sources() {
    heading(5, "error sources");

    if let Err(error) = parse_score("high") {
        println!("display: {error}");
        println!("source: {:?}", error.source());
    }
}

impl From<ParseIntError> for ScoreError {
    fn from(error: ParseIntError) -> Self {
        Self::NotANumber(error)
    }
}

fn parse_score_with_from(text: &str) -> Result<u32, ScoreError> {
    let value = text.parse::<i32>()?; // `From` converts ParseIntError to ScoreError.
    u32::try_from(value).map_err(|_| ScoreError::Negative(value))
}

fn example_06_from_conversions() {
    heading(6, "From conversions");

    println!("{:?}", parse_score_with_from("55"));
}

fn port_or_default(text: Option<&str>) -> Result<u16, ParseIntError> {
    match text {
        Some(value) => value.parse(),
        None => Ok(8080),
    }
}

fn example_07_recovery_and_propagation() {
    heading(7, "recovery versus propagation");

    println!("missing recovers={:?}", port_or_default(None));
    println!("invalid propagates={:?}", port_or_default(Some("web")));
}

fn application_task(input: &str) -> Result<String, Box<dyn Error>> {
    let score = parse_score_with_from(input)?;
    Ok(format!("accepted score {score}"))
}

fn example_08_boxed_application_errors() {
    heading(8, "boxed application errors");

    for input in ["100", "bad"] {
        match application_task(input) {
            Ok(message) => println!("{message}"),
            Err(error) => println!("application error: {error}"),
        }
    }
}
