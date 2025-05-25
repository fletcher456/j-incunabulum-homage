// Implementation of J verbs (operations)
use crate::j_interpreter::array::JArray;
use crate::j_interpreter::JError;

// Execute a verb with given arguments
pub fn execute_verb(left: Option<&JArray>, verb: char, right: &JArray) -> Result<JArray, JError> {
    match (left, verb) {
        // Dyadic verbs (two arguments)
        (Some(left), '+') => plus(left, right),
        (Some(left), '{') => from(left, right),
        (Some(left), '~') => find(left, right),
        (Some(left), '#') => reshape(left, right),
        (Some(left), ',') => concatenate(left, right),
        
        // Monadic verbs (one argument)
        (None, '+') => identity(right),
        (None, '{') => size(right),
        (None, '~') => iota(right),
        (None, '<') => box_array(right),
        (None, '#') => shape(right),
        
        // Unsupported verb
        (_, verb) => Err(JError::ExecutionError(format!("Unsupported verb: '{}'", verb))),
    }
}

// MONADIC VERBS (single argument)

// Identity function - return the array unchanged (+ monad)
pub fn identity(right: &JArray) -> Result<JArray, JError> {
    Ok(right.clone())
}

// Size function - return the size of the first dimension ({ monad)
pub fn size(right: &JArray) -> Result<JArray, JError> {
    let shape = right.shape();
    if shape.is_empty() {
        // Scalar has size 1
        return Ok(JArray::scalar(1.0));
    }
    
    // Return the size of the first dimension
    Ok(JArray::scalar(shape[0] as f64))
}

// Iota function - generate array [0,1,2,...,n-1] (~ monad)
pub fn iota(right: &JArray) -> Result<JArray, JError> {
    // Extract the size from the right argument
    let n = match right.is_scalar() {
        true => right.as_scalar()? as usize,
        false => {
            return Err(JError::InvalidArgument(
                "Iota requires a scalar argument".to_string(),
            ))
        }
    };
    
    if n == 0 {
        return Ok(JArray::vector(vec![]));
    }
    
    // Generate vector from 0 to n-1
    let data: Vec<f64> = (0..n).map(|i| i as f64).collect();
    Ok(JArray::vector(data))
}

// Box function - create a box containing the array (< monad)
pub fn box_array(right: &JArray) -> Result<JArray, JError> {
    Ok(JArray::boxed(vec![right.clone()]))
}

// Shape function - return the dimensions of an array (# monad)
pub fn shape(right: &JArray) -> Result<JArray, JError> {
    let shape_data: Vec<f64> = right.shape().iter().map(|&s| s as f64).collect();
    Ok(JArray::vector(shape_data))
}

// DYADIC VERBS (two arguments)

// Plus function - element-wise addition (+ dyad)
pub fn plus(left: &JArray, right: &JArray) -> Result<JArray, JError> {
    match (left, right) {
        // Scalar + scalar
        (JArray::Numeric { shape: shape_a, data: data_a }, 
         JArray::Numeric { shape: shape_b, data: data_b })
            if shape_a.is_empty() && shape_b.is_empty() => {
                Ok(JArray::scalar(data_a[0] + data_b[0]))
            }
        
        // Scalar + vector
        (JArray::Numeric { shape: shape_a, data: data_a }, 
         JArray::Numeric { shape: shape_b, data: data_b })
            if shape_a.is_empty() => {
                let scalar = data_a[0];
                let result: Vec<f64> = data_b.iter().map(|&x| scalar + x).collect();
                Ok(JArray::Numeric {
                    shape: shape_b.clone(),
                    data: result,
                })
            }
        
        // Vector + scalar
        (JArray::Numeric { shape: shape_a, data: data_a }, 
         JArray::Numeric { shape: shape_b, data: data_b })
            if shape_b.is_empty() => {
                let scalar = data_b[0];
                let result: Vec<f64> = data_a.iter().map(|&x| x + scalar).collect();
                Ok(JArray::Numeric {
                    shape: shape_a.clone(),
                    data: result,
                })
            }
        
        // Vector + vector of same length
        (JArray::Numeric { shape: shape_a, data: data_a }, 
         JArray::Numeric { shape: shape_b, data: data_b })
            if shape_a.len() == shape_b.len() && shape_a == shape_b => {
                let result: Vec<f64> = data_a.iter().zip(data_b.iter())
                    .map(|(&a, &b)| a + b)
                    .collect();
                Ok(JArray::Numeric {
                    shape: shape_a.clone(),
                    data: result,
                })
            }
        
        // Incompatible types
        _ => Err(JError::InvalidArgument(
            "Incompatible array shapes for addition".to_string(),
        )),
    }
}

// From function - index selection ({ dyad)
pub fn from(left: &JArray, right: &JArray) -> Result<JArray, JError> {
    // Extract the index from the left argument
    let index = match left.is_scalar() {
        true => left.as_scalar()? as usize,
        false => {
            return Err(JError::InvalidArgument(
                "From requires a scalar left argument".to_string(),
            ))
        }
    };
    
    // Get the element at the specified index
    match right {
        JArray::Numeric { shape, data } => {
            if shape.is_empty() {
                // Can't index into a scalar
                return Err(JError::InvalidArgument(
                    "Cannot index into a scalar".to_string(),
                ));
            }
            
            if shape.len() == 1 {
                // Vector case
                if index >= shape[0] {
                    return Err(JError::InvalidArgument(format!(
                        "Index out of bounds: {} >= {}", index, shape[0]
                    )));
                }
                
                // Return the element at the index
                return Ok(JArray::scalar(data[index]));
            }
            
            // For higher dimensions, extract a slice along the first dimension
            if index >= shape[0] {
                return Err(JError::InvalidArgument(format!(
                    "Index out of bounds: {} >= {}", index, shape[0]
                )));
            }
            
            // Calculate the size of each slice
            let slice_size: usize = shape.iter().skip(1).product();
            let start = index * slice_size;
            let end = start + slice_size;
            
            // Extract the slice
            let new_data = data[start..end].to_vec();
            let new_shape = shape.iter().skip(1).cloned().collect();
            
            Ok(JArray::Numeric {
                shape: new_shape,
                data: new_data,
            })
        }
        
        JArray::Boxed { shape, data } => {
            if shape.is_empty() {
                // Can't index into a scalar box
                return Err(JError::InvalidArgument(
                    "Cannot index into a scalar box".to_string(),
                ));
            }
            
            if index >= shape[0] {
                return Err(JError::InvalidArgument(format!(
                    "Index out of bounds: {} >= {}", index, shape[0]
                )));
            }
            
            // Return the boxed element
            Ok(*data[index].clone())
        }
    }
}

// Find function - search for elements (~ dyad)
pub fn find(left: &JArray, right: &JArray) -> Result<JArray, JError> {
    // Implement a simple find for vector inputs
    match (left, right) {
        (JArray::Numeric { shape: shape_a, data: data_a },
         JArray::Numeric { shape: shape_b, data: data_b })
            if shape_a.len() <= 1 && shape_b.len() <= 1 => {
                
                // Find the first occurrence of each element in left within right
                let result: Vec<f64> = data_a.iter()
                    .map(|&a| {
                        match data_b.iter().position(|&b| (b - a).abs() < 1e-10) {
                            Some(pos) => pos as f64,
                            None => -1.0, // Not found
                        }
                    })
                    .collect();
                
                Ok(JArray::vector(result))
            }
        
        _ => Err(JError::InvalidArgument(
            "Find requires vector arguments".to_string(),
        )),
    }
}

// Reshape function - change dimensions while preserving data (# dyad)
pub fn reshape(left: &JArray, right: &JArray) -> Result<JArray, JError> {
    // Extract the new shape from the left argument
    let new_shape: Vec<usize> = match left {
        JArray::Numeric { shape, data } => {
            if shape.is_empty() {
                // Scalar case
                vec![data[0] as usize]
            } else {
                // Vector case
                data.iter().map(|&x| x as usize).collect()
            }
        }
        _ => {
            return Err(JError::InvalidArgument(
                "Reshape requires numeric left argument".to_string(),
            ))
        }
    };
    
    // Get the data from the right argument
    match right {
        JArray::Numeric { data, .. } => {
            // Calculate total size needed
            let total_size: usize = new_shape.iter().product();
            
            // Create new data array with cycling if needed
            let mut new_data = Vec::with_capacity(total_size);
            for i in 0..total_size {
                new_data.push(data[i % data.len()]);
            }
            
            JArray::shaped(new_shape, new_data)
        }
        _ => Err(JError::InvalidArgument(
            "Reshape requires numeric right argument".to_string(),
        )),
    }
}

// Concatenate function - join arrays together (, dyad)
pub fn concatenate(left: &JArray, right: &JArray) -> Result<JArray, JError> {
    match (left, right) {
        (JArray::Numeric { shape: shape_a, data: data_a },
         JArray::Numeric { shape: shape_b, data: data_b }) => {
            
            // Handle scalars by treating them as single-element vectors
            let effective_shape_a = if shape_a.is_empty() { vec![1] } else { shape_a.clone() };
            let effective_shape_b = if shape_b.is_empty() { vec![1] } else { shape_b.clone() };
            
            // For now, we'll implement simple vector concatenation
            if effective_shape_a.len() == 1 && effective_shape_b.len() == 1 {
                // Concatenate vectors
                let mut new_data = data_a.clone();
                new_data.extend_from_slice(data_b);
                
                let new_shape = vec![new_data.len()];
                return Ok(JArray::Numeric {
                    shape: new_shape,
                    data: new_data,
                });
            }
            
            // For higher dimensions, shapes need to match except along the first dimension
            if effective_shape_a.len() != effective_shape_b.len() {
                return Err(JError::InvalidArgument(
                    "Cannot concatenate arrays of different ranks".to_string(),
                ));
            }
            
            // Check that all dimensions except the first match
            for i in 1..effective_shape_a.len() {
                if effective_shape_a[i] != effective_shape_b[i] {
                    return Err(JError::InvalidArgument(
                        "Cannot concatenate arrays with mismatched shapes".to_string(),
                    ));
                }
            }
            
            // Concatenate along the first dimension
            let mut new_data = data_a.clone();
            new_data.extend_from_slice(data_b);
            
            let mut new_shape = effective_shape_a.clone();
            new_shape[0] += effective_shape_b[0];
            
            Ok(JArray::Numeric {
                shape: new_shape,
                data: new_data,
            })
        }
        
        // Boxed arrays would be handled similarly
        _ => Err(JError::InvalidArgument(
            "Concatenation of boxed arrays not implemented".to_string(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plus() {
        let a = JArray::vector(vec![1.0, 2.0, 3.0]);
        let b = JArray::vector(vec![4.0, 5.0, 6.0]);
        let result = plus(&a, &b).unwrap();
        
        match result {
            JArray::Numeric { data, .. } => {
                assert_eq!(data, vec![5.0, 7.0, 9.0]);
            }
            _ => panic!("Expected numeric array"),
        }
    }

    #[test]
    fn test_iota() {
        let n = JArray::scalar(5.0);
        let result = iota(&n).unwrap();
        
        match result {
            JArray::Numeric { data, .. } => {
                assert_eq!(data, vec![0.0, 1.0, 2.0, 3.0, 4.0]);
            }
            _ => panic!("Expected numeric array"),
        }
    }

    #[test]
    fn test_reshape() {
        let shape = JArray::vector(vec![2.0, 3.0]);
        let data = JArray::vector(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let result = reshape(&shape, &data).unwrap();
        
        assert_eq!(result.shape(), &[2, 3]);
    }
}