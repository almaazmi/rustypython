#include <iostream>
#include <dlfcn.h>
#include "rust_python_bridge.h"

// Function pointer type
typedef int (*PythonAddFunc)(int, int);

int main() {
    // Open the shared library
    void* lib_handle = dlopen("./librust_python_bridge.dylib", RTLD_LAZY);
    if (!lib_handle) {
        std::cerr << "Error loading library: " << dlerror() << std::endl;
        return 1;
    }

    // Get the function from the shared library
    PythonAddFunc python_add_func = (PythonAddFunc)dlsym(lib_handle, "python_add");
    if (!python_add_func) {
        std::cerr << "Error getting function: " << dlerror() << std::endl;
        dlclose(lib_handle);
        return 1;
    }

    // Call the function
    int result = python_add_func(3, 5);
    std::cout << "Result from Python add via Rust: " << result << std::endl;

    // Close the shared library
    dlclose(lib_handle);
    return 0;
}
