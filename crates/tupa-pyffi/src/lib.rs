use once_cell::sync::Lazy;
use pyo3::prelude::*;
use pyo3::types::{PyModule, PyTuple};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Mutex;

pub mod serialize;
use serialize::{FromPython, ToPython};

pub static BRIDGE: Lazy<Mutex<PythonBridge>> = Lazy::new(|| Mutex::new(PythonBridge::default()));

pub struct PythonBridge {
    modules: HashMap<String, Py<PyModule>>,
}

impl Default for PythonBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl PythonBridge {
    pub fn new() -> Self {
        pyo3::prepare_freethreaded_python();
        Self {
            modules: HashMap::new(),
        }
    }

    pub fn ensure_module(&mut self, name: &str) -> PyResult<()> {
        if self.modules.contains_key(name) {
            return Ok(());
        }
        Python::with_gil(|py| {
            let module = PyModule::import_bound(py, name)?;
            self.modules.insert(name.to_string(), module.unbind());
            Ok(())
        })
    }

    pub fn call(&self, module_name: &str, func_name: &str, arg: Value) -> PyResult<Value> {
        Python::with_gil(|py| {
            let module = self.modules.get(module_name).ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyImportError, _>(format!(
                    "Module {} not loaded",
                    module_name
                ))
            })?;
            let func = module.bind(py).getattr(func_name)?;
            let py_arg = arg.to_python(py)?;
            let result = func.call1((py_arg,))?;
            Value::from_python(&result)
        })
    }

    pub fn call_with_multiple_args(
        &self,
        module_name: &str,
        func_name: &str,
        args: Vec<Value>,
    ) -> PyResult<Value> {
        Python::with_gil(|py| {
            let module = self.modules.get(module_name).ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyImportError, _>(format!(
                    "Module {} not loaded",
                    module_name
                ))
            })?;
            let func = module.bind(py).getattr(func_name)?;
            let py_args: Vec<PyObject> = args.iter().map(|a| a.to_python(py).unwrap()).collect();
            let tuple = PyTuple::new_bound(py, &py_args);
            let result = func.call1(tuple)?;
            Value::from_python(&result)
        })
    }

    pub fn reset(&mut self) {
        self.modules.clear();
    }
}

pub fn call_python_function(module: &str, func: &str, arg: Value) -> Result<Value, String> {
    let mut bridge = BRIDGE.lock().map_err(|e| e.to_string())?;
    bridge.ensure_module(module).map_err(|e| e.to_string())?;
    bridge.call(module, func, arg).map_err(|e| e.to_string())
}

pub fn call_python_function_with_args(
    module: &str,
    func: &str,
    args: Vec<Value>,
) -> Result<Value, String> {
    let mut bridge = BRIDGE.lock().map_err(|e| e.to_string())?;
    bridge.ensure_module(module).map_err(|e| e.to_string())?;
    bridge
        .call_with_multiple_args(module, func, args)
        .map_err(|e| e.to_string())
}

pub fn reset_python_bridge() {
    let mut bridge = BRIDGE.lock().unwrap();
    bridge.reset();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_math_sqrt() {
        let mut bridge = BRIDGE.lock().unwrap();
        bridge.ensure_module("math").unwrap();
        let result = bridge
            .call("math", "sqrt", serde_json::json!(16.0))
            .unwrap();
        assert_eq!(result, serde_json::json!(4.0));
    }

    #[test]
    fn test_math_pow() {
        let mut bridge = BRIDGE.lock().unwrap();
        bridge.ensure_module("math").unwrap();
        let result = bridge
            .call_with_multiple_args(
                "math",
                "pow",
                vec![serde_json::json!(2.0), serde_json::json!(3.0)],
            )
            .unwrap();
        assert_eq!(result, serde_json::json!(8.0));
    }

    #[test]
    fn test_module_not_loaded_error() {
        reset_python_bridge();
        let result = call_python_function("nonexistent", "func", serde_json::json!(0)).unwrap_err();
        assert!(
            result.contains("nonexistent") || result.contains("not found"),
            "got: {}",
            result
        );
    }

    #[test]
    fn test_reset_clears_modules() {
        // Hold the lock for the entire test so no concurrent test can
        // re-insert "math" between reset() and the final assertion.
        let mut bridge = BRIDGE.lock().unwrap();
        bridge.ensure_module("math").unwrap();
        assert!(bridge.modules.contains_key("math"));
        bridge.reset();
        assert!(!bridge.modules.contains_key("math"));
    }
}
