use pyo3::prelude::*;
use pyo3::types::PyModule;
use std::env;

fn main() -> PyResult<()> {
    // Get the current directory to add to Python's path
    let current_dir = env::current_dir().expect("Failed to get current directory");
    
    Python::with_gil(|py| {
        // Add the current directory to Python's sys.path
        let sys = PyModule::import_bound(py, "sys")?;
        let path = sys.getattr("path")?;
        path.call_method1("append", (current_dir.to_str().unwrap(),))?;
        
        // Now import our module
        let module = PyModule::import_bound(py, "math_ops")?;
        let add = module.getattr("add")?;
        let result: i32 = add.call1((3, 5))?.extract()?;
        println!("Python add(3, 5) = {}", result);
        Ok(())
    })
}