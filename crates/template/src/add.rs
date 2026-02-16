//! This module contains utilities related to addition.

/// A saturating addition method implementation.
#[must_use]
pub fn add(a: i32, b: i32) -> i32 {
    a.saturating_add(b)
}

#[cfg(test)]
mod tests;
