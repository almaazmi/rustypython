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
- `ARCHITECTURE.md`: Detailed architecture diagrams

## Requirements

- Rust toolchain (rustc, cargo)
- C++ compiler (clang++ or g++)
- Python 3.x (with development headers)
- Make

## Step-by-Step Guide to Run the Project

### 1. Clone the Repository

```bash
git clone https://github.com/almaazmi/rustypython.git
cd rustypython
```

### 2. Ensure Dependencies are Installed

#### Check Rust
```bash
rustc --version
cargo --version
```

#### Check Python
```bash
python --version
```

#### Check C++ Compiler
```bash
clang++ --version  # or g++ --version
```

### 3. Build the Rust Library

```bash
# Build the library in release mode
cargo build --release

# Copy the library to the current directory for easier access
cp target/release/librust_python_bridge.dylib .  # On macOS
# cp target/release/librust_python_bridge.so .   # On Linux
# cp target/release/rust_python_bridge.dll .     # On Windows
```

### 4. Build the C++ Example

```bash
# The Makefile handles building both the Rust library and C++ executable
make
```

This command:
- Ensures the Rust library is built
- Compiles the C++ example code
- Links it against the Rust library

### 5. Run the C++ Example

```bash
./cpp_example
```

Expected output:
```
Result from Python add via Rust: 8
```

### 6. Clean Up (Optional)

```bash
make clean
```

## Troubleshooting

### Common Issues

1. **Python Module Not Found**
   - Ensure `math_ops.py` is in the same directory as the executable
   - Check if Python path is correctly set in the Rust code

2. **Library Loading Errors**
   - Make sure the library name matches your platform:
     - macOS: `librust_python_bridge.dylib`
     - Linux: `librust_python_bridge.so`
     - Windows: `rust_python_bridge.dll`
   - Ensure the library is in the correct path

3. **Compilation Errors**
   - Check that you have the correct versions of Rust, Python, and C++ installed
   - Make sure Python development headers are installed

## How it Works

1. The Rust library exports a C ABI function `python_add`.
2. This function uses PyO3 to initialize the Python interpreter and call the `add` function from `math_ops.py`.
3. The C++ application dynamically loads the Rust library and calls the `python_add` function.

## Advanced Usage

For more details on the architecture and how the components interact, see the `ARCHITECTURE.md` file.
