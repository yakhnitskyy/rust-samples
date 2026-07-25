//! `unsafe` permits specific operations; it does not disable the borrow checker.

use std::ptr::NonNull;
use std::sync::atomic::{AtomicUsize, Ordering};

static GLOBAL_COUNT: AtomicUsize = AtomicUsize::new(0);

fn heading(number: u8, title: &str) {
    println!("\n=== {number:02}: {title} ===");
}

fn main() {
    example_01_unsafe_blocks();
    example_02_unsafe_functions();
    example_03_raw_pointer_mutation();
    example_04_safe_wrapper();
    example_05_non_null_pointer();
    example_06_safe_global_state();
    example_07_send_and_sync();
    example_08_c_compatible_interface();
}

fn example_01_unsafe_blocks() {
    heading(1, "unsafe blocks");

    let value = 42;
    let pointer = &value as *const i32;
    // SAFETY: `pointer` came from a live reference and remains aligned and valid.
    let read_back = unsafe { *pointer };
    println!("raw pointer read={read_back}");
}

unsafe fn read_offset(pointer: *const i32, offset: usize) -> i32 {
    // Rust 2024 still requires the unsafe operation to be explicit in an unsafe fn.
    // SAFETY: the caller promises that `pointer.add(offset)` is valid to read.
    unsafe { *pointer.add(offset) }
}

fn example_02_unsafe_functions() {
    heading(2, "unsafe functions");

    let values = [10, 20, 30];
    // SAFETY: the array is alive and index 1 is inside its three elements.
    let value = unsafe { read_offset(values.as_ptr(), 1) };
    println!("offset value={value}");
}

fn example_03_raw_pointer_mutation() {
    heading(3, "raw-pointer mutation");

    let mut value = 7;
    let pointer = &mut value as *mut i32;
    // SAFETY: this pointer is unique, aligned, and derived from a live mutable reference.
    unsafe {
        *pointer *= 6;
    }
    println!("mutated value={value}");
}

fn split_at_mut(values: &mut [i32], middle: usize) -> (&mut [i32], &mut [i32]) {
    assert!(middle <= values.len());
    let length = values.len();
    let pointer = values.as_mut_ptr();
    // SAFETY: `middle <= length`; the two constructed ranges are valid,
    // non-overlapping, and together cover the original exclusive slice.
    unsafe {
        (
            std::slice::from_raw_parts_mut(pointer, middle),
            std::slice::from_raw_parts_mut(pointer.add(middle), length - middle),
        )
    }
}

fn example_04_safe_wrapper() {
    heading(4, "safe wrapper around unsafe code");

    let mut values = [1, 2, 3, 4];
    let (left, right) = split_at_mut(&mut values, 2);
    left[0] = 10;
    right[0] = 30;
    println!("{values:?}");
}

fn example_05_non_null_pointer() {
    heading(5, "NonNull");

    let mut value = String::from("valid allocation");
    let pointer = NonNull::from(&mut value);
    // SAFETY: `value` is alive, has not moved, and `pointer` was made from `&mut value`.
    let borrowed = unsafe { pointer.as_ref() };
    println!("{borrowed}");
}

fn example_06_safe_global_state() {
    heading(6, "safe global state");

    // `static mut` would require unsafe synchronization. Atomics encode the rule safely.
    GLOBAL_COUNT.fetch_add(1, Ordering::Relaxed);
    GLOBAL_COUNT.fetch_add(1, Ordering::Relaxed);
    println!(
        "global atomic count={}",
        GLOBAL_COUNT.load(Ordering::Relaxed)
    );
}

fn assert_send_and_sync<T: Send + Sync>() {}

fn example_07_send_and_sync() {
    heading(7, "Send and Sync");

    assert_send_and_sync::<std::sync::Arc<std::sync::Mutex<Vec<i32>>>>();
    println!("Arc<Mutex<Vec<i32>>> satisfies Send + Sync");
    println!("Unsafe implementations are promises that must uphold thread-safety invariants.");
}

// Rust 2024 treats symbol export as an unsafe attribute because duplicate symbols
// can violate linker assumptions. The function body itself is safe.
#[unsafe(no_mangle)]
pub extern "C" fn rust_add(left: i32, right: i32) -> i32 {
    left + right
}

fn example_08_c_compatible_interface() {
    heading(8, "C-compatible interface");

    println!("extern \"C\" rust_add(20, 22)={}", rust_add(20, 22));
    println!("Real FFI must also define ownership, nullability, and panic boundaries.");
}
