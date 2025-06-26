// rust_python_bridge.h
#ifndef RUST_PYTHON_BRIDGE_H
#define RUST_PYTHON_BRIDGE_H

#ifdef __cplusplus
extern "C" {
#endif

// Declaration of the function exported from our Rust library
int python_add(int a, int b);

#ifdef __cplusplus
}
#endif

#endif // RUST_PYTHON_BRIDGE_H
