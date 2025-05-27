// J Evaluator Module - Enhanced for All Operators
// Expression evaluation and J verb implementation with full operator support

use crate::j_array::{JArray, JValue, ArrayShape, ArrayError};
use crate::parser::JNode;
use std::fmt;

// Evaluation errors
#[derive(Debug, Clone)]
pub enum EvaluationError {
    UnsupportedVerb(char, String),
    DimensionMismatch(String),
    DomainError(String),
    RankError(String),
    ArrayError(ArrayError),
}

impl From<ArrayError> for EvaluationError {
    fn from(err: ArrayError) -> Self {
        EvaluationError::ArrayError(err)
    }
}

impl fmt::Display for EvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvaluationError::UnsupportedVerb(verb, msg) => {
                write!(f, "Unsupported verb '{}': {}", verb, msg)
            }
            EvaluationError::DimensionMismatch(msg) => {
                write!(f, "Dimension mismatch: {}", msg)
            }
            EvaluationError::DomainError(msg) => {
                write!(f, "Domain error: {}", msg)
            }
            EvaluationError::RankError(msg) => {
                write!(f, "Rank error: {}", msg)
            }
            EvaluationError::ArrayError(err) => {
                write!(f, "Array error: {}", err)
            }
        }
    }
}

impl std::error::Error for EvaluationError {}

// J Evaluator
pub struct JEvaluator;

impl JEvaluator {
    pub fn new() -> Self {
        JEvaluator
    }

    // Evaluate an AST node
    pub fn evaluate(&self, ast: &JNode) -> Result<JArray, EvaluationError> {
        match ast {
            JNode::Literal(array) => Ok(array.clone()),
            
            JNode::MonadicVerb(verb, arg) => {
                let arg_value = self.evaluate(arg)?;
                
                match verb {
                    '~' => self.iota(&arg_value),
                    '+' => self.plus_monadic(&arg_value),
                    '#' => self.tally(&arg_value),
                    ',' => self.ravel(&arg_value),
                    '<' => self.box_verb(&arg_value),
                    _ => Err(EvaluationError::UnsupportedVerb(
                        *verb, 
                        "Monadic form not implemented".to_string()
                    )),
                }
            }
            
            JNode::DyadicVerb(verb, left, right) => {
                let left_value = self.evaluate(left)?;
                let right_value = self.evaluate(right)?;
                
                match verb {
                    '+' => self.plus_dyadic(&left_value, &right_value),
                    '#' => self.reshape(&left_value, &right_value),
                    '{' => self.from_verb(&left_value, &right_value),
                    ',' => self.concatenate(&left_value, &right_value),
                    '<' => self.less_than(&left_value, &right_value),
                    _ => Err(EvaluationError::UnsupportedVerb(
                        *verb, 
                        "Dyadic form not implemented".to_string()
                    )),
                }
            }
            
            JNode::AmbiguousVerb(_, _, _) => {
                Err(EvaluationError::DomainError(
                    "Internal error: AmbiguousVerb node should have been resolved before evaluation".to_string()
                ))
            }
        }
    }

    // MONADIC VERBS

    // Iota verb (~): Generate a sequence of integers from 0 to n-1
    fn iota(&self, array: &JArray) -> Result<JArray, EvaluationError> {
        if !array.is_scalar() {
            return Err(EvaluationError::DomainError(
                "iota requires a scalar argument".to_string()
            ));
        }
        
        let n = array.data[0].to_integer()
            .ok_or(EvaluationError::DomainError("iota requires integer argument".to_string()))?;
        
        if n < 0 {
            return Err(EvaluationError::DomainError(
                "iota requires a non-negative argument".to_string()
            ));
        }
        
        let data: Vec<i32> = (0..n).collect();
        Ok(JArray::vector(data))
    }
    
    // Plus verb (+): Identity function - returns the argument unchanged
    fn plus_monadic(&self, array: &JArray) -> Result<JArray, EvaluationError> {
        // Identity function just returns a clone of the input
        Ok(array.clone())
    }

    // Tally verb (#): Count number of elements along first axis
    fn tally(&self, array: &JArray) -> Result<JArray, EvaluationError> {
        let count = array.tally() as i32;
        Ok(JArray::scalar(count))
    }

    // Ravel verb (,): Flatten array to vector
    fn ravel(&self, array: &JArray) -> Result<JArray, EvaluationError> {
        Ok(array.ravel())
    }

    // Box verb (<): Create boxed array
    fn box_verb(&self, array: &JArray) -> Result<JArray, EvaluationError> {
        Ok(JArray::box_array(array.clone()))
    }

    // DYADIC VERBS

    // Plus verb (+): Element-wise addition
    fn plus_dyadic(&self, left: &JArray, right: &JArray) -> Result<JArray, EvaluationError> {
        match (left.is_scalar(), right.is_scalar()) {
            // Scalar + Scalar
            (true, true) => {
                let left_val = left.data[0].to_integer()
                    .ok_or(EvaluationError::DomainError("Addition requires numeric values".to_string()))?;
                let right_val = right.data[0].to_integer()
                    .ok_or(EvaluationError::DomainError("Addition requires numeric values".to_string()))?;
                Ok(JArray::scalar(left_val + right_val))
            }
            
            // Scalar + Vector
            (true, false) => {
                let scalar = left.data[0].to_integer()
                    .ok_or(EvaluationError::DomainError("Addition requires numeric values".to_string()))?;
                let result_data: Result<Vec<i32>, _> = right.data.iter()
                    .map(|v| v.to_integer()
                        .map(|i| scalar + i)
                        .ok_or(EvaluationError::DomainError("Addition requires numeric values".to_string())))
                    .collect();
                Ok(JArray::vector(result_data?))
            }
            
            // Vector + Scalar
            (false, true) => {
                let scalar = right.data[0].to_integer()
                    .ok_or(EvaluationError::DomainError("Addition requires numeric values".to_string()))?;
                let result_data: Result<Vec<i32>, _> = left.data.iter()
                    .map(|v| v.to_integer()
                        .map(|i| i + scalar)
                        .ok_or(EvaluationError::DomainError("Addition requires numeric values".to_string())))
                    .collect();
                Ok(JArray::vector(result_data?))
            }
            
            // Vector + Vector
            (false, false) => {
                if left.shape.dimensions != right.shape.dimensions {
                    return Err(EvaluationError::DimensionMismatch(
                        "Arrays must have matching shapes for addition".to_string()
                    ));
                }
                
                let result_data: Result<Vec<i32>, EvaluationError> = left.data.iter()
                    .zip(right.data.iter())
                    .map(|(l, r)| {
                        let left_val = l.to_integer()
                            .ok_or(EvaluationError::DomainError("Addition requires numeric values".to_string()))?;
                        let right_val = r.to_integer()
                            .ok_or(EvaluationError::DomainError("Addition requires numeric values".to_string()))?;
                        Ok(left_val + right_val)
                    })
                    .collect();
                
                Ok(JArray {
                    data: result_data?.into_iter().map(JValue::Integer).collect(),
                    shape: left.shape.clone(),
                })
            }
        }
    }

    // Reshape verb (#): Reshape array to new dimensions
    fn reshape(&self, shape_array: &JArray, data_array: &JArray) -> Result<JArray, EvaluationError> {
        // Extract shape dimensions
        let new_dims: Result<Vec<usize>, _> = shape_array.data.iter()
            .map(|v| v.to_integer()
                .map(|i| i as usize)
                .ok_or(EvaluationError::DomainError("Shape must contain integers".to_string())))
            .collect();
        
        let new_shape = ArrayShape { dimensions: new_dims? };
        Ok(data_array.reshape(new_shape)?)
    }

    // From verb ({): Index selection
    fn from_verb(&self, indices: &JArray, source: &JArray) -> Result<JArray, EvaluationError> {
        Ok(source.select_from(indices)?)
    }

    // Concatenate verb (,): Join arrays
    fn concatenate(&self, left: &JArray, right: &JArray) -> Result<JArray, EvaluationError> {
        Ok(left.concatenate(right)?)
    }

    // Less than verb (<): Element-wise comparison
    fn less_than(&self, left: &JArray, right: &JArray) -> Result<JArray, EvaluationError> {
        match (left.is_scalar(), right.is_scalar()) {
            // Scalar < Scalar
            (true, true) => {
                let left_val = left.data[0].to_integer()
                    .ok_or(EvaluationError::DomainError("Comparison requires numeric values".to_string()))?;
                let right_val = right.data[0].to_integer()
                    .ok_or(EvaluationError::DomainError("Comparison requires numeric values".to_string()))?;
                Ok(JArray::scalar(if left_val < right_val { 1 } else { 0 }))
            }
            
            // Element-wise comparison for arrays
            _ => {
                // For simplicity, convert both to same shape if possible
                if left.shape.total_elements() != right.shape.total_elements() {
                    return Err(EvaluationError::DimensionMismatch(
                        "Arrays must have compatible shapes for comparison".to_string()
                    ));
                }
                
                let result_data: Result<Vec<i32>, EvaluationError> = left.data.iter()
                    .zip(right.data.iter())
                    .map(|(l, r)| {
                        let left_val = l.to_integer()
                            .ok_or(EvaluationError::DomainError("Comparison requires numeric values".to_string()))?;
                        let right_val = r.to_integer()
                            .ok_or(EvaluationError::DomainError("Comparison requires numeric values".to_string()))?;
                        Ok(if left_val < right_val { 1 } else { 0 })
                    })
                    .collect();
                
                Ok(JArray {
                    data: result_data?.into_iter().map(JValue::Integer).collect(),
                    shape: left.shape.clone(),
                })
            }
        }
    }
}