use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDict, PyList};
use pyo3::exceptions::PyValueError;
use serde_json::{Value, Number, Map};

pub trait ToPython {
    fn to_python(&self, py: Python) -> PyResult<PyObject>;
}

pub trait FromPython: Sized {
    fn from_python(obj: &Bound<'_, PyAny>) -> PyResult<Self>;
}

impl ToPython for Value {
    fn to_python(&self, py: Python) -> PyResult<PyObject> {
        match self {
            Value::Null => Ok(py.None()),
            Value::Bool(b) => Ok(b.into_py(py)),
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Ok(i.into_py(py))
                } else if let Some(f) = n.as_f64() {
                    Ok(f.into_py(py))
                } else {
                    Ok(py.None()) // Should not happen for valid JSON numbers
                }
            }
            Value::String(s) => Ok(s.into_py(py)),
            Value::Array(arr) => {
                let list = PyList::empty_bound(py);
                for item in arr {
                    list.append(item.to_python(py)?)?;
                }
                Ok(list.into_py(py))
            }
            Value::Object(map) => {
                let dict = PyDict::new_bound(py);
                for (k, v) in map {
                    dict.set_item(k, v.to_python(py)?)?;
                }
                Ok(dict.into_py(py))
            }
        }
    }
}

impl FromPython for bool {
    fn from_python(obj: &Bound<'_, PyAny>) -> PyResult<Self> {
        obj.extract()
    }
}

impl FromPython for i64 {
    fn from_python(obj: &Bound<'_, PyAny>) -> PyResult<Self> {
        obj.extract()
    }
}

impl FromPython for f64 {
    fn from_python(obj: &Bound<'_, PyAny>) -> PyResult<Self> {
        obj.extract()
    }
}

impl FromPython for String {
    fn from_python(obj: &Bound<'_, PyAny>) -> PyResult<Self> {
        obj.extract()
    }
}

impl FromPython for Value {
    fn from_python(obj: &Bound<'_, PyAny>) -> PyResult<Self> {
        if obj.is_none() {
            return Ok(Value::Null);
        }
        if obj.is_instance_of::<PyBool>() {
             let b = bool::from_python(obj)?;
             return Ok(Value::Bool(b));
        }
        if let Ok(i) = i64::from_python(obj) {
            return Ok(Value::Number(Number::from(i)));
        }
        if let Ok(f) = f64::from_python(obj) {
             if let Some(n) = Number::from_f64(f) {
                return Ok(Value::Number(n));
            } else {
                 return Err(PyErr::new::<PyValueError, _>(
                    "NaN/Inf encountered in float conversion"
                ));
            }
        }
        if let Ok(s) = String::from_python(obj) {
            return Ok(Value::String(s));
        }
        if let Ok(list) = obj.downcast::<PyList>() {
            let mut vec = Vec::new();
            for item in list {
                vec.push(Value::from_python(&item)?);
            }
            return Ok(Value::Array(vec));
        }
        if let Ok(dict) = obj.downcast::<PyDict>() {
            let mut map = Map::new();
            for (k, v) in dict {
                let key = String::from_python(k.as_any())?;
                map.insert(key, Value::from_python(&v)?);
            }
            return Ok(Value::Object(map));
        }
        Ok(Value::String(obj.to_string()))
    }
}
