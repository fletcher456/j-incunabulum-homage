use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_double};
use std::ptr;
use std::collections::VecDeque;
use std::fmt;

/// Represents a J array with values and metadata
#[derive(Debug, Clone)]
pub struct JArray {
    values: Vec<f64>,
    rank: usize,
    shape: Vec<usize>,
}

impl JArray {
    /// Create a new array with given shape and filled with zeros
    fn new(shape: Vec<usize>) -> Self {
        let size = shape.iter().product::<usize>();
        JArray {
            values: vec![0.0; size],
            rank: shape.len(),
            shape,
        }
    }

    /// Create a scalar (rank 0) array with a single value
    fn scalar(value: f64) -> Self {
        JArray {
            values: vec![value],
            rank: 0,
            shape: vec![],
        }
    }

    /// Create a vector (rank 1) array from a slice of values
    fn vector(values: Vec<f64>) -> Self {
        let length = values.len();
        JArray {
            values,
            rank: 1,
            shape: vec![length],
        }
    }

    /// Create an iota array (0..n)
    fn iota(n: usize) -> Self {
        let values: Vec<f64> = (0..n).map(|i| i as f64).collect();
        JArray {
            values,
            rank: 1,
            shape: vec![n],
        }
    }
    
    /// Add a scalar to each element of this array
    fn add_scalar(&self, scalar: f64) -> Self {
        let new_values = self.values.iter().map(|&x| x + scalar).collect();
        JArray {
            values: new_values,
            rank: self.rank,
            shape: self.shape.clone(),
        }
    }
    
    /// Multiply each element of this array by a scalar
    fn multiply_scalar(&self, scalar: f64) -> Self {
        let new_values = self.values.iter().map(|&x| x * scalar).collect();
        JArray {
            values: new_values,
            rank: self.rank,
            shape: self.shape.clone(),
        }
    }
    
    /// Subtract a scalar from each element of this array
    fn subtract_scalar(&self, scalar: f64) -> Self {
        let new_values = self.values.iter().map(|&x| x - scalar).collect();
        JArray {
            values: new_values,
            rank: self.rank,
            shape: self.shape.clone(),
        }
    }
    
    /// Divide each element of this array by a scalar
    fn divide_scalar(&self, scalar: f64) -> Result<Self, String> {
        if scalar == 0.0 {
            return Err("Division by zero".to_string());
        }
        
        let new_values = self.values.iter().map(|&x| x / scalar).collect();
        Ok(JArray {
            values: new_values,
            rank: self.rank,
            shape: self.shape.clone(),
        })
    }
    
    /// Concatenate two arrays (assuming they have the same rank)
    fn concatenate(&self, other: &JArray) -> Result<Self, String> {
        if self.rank != other.rank {
            return Err("Cannot concatenate arrays with different ranks".to_string());
        }
        
        if self.rank == 0 {
            // For scalars, create a vector
            return Ok(JArray::vector(vec![self.values[0], other.values[0]]));
        }
        
        // For now, only implement for vectors (rank 1)
        if self.rank == 1 {
            let mut new_values = self.values.clone();
            new_values.extend(other.values.clone());
            
            let new_shape = vec![new_values.len()];
            
            return Ok(JArray {
                values: new_values,
                rank: 1,
                shape: new_shape,
            });
        }
        
        Err("Concatenation only implemented for vectors".to_string())
    }
    
    /// Get string representation of this array
    fn to_string(&self) -> String {
        if self.rank == 0 {
            // Scalar
            format!("{:.8}", self.values[0])
        } else if self.rank == 1 {
            // Vector
            let elements: Vec<String> = self.values.iter()
                .map(|&x| format!("{:.8}", x))
                .collect();
            
            format!("[{}]", elements.join(" "))
        } else {
            // Higher-rank arrays - simplified for now
            format!("Array with shape {:?}", self.shape)
        }
    }
}

/// Represents a token in J syntax
#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    Operator(char),
    Identifier(String),
    Space,
}

/// Simple J parser and interpreter
struct JInterpreter {
    tokens: VecDeque<Token>,
}

impl JInterpreter {
    /// Create a new interpreter for the given input
    fn new(input: &str) -> Self {
        JInterpreter {
            tokens: JInterpreter::tokenize(input),
        }
    }
    
    /// Tokenize a string into J tokens
    fn tokenize(input: &str) -> VecDeque<Token> {
        let mut tokens = VecDeque::new();
        let mut chars = input.chars().peekable();
        
        while let Some(&c) = chars.peek() {
            match c {
                '0'..='9' | '.' => {
                    // Parse a number
                    let mut number = String::new();
                    
                    while let Some(&c) = chars.peek() {
                        if c.is_digit(10) || c == '.' || c == 'e' || c == 'E' || c == '-' && !number.is_empty() {
                            number.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    
                    if let Ok(value) = number.parse::<f64>() {
                        tokens.push_back(Token::Number(value));
                    }
                },
                'a'..='z' | 'A'..='Z' => {
                    // Parse an identifier
                    let mut ident = String::new();
                    
                    while let Some(&c) = chars.peek() {
                        if c.is_alphabetic() || c == '.' {
                            ident.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    
                    tokens.push_back(Token::Identifier(ident));
                },
                '+' | '-' | '*' | '/' | '%' | '^' => {
                    // Parse an operator
                    tokens.push_back(Token::Operator(c));
                    chars.next();
                },
                ' ' | '\t' => {
                    // Parse whitespace
                    tokens.push_back(Token::Space);
                    chars.next();
                    
                    // Skip additional whitespace
                    while let Some(&c) = chars.peek() {
                        if c == ' ' || c == '\t' {
                            chars.next();
                        } else {
                            break;
                        }
                    }
                },
                _ => {
                    // Skip other characters
                    chars.next();
                }
            }
        }
        
        tokens
    }
    
    /// Interpret the J expression and return the result
    fn interpret(&mut self) -> Result<JArray, String> {
        // Check for iota first (special case)
        if self.tokens.len() >= 2 {
            if let Some(Token::Identifier(id)) = self.tokens.get(0) {
                if id == "i" {
                    if let Some(Token::Operator(op)) = self.tokens.get(1) {
                        if *op == '.' {
                            // Handle i.n (iota)
                            self.tokens.pop_front(); // Remove i
                            self.tokens.pop_front(); // Remove .
                            
                            if let Some(Token::Number(n)) = self.tokens.pop_front() {
                                if n >= 0.0 && n < 1000.0 {
                                    return Ok(JArray::iota(n as usize));
                                } else {
                                    return Err("Invalid iota size".to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Parse array with possible operation
        self.parse_expression()
    }
    
    /// Parse a basic expression (number or array with possible operation)
    fn parse_expression(&mut self) -> Result<JArray, String> {
        if self.tokens.is_empty() {
            return Err("Empty expression".to_string());
        }
        
        // Check for array expression (space-separated numbers)
        let has_spaces = self.tokens.iter().any(|t| *t == Token::Space);
        
        if has_spaces {
            // Parse as array
            let mut values = Vec::new();
            
            while !self.tokens.is_empty() {
                match self.tokens.front() {
                    Some(Token::Number(n)) => {
                        values.push(*n);
                        self.tokens.pop_front();
                    },
                    Some(Token::Space) => {
                        self.tokens.pop_front();
                    },
                    Some(Token::Operator(_)) => {
                        break;
                    },
                    _ => {
                        return Err("Invalid array syntax".to_string());
                    }
                }
            }
            
            let array = JArray::vector(values);
            
            // Check for an operation on the array
            if !self.tokens.is_empty() {
                if let Some(Token::Operator(op)) = self.tokens.pop_front() {
                    if let Some(Token::Number(n)) = self.tokens.pop_front() {
                        // Apply the operation
                        return match op {
                            '+' => Ok(array.add_scalar(n)),
                            '-' => Ok(array.subtract_scalar(n)),
                            '*' => Ok(array.multiply_scalar(n)),
                            '/' => array.divide_scalar(n),
                            _ => Err(format!("Unsupported array operation: {}", op)),
                        };
                    }
                }
            }
            
            return Ok(array);
        }
        
        // Simple expression (e.g., 2+3)
        if let Some(Token::Number(left)) = self.tokens.pop_front() {
            if !self.tokens.is_empty() {
                if let Some(Token::Operator(op)) = self.tokens.pop_front() {
                    if let Some(Token::Number(right)) = self.tokens.pop_front() {
                        // Compute the result
                        let result = match op {
                            '+' => left + right,
                            '-' => left - right,
                            '*' => left * right,
                            '/' => {
                                if right == 0.0 {
                                    return Err("Division by zero".to_string());
                                }
                                left / right
                            },
                            '^' => left.powf(right),
                            _ => return Err(format!("Unsupported operation: {}", op)),
                        };
                        
                        return Ok(JArray::scalar(result));
                    }
                }
            }
            
            // Just a single number
            return Ok(JArray::scalar(left));
        }
        
        Err("Invalid expression".to_string())
    }
}

/// C-compatible function to interpret J code
///
/// # Safety
///
/// This function takes a raw C string pointer and returns a new C string pointer.
/// The caller is responsible for freeing the returned string with free_string().
#[no_mangle]
pub unsafe extern "C" fn interpret_j_code(input: *const c_char) -> *mut c_char {
    // Default error message in case of early failure
    let default_error = CString::new("Error processing J code").unwrap();
    
    // Check for null pointer
    if input.is_null() {
        return CString::into_raw(default_error);
    }
    
    // Convert C string to Rust string
    let c_str = match CStr::from_ptr(input).to_str() {
        Ok(s) => s,
        Err(_) => return CString::into_raw(default_error),
    };
    
    // Create interpreter and interpret the code
    let mut interpreter = JInterpreter::new(c_str);
    let result = match interpreter.interpret() {
        Ok(array) => array.to_string(),
        Err(msg) => format!("Error: {}", msg),
    };
    
    // Convert result to C string
    match CString::new(result) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => CString::into_raw(default_error),
    }
}

/// Free a string created by interpret_j_code
///
/// # Safety
///
/// This function must be called with a pointer returned by interpret_j_code.
#[no_mangle]
pub unsafe extern "C" fn free_string(s: *mut c_char) {
    if !s.is_null() {
        let _ = CString::from_raw(s);
    }
}