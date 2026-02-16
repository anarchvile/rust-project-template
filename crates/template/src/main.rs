//! A simple binary crate.

use template::add;

/// Entry-point to the binary crate.
fn main() {
    let a = 5;
    let b = 4;
    println!("{} + {} = {}", a, b, add(a, b));
}
