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

    // Evaluate a token as a J expression
    fn eval_token(&self, token: &str) -> Result<JArray, String> {
        // Check for our special array format
        if token.starts_with("__ARRAY__") {
            let array_data = &token[9..]; // Skip the "__ARRAY__" prefix
            return self.parse_numeric_vector(array_data);
        }
        
        // Try to parse as a scalar
        if let Ok(n) = token.parse::<i64>() {
            return Ok(JArray::new_scalar(n));
        }
        
        // Try to parse as a vector
        match self.parse_numeric_vector(token) {
            Ok(array) => return Ok(array),
            Err(_) => {}  // Continue to next checks
        }
        
        // Unsupported token
        Err(format!("Unsupported token: '{}'", token))
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
                       Compound expressions with precedence are supported (e.g., '1+~5')\n\
                       help - show this help message".to_string());
        }
        
        // Tokenize the input for parsing with operator precedence
        let mut tokens = Vec::new();
        let mut current_token = String::new();
        let mut i = 0;
        let input_chars: Vec<char> = input.chars().collect();
        
        while i < input_chars.len() {
            let c = input_chars[i];
            
            match c {
                '+' | '~' => {
                    // If we have a current token, add it to tokens
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                    
                    // Add the operator as a separate token
                    tokens.push(c.to_string());
                },
                ' ' => {
                    // Space - if we have a current token, add it to tokens
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                },
                _ => {
                    // Add to current token
                    current_token.push(c);
                }
            }
            
            i += 1;
        }
        
        // Add any remaining token
        if !current_token.is_empty() {
            tokens.push(current_token);
        }
        
        // Handle an empty expression
        if tokens.is_empty() {
            return Err("Empty expression".to_string());
        }
        
        // Process monadic verbs first (right to left)
        let mut i = tokens.len() - 1;
        while i > 0 {
            if tokens[i - 1] == "~" && i < tokens.len() {
                // Handle iota
                if let Ok(n) = tokens[i].parse::<i64>() {
                    let result = self.iota(n)?;
                    tokens.remove(i);
                    tokens.remove(i - 1);
                    
                    // For arrays, we need to store them in a special format for internal processing
                    let array_str = if result.rank == 1 {
                        // For vectors, store as special format that will be recognized by eval_token
                        let nums = result.data.iter()
                            .map(|n| n.to_string())
                            .collect::<Vec<_>>()
                            .join(" ");
                        format!("__ARRAY__{}", nums)
                    } else {
                        // For other ranks, just use standard formatting
                        format!("{}", result)
                    };
                    
                    tokens.insert(i - 1, array_str);
                    i -= 1;
                } else {
                    return Err(format!("Invalid argument for iota: '{}'", tokens[i]));
                }
            } else if tokens[i - 1] == "+" && i < tokens.len() {
                // Handle monadic plus
                let right_result = self.eval_token(&tokens[i])?;
                let result = self.plus_monadic(&right_result)?;
                tokens.remove(i);
                tokens.remove(i - 1);
                
                // Format the result the same way as for iota
                let array_str = if result.rank == 1 {
                    let nums = result.data.iter()
                        .map(|n| n.to_string())
                        .collect::<Vec<_>>()
                        .join(" ");
                    format!("__ARRAY__{}", nums)
                } else {
                    format!("{}", result)
                };
                
                tokens.insert(i - 1, array_str);
                i -= 1;
            } else {
                i -= 1;
            }
        }
        
        // Process dyadic verbs (right to left)
        i = tokens.len() - 2;
        while i > 0 {
            if tokens[i] == "+" {
                // Parse left operand
                let left_result = self.eval_token(&tokens[i - 1])?;
                
                // Parse right operand
                let right_result = self.eval_token(&tokens[i + 1])?;
                
                // Apply dyadic plus
                let result = self.plus_dyadic(&left_result, &right_result)?;
                
                // Replace the operation and operands with the result
                tokens.remove(i + 1);
                tokens.remove(i);
                tokens.remove(i - 1);
                
                // Format the result the same way as for other operations
                let array_str = if result.rank == 1 {
                    let nums = result.data.iter()
                        .map(|n| n.to_string())
                        .collect::<Vec<_>>()
                        .join(" ");
                    format!("__ARRAY__{}", nums)
                } else {
                    format!("{}", result)
                };
                
                tokens.insert(i - 1, array_str);
                
                // Adjust index
                i = tokens.len() - 2;
            } else {
                i -= 1;
            }
        }
        
        // By now, we should have a single token that is our result
        if tokens.len() == 1 {
            // Check for our special array format
            if tokens[0].starts_with("__ARRAY__") {
                let array_data = &tokens[0][9..]; // Skip the "__ARRAY__" prefix
                return self.parse_numeric_vector(array_data);
            }
        
            // Try to parse the final result
            if let Ok(n) = tokens[0].parse::<i64>() {
                return Ok(JArray::new_scalar(n));
            }
            
            // Try to parse as a vector
            match self.parse_numeric_vector(&tokens[0]) {
                Ok(array) => return Ok(array),
                Err(_) => {}  // Continue to final error
            }
        }
        
        // If we still have multiple tokens or couldn't parse the final token
        Err(format!("Failed to evaluate expression: '{}', tokens: {:?}", input, tokens))
    }
}

// Create a more user-friendly output for the web display
pub fn format_result(result: Result<JArray, String>) -> String {
    match result {
        Ok(array) => format!("{}", array),
        Err(error) => format!("Error: {}", error),
    }
}