//! Serde converts between Rust types and data formats; serde_json handles JSON.

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::error::Error;
use std::fs;

fn heading(number: u8, title: &str) {
    println!("\n=== {number:02}: {title} ===");
}

fn main() -> Result<(), Box<dyn Error>> {
    example_01_construct_json();
    example_02_parse_into_value()?;
    example_03_traverse_safely()?;
    example_04_typed_deserialization()?;
    example_05_field_attributes_and_defaults()?;
    example_06_nested_structures()?;
    example_07_pretty_serialization()?;
    example_08_json_files()?;
    example_09_custom_validation()?;
    Ok(())
}

fn example_01_construct_json() {
    heading(1, "construct JSON");

    let document = json!({
        "language": "Rust",
        "stable": true,
        "features": ["safe", "fast", "concurrent"]
    });
    println!("{document}");
}

fn example_02_parse_into_value() -> serde_json::Result<()> {
    heading(2, "parse into Value");

    let value: Value = serde_json::from_str(r#"{"name":"Ferris","level":3}"#)?;
    println!("whole={value}; name={}", value["name"]);
    Ok(())
}

fn example_03_traverse_safely() -> serde_json::Result<()> {
    heading(3, "safe traversal");

    let value: Value = serde_json::from_str(r#"{"user":{"skills":["Rust","SQL"]}}"#)?;
    let first_skill = value
        .get("user")
        .and_then(|user| user.get("skills"))
        .and_then(Value::as_array)
        .and_then(|skills| skills.first())
        .and_then(Value::as_str);
    println!(
        "first skill={first_skill:?}; missing={:?}",
        value.get("missing")
    );
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    active: bool,
}

fn example_04_typed_deserialization() -> serde_json::Result<()> {
    heading(4, "typed deserialization");

    let user: User = serde_json::from_str(r#"{"id":7,"name":"Ada","active":true}"#)?;
    println!("{user:?}");
    Ok(())
}

fn default_role() -> String {
    "reader".into()
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Account {
    user_name: String,
    #[serde(default)]
    email: Option<String>,
    #[serde(default = "default_role")]
    role: String,
}

fn example_05_field_attributes_and_defaults() -> serde_json::Result<()> {
    heading(5, "field attributes and defaults");

    let account: Account = serde_json::from_str(r#"{"userName":"grace"}"#)?;
    println!(
        "user={}, email={:?}, role={}",
        account.user_name, account.email, account.role
    );
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct Project {
    name: String,
    owner: User,
    tags: Vec<String>,
}

fn example_06_nested_structures() -> serde_json::Result<()> {
    heading(6, "nested structures");

    let input = r#"{
        "name":"compiler",
        "owner":{"id":1,"name":"Lin","active":true},
        "tags":["rust","tools"]
    }"#;
    let project: Project = serde_json::from_str(input)?;
    println!(
        "{} owned by {}; tags={:?}",
        project.name, project.owner.name, project.tags
    );
    Ok(())
}

fn example_07_pretty_serialization() -> serde_json::Result<()> {
    heading(7, "pretty serialization");

    let user = User {
        id: 9,
        name: "Ferris".into(),
        active: true,
    };
    println!("{}", serde_json::to_string_pretty(&user)?);
    Ok(())
}

fn example_08_json_files() -> Result<(), Box<dyn Error>> {
    heading(8, "read and write a JSON file");

    let path = std::env::temp_dir().join("rust-learning-json-example.json");
    let user = User {
        id: 11,
        name: "Temporary".into(),
        active: false,
    };
    fs::write(&path, serde_json::to_vec_pretty(&user)?)?;
    let decoded: User = serde_json::from_slice(&fs::read(&path)?)?;
    println!("read {decoded:?} from {}", path.display());
    fs::remove_file(path)?;
    Ok(())
}

fn validate_user(user: User) -> Result<User, String> {
    if user.name.trim().is_empty() {
        Err("name must not be empty".into())
    } else if user.id == 0 {
        Err("id must be positive".into())
    } else {
        Ok(user)
    }
}

fn example_09_custom_validation() -> serde_json::Result<()> {
    heading(9, "validate decoded data");

    let user: User = serde_json::from_str(r#"{"id":0,"name":"","active":true}"#)?;
    println!("validation result={:?}", validate_user(user));
    Ok(())
}
