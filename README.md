# Rust Python Bridge

This project demonstrates how to create a Rust dynamic library (DLL) that calls Python functions and can be used from C++.

## Components

1. **Rust Library**: A dynamic library that exports a C-compatible function which calls a Python function.
2. **Python Module**: A simple Python module with an `add` function.
3. **C++ Example**: A sample C++ application that loads the Rust library and calls its function.

## Structure

- `src/lib.rs`: Rust library code that exports a C-compatible function
- `math_ops.py`: Python module with a simple add function
- `cpp_example.cpp`: C++ example that loads and uses the Rust library
- `rust_python_bridge.h`: C header for the exported Rust function

## Building and Running

### Build the Rust Library

```bash
cargo build --release
```

The library will be created at `target/release/librust_python_bridge.dylib` (macOS) or `target/release/rust_python_bridge.dll` (Windows) or `target/release/librust_python_bridge.so` (Linux).

### Build and Run the C++ Example

```bash
make
./cpp_example
```

## How it Works

1. The Rust library exports a C ABI function `python_add`.
2. This function uses PyO3 to initialize the Python interpreter and call the `add` function from `math_ops.py`.
3. The C++ application dynamically loads the Rust library and calls the `python_add` function.

## Requirements

- Rust toolchain
- C++ compiler
- Python (with development headers)
