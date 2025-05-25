// J Interpreter Module
// Basic implementation of a J interpreter with support for the iota verb

use std::fmt;

// J array types
#[derive(Debug, Clone)]
pub enum JType {
    Integer,
    Box,
}

// J array structure (similar to the C version's 'A' struct)
#[derive(Debug, Clone)]
pub struct JArray {
    pub array_type: JType,
    pub rank: usize,
    pub shape: Vec<usize>,
    pub data: Vec<i64>,
}

impl JArray {
    // Create a new integer array
    pub fn new_integer(rank: usize, shape: Vec<usize>, data: Vec<i64>) -> Self {
        JArray {
            array_type: JType::Integer,
            rank,
            shape,
            data,
        }
    }

    // Create a scalar integer
    pub fn new_scalar(value: i64) -> Self {
        JArray {
            array_type: JType::Integer,
            rank: 0,
            shape: vec![],
            data: vec![value],
        }
    }

    // Calculate the total number of elements in the array
    pub fn tally(&self) -> usize {
        if self.rank == 0 {
            1
        } else {
            self.shape.iter().product()
        }
    }
}

// Display implementation for JArray to format output
impl fmt::Display for JArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.array_type {
            JType::Integer => {
                if self.rank == 0 {
                    // Scalar
                    write!(f, "{}", self.data[0])
                } else if self.rank == 1 {
                    // Vector
                    write!(
                        f,
                        "{}",
                        self.data
                            .iter()
                            .map(|i| i.to_string())
                            .collect::<Vec<_>>()
                            .join(" ")
                    )
                } else {
                    // Matrix or higher dimensions - simplified output
                    write!(f, "Array(rank={}, shape={:?}): {:?}", self.rank, self.shape, self.data)
                }
            }
            JType::Box => {
                write!(f, "<box>")
            }
        }
    }
}

// Interpreter struct to manage the J session
pub struct JInterpreter {
    // We could add symbol tables and other state here
}

impl JInterpreter {
    // Create a new interpreter
    pub fn new() -> Self {
        JInterpreter {}
    }

    // Iota verb: Generate a sequence of integers from 0 to n-1
    pub fn iota(&self, n: i64) -> Result<JArray, String> {
        if n < 0 {
            return Err("Domain error: iota requires a non-negative argument".to_string());
        }
        
        let n_usize = n as usize;
        let data: Vec<i64> = (0..n).collect();
        
        Ok(JArray::new_integer(1, vec![n_usize], data))
    }
    
    // Plus verb (monadic): Identity function - returns the argument unchanged
    pub fn plus_monadic(&self, array: &JArray) -> Result<JArray, String> {
        // Identity function just returns a clone of the input
        Ok(array.clone())
    }
    
    // Plus verb (dyadic): Element-wise addition
    pub fn plus_dyadic(&self, left: &JArray, right: &JArray) -> Result<JArray, String> {
        // For now, we'll only support scalar and vector additions
        match (left.rank, right.rank) {
            // Scalar + Scalar
            (0, 0) => {
                let result = left.data[0] + right.data[0];
                Ok(JArray::new_scalar(result))
            },
            
            // Scalar + Vector
            (0, 1) => {
                let scalar = left.data[0];
                let mut result_data = Vec::with_capacity(right.data.len());
                
                for &value in &right.data {
                    result_data.push(scalar + value);
                }
                
                Ok(JArray::new_integer(1, right.shape.clone(), result_data))
            },
            
            // Vector + Scalar
            (1, 0) => {
                let scalar = right.data[0];
                let mut result_data = Vec::with_capacity(left.data.len());
                
                for &value in &left.data {
                    result_data.push(value + scalar);
                }
                
                Ok(JArray::new_integer(1, left.shape.clone(), result_data))
            },
            
            // Vector + Vector (same length)
            (1, 1) => {
                if left.shape[0] != right.shape[0] {
                    return Err("Length error: vectors must have the same length for addition".to_string());
                }
                
                let mut result_data = Vec::with_capacity(left.data.len());
                
                for i in 0..left.data.len() {
                    result_data.push(left.data[i] + right.data[i]);
                }
                
                Ok(JArray::new_integer(1, left.shape.clone(), result_data))
            },
            
            // Unsupported rank combinations
            _ => Err("Rank error: plus is only implemented for scalars and vectors".to_string()),
        }
    }

    // Parse a simple numeric vector like "1 2 3 4"
    fn parse_numeric_vector(&self, input: &str) -> Result<JArray, String> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        let mut values = Vec::with_capacity(parts.len());
        
        for part in parts {
            match part.parse::<i64>() {
                Ok(n) => values.push(n),
                Err(_) => return Err(format!("Invalid number in vector: '{}'", part)),
            }
        }
        
        if values.is_empty() {
            return Err("Empty vector".to_string());
        }
        
        Ok(JArray::new_integer(1, vec![values.len()], values))
    }

    // Parse and execute a J expression
    pub fn execute(&self, input: &str) -> Result<JArray, String> {
        let input = input.trim();
        
        // Help command
        if input == "help" {
            return Err("Available commands:\n\
                       ~n - iota: generate integers from 0 to n-1\n\
                       +y - plus (monadic): identity function\n\
                       x+y - plus (dyadic): element-wise addition\n\
                       help - show this help message".to_string());
        }
        
        // Handle monadic plus (identity)
        if input.starts_with('+') {
            let arg = input[1..].trim();
            
            // Try to parse the argument as a scalar
            if let Ok(n) = arg.parse::<i64>() {
                return self.plus_monadic(&JArray::new_scalar(n));
            }
            
            // Try to parse the argument as a vector
            match self.parse_numeric_vector(arg) {
                Ok(array) => return self.plus_monadic(&array),
                Err(_) => return Err(format!("Invalid argument for monadic plus: '{}'", arg)),
            }
        }
        
        // Handle iota verb
        if input.starts_with('~') {
            let arg = input[1..].trim();
            match arg.parse::<i64>() {
                Ok(n) => return self.iota(n),
                Err(_) => return Err(format!("Invalid argument for iota: '{}'", arg)),
            }
        }
        
        // Try to parse dyadic plus: "x + y"
        if input.contains('+') {
            let parts: Vec<&str> = input.split('+').collect();
            if parts.len() == 2 {
                let left_str = parts[0].trim();
                let right_str = parts[1].trim();
                
                // Parse left operand
                let left = if let Ok(n) = left_str.parse::<i64>() {
                    JArray::new_scalar(n)
                } else {
                    match self.parse_numeric_vector(left_str) {
                        Ok(array) => array,
                        Err(_) => return Err(format!("Invalid left operand for dyadic plus: '{}'", left_str)),
                    }
                };
                
                // Parse right operand
                let right = if let Ok(n) = right_str.parse::<i64>() {
                    JArray::new_scalar(n)
                } else {
                    match self.parse_numeric_vector(right_str) {
                        Ok(array) => array,
                        Err(_) => return Err(format!("Invalid right operand for dyadic plus: '{}'", right_str)),
                    }
                };
                
                return self.plus_dyadic(&left, &right);
            }
        }
        
        // Simple numeric scalar
        if let Ok(n) = input.parse::<i64>() {
            return Ok(JArray::new_scalar(n));
        }
        
        // Simple numeric vector
        match self.parse_numeric_vector(input) {
            Ok(array) => return Ok(array),
            Err(_) => {}  // Continue to next checks
        }
        
        // Unsupported expression
        Err(format!("Unsupported J expression: '{}'", input))
    }
}

// Create a more user-friendly output for the web display
pub fn format_result(result: Result<JArray, String>) -> String {
    match result {
        Ok(array) => format!("{}", array),
        Err(error) => format!("Error: {}", error),
    }
}