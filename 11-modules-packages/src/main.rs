//! Modules form a privacy boundary and a namespace within a crate.

mod garden;

mod inline {
    pub fn visible_message() -> &'static str {
        private_message()
    }

    fn private_message() -> &'static str {
        "private implementation reached through a public function"
    }
}

mod outer {
    pub mod inner {
        pub fn path() -> &'static str {
            "outer::inner"
        }
    }
}

mod visibility {
    pub struct Counter {
        pub value: u32,
        step: u32,
    }

    impl Counter {
        pub fn new(step: u32) -> Self {
            Self { value: 0, step }
        }

        pub fn tick(&mut self) {
            self.value += self.step;
        }
    }
}

mod paths {
    pub fn root_name() -> &'static str {
        crate::crate_name()
    }

    pub mod child {
        pub fn parent_value() -> u32 {
            super::shared_value()
        }
    }

    fn shared_value() -> u32 {
        42
    }
}

use std::collections::HashMap as Map;

// A public re-export gives callers a shorter, intentional path.
pub use garden::Garden;

fn heading(number: u8, title: &str) {
    println!("\n=== {number:02}: {title} ===");
}

fn crate_name() -> &'static str {
    "modules-packages"
}

fn main() {
    example_01_inline_modules();
    example_02_nested_modules();
    example_03_visibility();
    example_04_use_and_aliases();
    example_05_re_exports();
    example_06_super_and_crate_paths();
    example_07_filesystem_modules();
}

fn example_01_inline_modules() {
    heading(1, "inline modules");
    println!("{}", inline::visible_message());
}

fn example_02_nested_modules() {
    heading(2, "nested modules");
    println!("{}", outer::inner::path());
}

fn example_03_visibility() {
    heading(3, "public and private items");

    let mut counter = visibility::Counter::new(2);
    counter.tick();
    println!("public value={}", counter.value);
    // `counter.step` is private to the `visibility` module.
}

fn example_04_use_and_aliases() {
    heading(4, "use and aliases");

    let mut values = Map::new();
    values.insert("answer", 42);
    println!("{values:?}");
}

fn example_05_re_exports() {
    heading(5, "public re-exports");

    let garden = Garden::new("community");
    println!("re-exported Garden: {}", garden.name());
}

fn example_06_super_and_crate_paths() {
    heading(6, "super and crate paths");

    println!(
        "crate path={}; parent value={}",
        paths::root_name(),
        paths::child::parent_value()
    );
}

fn example_07_filesystem_modules() {
    heading(7, "filesystem modules");

    let carrot = garden::vegetables::Vegetable::new("carrot");
    println!(
        "{} grows in {}",
        carrot.name(),
        Garden::new("back garden").name()
    );
}
