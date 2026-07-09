//! A simple binary crate.

use template::add;

/// Hard-coded addition and print statements. Separated out for code coverage purposes.
fn hard_coded_add() {
    let a = 5;
    let b = 4;
    println!("{} + {} = {}", a, b, add(a, b));
}

/// Entry-point to the binary crate.
fn main() {
    hard_coded_add();
}

/// Break the typical testing pattern outlined in the `README.md` because this is a
/// binary crate.
#[cfg(test)]
mod tests {
    #[test]
    fn hard_coded_add() {
        super::hard_coded_add();
    }
}
