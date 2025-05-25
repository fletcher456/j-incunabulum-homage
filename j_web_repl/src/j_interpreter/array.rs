// J Array implementation
use std::fmt;
use crate::j_interpreter::JError;

// J arrays can be either numeric arrays or boxed arrays
#[derive(Debug, Clone)]
pub enum JArray {
    // Numeric array with rank and data
    Numeric {
        shape: Vec<usize>,
        data: Vec<f64>,
    },
    // Boxed array (containing other arrays)
    Boxed {
        shape: Vec<usize>,
        data: Vec<Box<JArray>>,
    },
}

impl JArray {
    // Create a scalar numeric value
    pub fn scalar(value: f64) -> Self {
        JArray::Numeric {
            shape: vec![],
            data: vec![value],
        }
    }

    // Create a vector of numeric values
    pub fn vector(values: Vec<f64>) -> Self {
        let len = values.len();
        JArray::Numeric {
            shape: vec![len],
            data: values,
        }
    }

    // Create a shaped array from a vector
    pub fn shaped(shape: Vec<usize>, values: Vec<f64>) -> Result<Self, JError> {
        let expected_size: usize = shape.iter().product();
        if values.len() != expected_size {
            return Err(JError::InvalidArgument(format!(
                "Data size mismatch: expected {}, got {}",
                expected_size, values.len()
            )));
        }
        
        Ok(JArray::Numeric {
            shape,
            data: values,
        })
    }

    // Create a boxed array from other arrays
    pub fn boxed(arrays: Vec<JArray>) -> Self {
        let len = arrays.len();
        JArray::Boxed {
            shape: vec![len],
            data: arrays.into_iter().map(Box::new).collect(),
        }
    }

    // Get the rank (number of dimensions)
    pub fn rank(&self) -> usize {
        match self {
            JArray::Numeric { shape, .. } => shape.len(),
            JArray::Boxed { shape, .. } => shape.len(),
        }
    }

    // Get the shape (dimensions)
    pub fn shape(&self) -> &[usize] {
        match self {
            JArray::Numeric { shape, .. } => shape,
            JArray::Boxed { shape, .. } => shape,
        }
    }

    // Get the total size of the array
    pub fn size(&self) -> usize {
        match self {
            JArray::Numeric { data, .. } => data.len(),
            JArray::Boxed { data, .. } => data.len(),
        }
    }

    // Check if array is empty
    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    // Get a value at specified indices
    pub fn get(&self, indices: &[usize]) -> Result<&f64, JError> {
        match self {
            JArray::Numeric { shape, data } => {
                if indices.len() != shape.len() {
                    return Err(JError::InvalidArgument(format!(
                        "Index rank mismatch: expected {}, got {}",
                        shape.len(), indices.len()
                    )));
                }
                
                let index = calculate_flat_index(shape, indices)?;
                data.get(index).ok_or_else(|| {
                    JError::InvalidArgument(format!("Index out of bounds: {:?}", indices))
                })
            }
            JArray::Boxed { .. } => Err(JError::InvalidArgument(
                "Cannot get numeric value from boxed array".to_string(),
            )),
        }
    }

    // Get a boxed value at specified indices
    pub fn get_boxed(&self, indices: &[usize]) -> Result<&JArray, JError> {
        match self {
            JArray::Boxed { shape, data } => {
                if indices.len() != shape.len() {
                    return Err(JError::InvalidArgument(format!(
                        "Index rank mismatch: expected {}, got {}",
                        shape.len(), indices.len()
                    )));
                }
                
                let index = calculate_flat_index(shape, indices)?;
                data.get(index).map(|b| b.as_ref()).ok_or_else(|| {
                    JError::InvalidArgument(format!("Index out of bounds: {:?}", indices))
                })
            }
            JArray::Numeric { .. } => Err(JError::InvalidArgument(
                "Cannot get boxed value from numeric array".to_string(),
            )),
        }
    }

    // Convert to a f64 if it's a scalar
    pub fn as_scalar(&self) -> Result<f64, JError> {
        match self {
            JArray::Numeric { shape, data } if shape.is_empty() && data.len() == 1 => {
                Ok(data[0])
            }
            _ => Err(JError::InvalidArgument("Not a scalar".to_string())),
        }
    }

    // Check if array is a scalar
    pub fn is_scalar(&self) -> bool {
        match self {
            JArray::Numeric { shape, data } => shape.is_empty() && data.len() == 1,
            _ => false,
        }
    }

    // Check if array is a vector
    pub fn is_vector(&self) -> bool {
        match self {
            JArray::Numeric { shape, .. } => shape.len() == 1,
            JArray::Boxed { shape, .. } => shape.len() == 1,
        }
    }

    // Convert to a vector if possible
    pub fn as_vector(&self) -> Result<&[f64], JError> {
        match self {
            JArray::Numeric { shape, data } if shape.len() == 1 || shape.is_empty() => {
                Ok(data)
            }
            _ => Err(JError::InvalidArgument("Not a vector or scalar".to_string())),
        }
    }
}

// Calculate the flat index from a multidimensional index
fn calculate_flat_index(shape: &[usize], indices: &[usize]) -> Result<usize, JError> {
    if indices.len() != shape.len() {
        return Err(JError::InvalidArgument(
            "Index dimensions don't match array rank".to_string(),
        ));
    }

    // Check if any index is out of bounds
    for (i, &idx) in indices.iter().enumerate() {
        if idx >= shape[i] {
            return Err(JError::InvalidArgument(format!(
                "Index out of bounds: {} >= {} at dimension {}",
                idx, shape[i], i
            )));
        }
    }

    // Calculate the flat index
    let mut flat_index = 0;
    let mut stride = 1;
    
    // Go from the rightmost dimension to the leftmost
    for i in (0..shape.len()).rev() {
        flat_index += indices[i] * stride;
        stride *= shape[i];
    }

    Ok(flat_index)
}

// Display implementation for JArray
impl fmt::Display for JArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JArray::Numeric { shape, data } => {
                if shape.is_empty() {
                    // Scalar
                    write!(f, "{}", data[0])
                } else if shape.len() == 1 {
                    // Vector
                    write!(f, "[")?;
                    for (i, val) in data.iter().enumerate() {
                        if i > 0 {
                            write!(f, " ")?;
                        }
                        write!(f, "{}", val)?;
                    }
                    write!(f, "]")
                } else {
                    // Higher-dimensional array
                    write_shaped_array(f, shape, data)
                }
            }
            JArray::Boxed { shape, data } => {
                if shape.is_empty() {
                    // Scalar box
                    write!(f, "<{}>", data[0])
                } else if shape.len() == 1 {
                    // Vector of boxes
                    write!(f, "[")?;
                    for (i, boxed) in data.iter().enumerate() {
                        if i > 0 {
                            write!(f, " ")?;
                        }
                        write!(f, "<{}>", boxed)?;
                    }
                    write!(f, "]")
                } else {
                    // Higher-dimensional boxed array
                    write!(f, "Boxed array of shape {:?}", shape)
                }
            }
        }
    }
}

// Helper function to pretty-print shaped arrays
fn write_shaped_array(
    f: &mut fmt::Formatter<'_>,
    shape: &[usize],
    data: &[f64],
) -> fmt::Result {
    if shape.len() == 2 {
        // Matrix
        let rows = shape[0];
        let cols = shape[1];
        
        writeln!(f, "[")?;
        for row in 0..rows {
            write!(f, " [")?;
            for col in 0..cols {
                let idx = row * cols + col;
                if col > 0 {
                    write!(f, " ")?;
                }
                write!(f, "{}", data[idx])?;
            }
            writeln!(f, "]")?;
        }
        write!(f, "]")
    } else {
        // Higher dimensions just show shape and flattened data
        write!(f, "Array of shape {:?}: {:?}", shape, data)
    }
}

// Test module for JArray
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scalar_creation() {
        let scalar = JArray::scalar(42.0);
        assert_eq!(scalar.rank(), 0);
        assert_eq!(scalar.size(), 1);
        assert!(scalar.is_scalar());
    }

    #[test]
    fn test_vector_creation() {
        let vector = JArray::vector(vec![1.0, 2.0, 3.0]);
        assert_eq!(vector.rank(), 1);
        assert_eq!(vector.size(), 3);
        assert!(vector.is_vector());
    }

    #[test]
    fn test_shaped_array() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let array = JArray::shaped(vec![2, 3], data).unwrap();
        assert_eq!(array.rank(), 2);
        assert_eq!(array.size(), 6);
        assert_eq!(array.shape(), &[2, 3]);
    }
}