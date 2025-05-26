// J Evaluator Module
// Expression evaluation and J verb implementation

use crate::j_array::{JArray, JType};
use crate::parser::JNode;
use std::fmt;

// Evaluation errors
#[derive(Debug, Clone)]
pub enum EvaluationError {
    UnsupportedVerb(char, String),
    DimensionMismatch(String),
    DomainError(String),
    RankError(String),
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
        }
    }
}

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
                    '~' => self.iota(arg_value.data[0]),
                    '+' => self.plus_monadic(&arg_value),
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

    // Iota verb: Generate a sequence of integers from 0 to n-1
    fn iota(&self, n: i64) -> Result<JArray, EvaluationError> {
        if n < 0 {
            return Err(EvaluationError::DomainError(
                "iota requires a non-negative argument".to_string()
            ));
        }
        
        let n_usize = n as usize;
        let data: Vec<i64> = (0..n).collect();
        
        Ok(JArray::new_integer(1, vec![n_usize], data))
    }
    
    // Plus verb (monadic): Identity function - returns the argument unchanged
    fn plus_monadic(&self, array: &JArray) -> Result<JArray, EvaluationError> {
        // Identity function just returns a clone of the input
        Ok(array.clone())
    }
    
    // Plus verb (dyadic): Element-wise addition
    fn plus_dyadic(&self, left: &JArray, right: &JArray) -> Result<JArray, EvaluationError> {
        // For now, we'll only support scalar and vector additions
        match (left.rank, right.rank) {
            // Scalar + Scalar
            (0, 0) => {
                let result = left.data[0] + right.data[0];
                Ok(JArray::new_scalar(result))
            }
            
            // Scalar + Vector
            (0, 1) => {
                let scalar = left.data[0];
                let mut result_data = Vec::with_capacity(right.data.len());
                
                for &value in &right.data {
                    result_data.push(scalar + value);
                }
                
                Ok(JArray::new_integer(1, right.shape.clone(), result_data))
            }
            
            // Vector + Scalar
            (1, 0) => {
                let scalar = right.data[0];
                let mut result_data = Vec::with_capacity(left.data.len());
                
                for &value in &left.data {
                    result_data.push(value + scalar);
                }
                
                Ok(JArray::new_integer(1, left.shape.clone(), result_data))
            }
            
            // Vector + Vector (same length)
            (1, 1) => {
                if left.shape[0] != right.shape[0] {
                    return Err(EvaluationError::DimensionMismatch(
                        "vectors must have the same length for addition".to_string()
                    ));
                }
                
                let mut result_data = Vec::with_capacity(left.data.len());
                
                for i in 0..left.data.len() {
                    result_data.push(left.data[i] + right.data[i]);
                }
                
                Ok(JArray::new_integer(1, left.shape.clone(), result_data))
            }
            
            // Unsupported rank combinations
            _ => Err(EvaluationError::RankError(
                "plus is only implemented for scalars and vectors".to_string()
            )),
        }
    }
}