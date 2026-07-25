//! A nested filesystem module declared by `pub mod vegetables;`.

pub struct Vegetable {
    name: String,
}

impl Vegetable {
    pub fn new(name: &str) -> Self {
        Self { name: name.into() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
