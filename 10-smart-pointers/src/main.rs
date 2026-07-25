//! Smart pointers combine pointer behavior with ownership-related guarantees.

use std::borrow::Cow;
use std::cell::RefCell;
use std::marker::PhantomPinned;
use std::ops::Deref;
use std::pin::Pin;
use std::rc::{Rc, Weak};
use std::sync::Arc;
use std::thread;

fn heading(number: u8, title: &str) {
    println!("\n=== {number:02}: {title} ===");
}

fn main() {
    example_01_box_and_recursive_data();
    example_02_deref();
    example_03_drop();
    example_04_rc_shared_ownership();
    example_05_refcell_interior_mutability();
    example_06_weak_links();
    example_07_arc_across_threads();
    example_08_cow_clone_on_write();
    example_09_pin_keeps_an_address_stable();
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn list_sum(list: &List) -> i32 {
    match list {
        List::Cons(value, next) => value + list_sum(next),
        List::Nil => 0,
    }
}

fn example_01_box_and_recursive_data() {
    heading(1, "Box and recursive data");

    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("{list:?}; sum={}", list_sum(&list));
}

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn greet(name: &str) {
    println!("hello, {name}");
}

fn example_02_deref() {
    heading(2, "Deref and deref coercion");

    let name = MyBox(String::from("Ferris"));
    greet(&name); // MyBox<String> -> String -> str through deref coercion.
}

struct TraceDrop(&'static str);

impl Drop for TraceDrop {
    fn drop(&mut self) {
        println!("dropping {}", self.0);
    }
}

fn example_03_drop() {
    heading(3, "Drop");

    let early = TraceDrop("early");
    let _at_scope_end = TraceDrop("scope-end");
    drop(early); // Call `std::mem::drop`; `early.drop()` is not allowed.
    println!("continuing after the explicit drop");
}

fn example_04_rc_shared_ownership() {
    heading(4, "Rc shared ownership");

    let shared = Rc::new(String::from("one allocation"));
    let first = Rc::clone(&shared);
    let second = Rc::clone(&shared);
    println!(
        "{first}; {second}; strong_count={}",
        Rc::strong_count(&shared)
    );
}

fn example_05_refcell_interior_mutability() {
    heading(5, "RefCell interior mutability");

    let shared_log = Rc::new(RefCell::new(Vec::new()));
    let another_owner = Rc::clone(&shared_log);
    shared_log.borrow_mut().push("first");
    another_owner.borrow_mut().push("second");
    println!("{:?}", shared_log.borrow());
}

#[derive(Debug)]
struct Node {
    name: &'static str,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn example_06_weak_links() {
    heading(6, "Weak links avoid ownership cycles");

    let parent = Rc::new(Node {
        name: "parent",
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(Vec::new()),
    });
    let child = Rc::new(Node {
        name: "child",
        parent: RefCell::new(Rc::downgrade(&parent)),
        children: RefCell::new(Vec::new()),
    });
    parent.children.borrow_mut().push(Rc::clone(&child));

    let parent_name = child
        .parent
        .borrow()
        .upgrade()
        .map(|node| node.name)
        .unwrap_or("none");
    println!("{} -> parent={parent_name}", child.name);
}

fn example_07_arc_across_threads() {
    heading(7, "Arc across threads");

    let shared = Arc::new(vec![2, 4, 6]);
    let worker_copy = Arc::clone(&shared);
    let worker = thread::spawn(move || worker_copy.iter().sum::<i32>());
    println!(
        "worker sum={}; main still has {shared:?}",
        worker.join().unwrap()
    );
}

fn normalize(input: &str) -> Cow<'_, str> {
    if input.chars().all(|character| !character.is_uppercase()) {
        Cow::Borrowed(input)
    } else {
        Cow::Owned(input.to_lowercase())
    }
}

fn example_08_cow_clone_on_write() {
    heading(8, "Cow clone-on-write");

    let unchanged = normalize("already lowercase");
    let changed = normalize("Needs Work");
    println!(
        "{unchanged:?}; borrowed={}",
        matches!(unchanged, Cow::Borrowed(_))
    );
    println!("{changed:?}; owned={}", matches!(changed, Cow::Owned(_)));
}

struct AddressSensitive {
    label: String,
    _pinned: PhantomPinned,
}

fn example_09_pin_keeps_an_address_stable() {
    heading(9, "introductory Pin");

    let pinned: Pin<Box<AddressSensitive>> = Box::pin(AddressSensitive {
        label: "fixed in place".into(),
        _pinned: PhantomPinned,
    });
    let address_before = &*pinned as *const AddressSensitive;
    let address_after = &*pinned as *const AddressSensitive;
    println!(
        "{}; address stable={}",
        pinned.as_ref().get_ref().label,
        address_before == address_after
    );
}
