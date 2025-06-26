# Rust Calling Python

A simple example of calling Python code from Rust using PyO3.

## Overview

This project demonstrates how to:
- Call Python functions from Rust
- Pass parameters between Rust and Python
- Handle Python errors in Rust

## Structure

- `src/main.rs`: Rust code that calls Python
- `math_ops.py`: Python module with a simple add function

## Requirements

- Rust (latest stable)
- Python 3.x
- PyO3 crate

## Running the project

```
cargo run
```

## How it works

The project uses PyO3 to interact with the Python interpreter from Rust. It:
1. Adds the current directory to Python's module search path
2. Imports the `math_ops` Python module
3. Calls the `add` function with parameters
4. Extracts the result back to Rust

This is just a simple demonstration, but PyO3 supports much more complex interactions between Rust and Python.
