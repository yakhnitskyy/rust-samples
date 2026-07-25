//! A filesystem module declared by `mod garden;` in `main.rs`.

pub mod vegetables;

pub struct Garden {
    name: String,
}

impl Garden {
    pub fn new(name: &str) -> Self {
        Self { name: name.into() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
