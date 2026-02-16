[![Version](https://img.shields.io/badge/version-1.0.2-blue)](Cargo.toml) <!-- badge:version -->
[![CI](https://github.com/anarchvile/rust-project-template/actions/workflows/ci.yml/badge.svg)](https://github.com/anarchvile/rust-project-template/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/badge/coverage-84.3%25-green)](https://github.com/anarchvile/rust-project-template/actions/workflows/ci.yml) <!-- badge:coverage -->

# Rust Project Template

This is a small template repo meant to be copied and used as a starting point for building Rust-based projects (with
specialization towards GitHub). Features include:
- [`cargo-nextest`](https://nexte.st/docs/installation/from-source/)-based testing set-up.
- [`criterion`](https://github.com/bheisler/criterion.rs)-based benchmarking.
- Small Cargo workspace configuration with a single `template` package containing a binary and library crate.
- GitHub actions-based CI pipeline for building, formatting, linting, testing (with code coverage), and benchmarking
  each PR made against the repo, and posting comments to PRs with the results.

## Prerequisites

The following tools are recommended for building and running this project:

- [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html)
- [`cargo-nextest`](https://nexte.st/docs/installation/from-source/)
  * `cargo install --locked cargo-nextest`

## Commands

```bash
# Check source code is valid.
cargo check --workspace
# Apply auto-formatting.
cargo fmt --all -- --config wrap_comments=true,comment_width=120
# Linting (runs `pedantic` in CI as well).
cargo clippy --workspace --all-targets --all-features -- -W clippy::pedantic
# Run default tests.
cargo nextest run
# Run all benchmarks.
cargo bench
# Full build (debug) - check is faster and sufficient, run is an alternative.
cargo build
# Full build (release) - this is usually not needed for "typical" development purposes.
cargo build --release
# Run the binary crate.
cargo run
```

## Code structure

### Modules

Modules are structured as follows:

```text
foo.rs
foo/tests.rs
```

This leads to a flatter code coverage report.

### Tests

Strongly prefer a separate tests file. Yes:

```rust
#[cfg(test)]
mod tests;
```

No:

```rust
#[cfg(test)]
mod tests {
    // ...
}
```

This is required for code coverage to be able to ignore test code, but also speeds up test compilation.

Non-integration tests should be in `crates/*/src/**/tests.rs` (or sub-modules of `tests.rs` if necessary to split them
up into more readable chunks).