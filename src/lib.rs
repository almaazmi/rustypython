use pyo3::prelude::*;
use pyo3::types::PyModule;
use std::env;
use std::path::Path;
use std::ffi::c_int;

// Function that will be exported to C/C++
#[no_mangle]
pub extern "C" fn python_add(a: c_int, b: c_int) -> c_int {
    // Initialize Python and handle errors safely in an FFI context
    match run_python_add(a, b) {
        Ok(result) => result,
        Err(_) => {
            eprintln!("Error calling Python function");
            0 // Return 0 on error
        }
    }
}

// Internal function that does the actual Python calling
fn run_python_add(a: c_int, b: c_int) -> Result<c_int, PyErr> {
    // Get the current executable's directory to find the Python script
    let exe_path = env::current_exe().expect("Failed to get executable path");
    let exe_dir = exe_path.parent().expect("Failed to get executable directory");
    
    Python::with_gil(|py| {
        // Add the directory containing our Python script to sys.path
        let sys = PyModule::import_bound(py, "sys")?;
        let path = sys.getattr("path")?;
        path.call_method1("append", (exe_dir.to_str().unwrap(),))?;
        
        // Also add the current directory in case the DLL is being used from another location
        let current_dir = env::current_dir().expect("Failed to get current directory");
        path.call_method1("append", (current_dir.to_str().unwrap(),))?;
        
        // Import our Python module
        let module = PyModule::import_bound(py, "math_ops")?;
        let add = module.getattr("add")?;
        let result: c_int = add.call1((a, b))?.extract()?;
        
        Ok(result)
    })
}

// Optional: Add a test to verify our function works
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_python_add() {
        assert_eq!(python_add(3, 5), 8);
    }
}
