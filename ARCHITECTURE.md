# Rust-Python-C++ Integration Architecture

```
+-------------------------------------+
|                                     |
|         C++ Application             |
|  +----------------------------+     |
|  |                            |     |
|  |  // Load Rust library      |     |
|  |  void* lib = dlopen(...);  |     |
|  |                            |     |
|  |  // Get function pointer   |     |
|  |  auto func = dlsym(...);   |     |
|  |                            |     |
|  |  // Call Rust function     |     |
|  |  int result = func(3, 5);  |     |
|  |                            |     |
|  +------------+---------------+     |
|               |                     |
+---------------|---------------------+
                |
                | Dynamic Library Call
                v
+-------------------------------------+
|                                     |
|         Rust Dynamic Library        |
|  +----------------------------+     |
|  | #[no_mangle]               |     |
|  | pub extern "C" fn          |     |
|  | python_add(a, b) {         |     |
|  |                            |     |
|  |   // Initialize Python     |     |
|  |   Python::with_gil(|py| {  |     |
|  |     // Call Python         |     |
|  |     let module =           |     |
|  |       PyModule::import(..);|     |
|  |     module.call(...);      |     |
|  |   })                       |     |
|  | }                          |     |
|  +------------+---------------+     |
|               |                     |
+---------------|---------------------+
                |
                | Python Interpreter Call
                v
+-------------------------------------+
|                                     |
|         Python Module               |
|  +----------------------------+     |
|  |                            |     |
|  | def add(a, b):             |     |
|  |     return a + b           |     |
|  |                            |     |
|  +----------------------------+     |
|                                     |
+-------------------------------------+
```

## Data Flow

```
+--------+    Function Call    +--------+    Python Call    +--------+
|        | ------------------> |        | ----------------> |        |
|  C++   |  int result = f()   |  Rust  |  module.call()    | Python |
|  App   | <------------------ |  DLL   | <---------------- | Module |
|        |   Returns result    |        |  Returns result   |        |
+--------+                     +--------+                   +--------+
```

## Component Responsibilities

```
+-------------------+-------------------+-------------------+
|      C++ App      |     Rust DLL      |   Python Module   |
+-------------------+-------------------+-------------------+
| • Load library    | • Expose C ABI    | • Define logic    |
| • Resolve symbols | • Init Python     | • Process data    |
| • Call functions  | • Manage memory   | • Return results  |
| • Handle results  | • Error handling  | • Define API      |
+-------------------+-------------------+-------------------+
```

## Dependencies

```
   C++ Application
         |
         | Depends on
         v
   Rust DLL (librust_python_bridge)
         |
         | Depends on
         v
   Python Installation
         |
         | Depends on
         v
   Python Module (math_ops.py)
```

## Key Files

```
project/
|
├── cpp_example.cpp    # C++ application that calls the Rust library
├── rust_python_bridge.h  # C header for the Rust library interface
├── Cargo.toml         # Rust project configuration
├── src/
|   └── lib.rs         # Rust library implementation
└── math_ops.py        # Python module with implementation
```

## C++ Compilation Process

```
+---------------------------------------------------+
|                                                   |
|           C++ Compilation Process                 |
|                                                   |
| +-------------------+      +-------------------+  |
| |                   |      |                   |  |
| | cpp_example.cpp   |      | rust_python       |  |
| | (Source Code)     |      | _bridge.h         |  |
| |                   |      | (Header)          |  |
| +--------+----------+      +---------+---------+  |
|          |                           |            |
|          v                           v            |
| +-----------------------------------------+       |
| |                                         |       |
| |            clang++ Compiler             |       |
| |  clang++ -std=c++11 -Wall -Wextra       |       |
| |  -o cpp_example cpp_example.cpp         |       |
| |  -L. -lrust_python_bridge               |       |
| |                                         |       |
| +------------------+----------------------+       |
|                    |                              |
|                    v                              |
| +------------------------------------------+      |
| |                                          |      |
| |            cpp_example                   |      |
| |            (Executable)                  |      |
| |                                          |      |
| +------------------------------------------+      |
|                                                   |
+---------------------------------------------------+

Compiler Flags Explained:
-----------------------
-std=c++11   : Use C++11 standard
-Wall -Wextra: Enable comprehensive warnings
-o cpp_example: Output executable name
-L.          : Look for libraries in current directory
-lrust_python_bridge: Link against librust_python_bridge.dylib
```

## Dynamic Linking Process

```
+---------------------------------------------------+
|                                                   |
|          Dynamic Linking at Runtime               |
|                                                   |
| +-----------------+      +---------------------+  |
| |                 |      |                     |  |
| | cpp_example     |      | librust_python      |  |
| | (Executable)    +----->+ _bridge.dylib       |  |
| |                 |      | (Shared Library)    |  |
| +-----------------+      +----------+----------+  |
|                                     |             |
|                                     v             |
|                          +---------------------+  |
|                          |                     |  |
|                          | Python Interpreter  |  |
|                          | & math_ops.py       |  |
|                          |                     |  |
|                          +---------------------+  |
|                                                   |
+---------------------------------------------------+

Dynamic Loading Code:
------------------
void* lib_handle = dlopen("./librust_python_bridge.dylib", RTLD_LAZY);
PythonAddFunc python_add_func = (PythonAddFunc)dlsym(lib_handle, "python_add");
int result = python_add_func(3, 5);
dlclose(lib_handle);
```
