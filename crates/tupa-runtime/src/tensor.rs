use tupa_parser::TensorType;
use ndarray::ArrayD;
use thiserror::Error;
use serde_json::Value;

#[derive(Debug, Error)]
pub enum Error {
    #[error("shape mismatch: expected {expected:?}, actual {actual:?}")]
    ShapeMismatch {
        expected: Vec<Option<usize>>,
        actual: Vec<usize>,
    },
    #[error("invalid tensor format in JSON")]
    InvalidJsonFormat,
}

type Result<T> = std::result::Result<T, Error>;

pub fn validate_shape(
    actual: &[usize],
    expected: &TensorType,
) -> Result<()> {
    if actual.len() != expected.shape.len() {
        return Err(Error::ShapeMismatch {
            expected: expected.shape.clone(),
            actual: actual.to_vec(),
        });
    }
    
    for (exp_dim, act_dim) in expected.shape.iter().zip(actual.iter()) {
        if let Some(exp_size) = exp_dim {
            if *exp_size != *act_dim {
                return Err(Error::ShapeMismatch {
                    expected: expected.shape.clone(),
                    actual: actual.to_vec(),
                });
            }
        }
    }
    
    Ok(())
}

pub fn validate_tensor_shape<A>(
    actual: &ArrayD<A>,
    expected: &TensorType,
) -> Result<()> {
    validate_shape(actual.shape(), expected)
}

pub fn get_json_shape(v: &Value) -> Result<Vec<usize>> {
    match v {
        Value::Array(arr) => {
            if arr.is_empty() {
                return Ok(vec![0]);
            }
            let sub_shape = get_json_shape(&arr[0])?;
            for item in arr.iter().skip(1) {
                let item_shape = get_json_shape(item)?;
                if item_shape != sub_shape {
                    return Err(Error::InvalidJsonFormat);
                }
            }
            let mut shape = vec![arr.len()];
            shape.extend(sub_shape);
            Ok(shape)
        }
        Value::Number(_) | Value::Bool(_) => Ok(vec![]),
        _ => Err(Error::InvalidJsonFormat),
    }
}

pub fn validate_json_tensor(v: &Value, expected: &TensorType) -> Result<()> {
    let shape = get_json_shape(v)?;
    validate_shape(&shape, expected)
}
