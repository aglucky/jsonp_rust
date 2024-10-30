# Rust JSON Parser

A toy implementation of a JSON parser.

## Prerequisites

- Rust
- Cargo (Rust's package manager)

You can install both by following the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).

## Usage

Run the parser with a JSON file:

```bash
cargo run -- <JSON_PATH>
```

## Testing

Run the test suite:

```bash
cargo test
```

## Features

- Parse JSON objects and arrays
- Support for all JSON data types (strings, numbers, booleans, null)
- Error handling with detailed messages

## TODO

- [ ] Add recursive parser implementation once tail recursion is officially supported in Rust
