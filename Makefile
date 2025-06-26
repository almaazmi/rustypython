# Makefile for building the C++ example that uses the Rust library

CXX = clang++
CXXFLAGS = -std=c++11 -Wall -Wextra

all: cpp_example

# Build the Rust library
librust_python_bridge.dylib:
	cargo build --release
	cp target/release/librust_python_bridge.dylib .

# Build the C++ example
cpp_example: librust_python_bridge.dylib cpp_example.cpp
	$(CXX) $(CXXFLAGS) -o cpp_example cpp_example.cpp -L. -lrust_python_bridge

clean:
	rm -f cpp_example librust_python_bridge.dylib
	cargo clean

.PHONY: all clean
