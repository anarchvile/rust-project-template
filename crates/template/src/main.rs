//! A simple binary crate.

use template::add;

/// Entry-point to the binary crate.
// GCOVR_EXCL_START
fn main() {
    let a = 5;
    let b = 4;
    println!("{} + {} = {}", a, b, add(a, b));
}
// GCOVR_EXCL_STOP
