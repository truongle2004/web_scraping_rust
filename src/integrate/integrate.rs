use std::ffi::CString;

use pyo3::{types::{PyAnyMethods, PyModule}, PyResult, Python};

pub fn greet_test() -> PyResult<()> {
    Python::with_gil(|py| {
        let script = std::fs::read_to_string("./business/logic.py").unwrap();
        let cs_script = CString::new(script).unwrap();
        let file_name = CString::new("logic.py").unwrap();
        let module_name = CString::new("logic").unwrap();
        let module = PyModule::from_code(py, &cs_script, &file_name, &module_name)?;

        let result: String = module.getattr("greet")?.call1(("rust",))?.extract().expect("REASON");
        println!("{}", result);
        Ok(())
    })
}