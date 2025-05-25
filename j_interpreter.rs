// J Interpreter Implementation based on the feature list
// Supports monadic and dyadic verbs as specified

use std::fmt;
use std::collections::HashMap;
use std::str::FromStr;

// J Array Structure
#[derive(Debug, Clone)]
pub struct JArray {
    // Rank (number of dimensions)
    rank: usize,
    // Shape (size of each dimension)
    shape: Vec<usize>,
    // Data (flattened array)
    data: Vec<f64>,
}

impl JArray {
    // Create a scalar (rank 0 array)
    pub fn scalar(value: f64) -> Self {
        JArray {
            rank: 0,
            shape: vec![],
            data: vec![value],
        }
    }

    // Create a vector (rank 1 array)
    pub fn vector(values: Vec<f64>) -> Self {
        let len = values.len();
        JArray {
            rank: 1,
            shape: vec![len],
            data: values,
        }
    }

    // Create a shaped array
    pub fn shaped(shape: Vec<usize>, values: Vec<f64>) -> Result<Self, JError> {
        let expected_size: usize = shape.iter().product();
        if values.len() != expected_size {
            return Err(JError::InvalidArgument(format!(
                "Data size mismatch: expected {}, got {}",
                expected_size, values.len()
            )));
        }
        
        Ok(JArray {
            rank: shape.len(),
            shape,
            data: values,
        })
    }

    // Get the total size of the array
    pub fn size(&self) -> usize {
        self.data.len()
    }

    // Check if it's a scalar
    pub fn is_scalar(&self) -> bool {
        self.rank == 0 && self.data.len() == 1
    }

    // Get scalar value if it's a scalar
    pub fn as_scalar(&self) -> Option<f64> {
        if self.is_scalar() {
            Some(self.data[0])
        } else {
            None
        }
    }
}

// Error types for the J interpreter
#[derive(Debug)]
pub enum JError {
    ParseError(String),
    ExecutionError(String),
    InvalidArgument(String),
}

impl fmt::Display for JError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            JError::ExecutionError(msg) => write!(f, "Execution error: {}", msg),
            JError::InvalidArgument(msg) => write!(f, "Invalid argument: {}", msg),
        }
    }
}

// Display implementation for JArray
impl fmt::Display for JArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_scalar() {
            // Scalar value
            write!(f, "{}", self.data[0])
        } else if self.rank == 1 {
            // Vector
            write!(f, "[")?;
            for (i, val) in self.data.iter().enumerate() {
                if i > 0 {
                    write!(f, " ")?;
                }
                write!(f, "{}", val)?;
            }
            write!(f, "]")
        } else if self.rank == 2 {
            // Matrix
            let rows = self.shape[0];
            let cols = self.shape[1];
            
            writeln!(f, "[")?;
            for r in 0..rows {
                write!(f, " [")?;
                for c in 0..cols {
                    let idx = r * cols + c;
                    if c > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", self.data[idx])?;
                }
                writeln!(f, "]")?;
            }
            write!(f, "]")
        } else {
            // Higher rank arrays
            write!(f, "Array with shape {:?}: {:?}", self.shape, self.data)
        }
    }
}

// J Interpreter
pub struct JInterpreter {
    // Symbol table for variables
    symbols: HashMap<String, JArray>,
}

impl JInterpreter {
    // Create a new interpreter
    pub fn new() -> Self {
        JInterpreter {
            symbols: HashMap::new(),
        }
    }

    // Evaluate a J expression
    pub fn evaluate(&mut self, expression: &str) -> Result<JArray, JError> {
        // Handle help command
        if expression.trim().to_lowercase() == "help" {
            return Ok(JArray::scalar(0.0)); // Special case handled by caller
        }

        // Parse and evaluate the expression
        let tokens = self.tokenize(expression)?;
        self.parse_and_execute(&tokens)
    }

    // Tokenize a J expression
    fn tokenize(&self, expression: &str) -> Result<Vec<String>, JError> {
        let mut tokens = Vec::new();
        let mut current_token = String::new();
        
        let expression = expression.trim();
        
        for c in expression.chars() {
            if c.is_whitespace() {
                if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
            } else if "+-~<#{},".contains(c) {
                // Operator token
                if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
                tokens.push(c.to_string());
            } else {
                current_token.push(c);
            }
        }
        
        if !current_token.is_empty() {
            tokens.push(current_token);
        }
        
        Ok(tokens)
    }

    // Parse and execute a J expression
    fn parse_and_execute(&mut self, tokens: &[String]) -> Result<JArray, JError> {
        if tokens.is_empty() {
            return Err(JError::ParseError("Empty expression".to_string()));
        }
        
        // Check for assignment
        if tokens.len() >= 3 && tokens[1] == "=" {
            let var_name = &tokens[0];
            let value = self.parse_and_execute(&tokens[2..])?;
            self.symbols.insert(var_name.clone(), value.clone());
            return Ok(value);
        }
        
        // Parse as monadic or dyadic expression
        if tokens.len() == 2 && "+-~<#".contains(&tokens[0]) {
            // Monadic verb
            let verb = &tokens[0];
            let right = self.parse_array(&tokens[1])?;
            self.execute_monadic(verb, right)
        } else if tokens.len() == 3 && "+-~#{},".contains(&tokens[1]) {
            // Dyadic verb
            let left = self.parse_array(&tokens[0])?;
            let verb = &tokens[1];
            let right = self.parse_array(&tokens[2])?;
            self.execute_dyadic(verb, left, right)
        } else if tokens.len() == 1 {
            // Just a value
            self.parse_array(&tokens[0])
        } else {
            Err(JError::ParseError("Invalid expression format".to_string()))
        }
    }

    // Parse a string as an array
    fn parse_array(&self, token: &str) -> Result<JArray, JError> {
        // Check if it's a variable
        if let Some(value) = self.symbols.get(token) {
            return Ok(value.clone());
        }
        
        // Try to parse as a number
        if let Ok(value) = f64::from_str(token) {
            return Ok(JArray::scalar(value));
        }
        
        // Try to parse as a vector (space-separated numbers)
        let parts: Vec<&str> = token.split_whitespace().collect();
        if !parts.is_empty() {
            let mut values = Vec::with_capacity(parts.len());
            for part in parts {
                match f64::from_str(part) {
                    Ok(value) => values.push(value),
                    Err(_) => return Err(JError::ParseError(format!("Invalid number: {}", part))),
                }
            }
            return Ok(JArray::vector(values));
        }
        
        Err(JError::ParseError(format!("Cannot parse token: {}", token)))
    }

    // Execute monadic verbs (single argument)
    fn execute_monadic(&self, verb: &str, right: JArray) -> Result<JArray, JError> {
        match verb {
            "+" => self.identity(right),
            "{" => self.size_of(right),
            "~" => self.iota(right),
            "<" => self.box_array(right),
            "#" => self.shape(right),
            _ => Err(JError::ExecutionError(format!("Unknown monadic verb: {}", verb))),
        }
    }

    // Execute dyadic verbs (two arguments)
    fn execute_dyadic(&self, verb: &str, left: JArray, right: JArray) -> Result<JArray, JError> {
        match verb {
            "+" => self.plus(left, right),
            "{" => self.from(left, right),
            "~" => self.find(left, right),
            "#" => self.reshape(left, right),
            "," => self.concatenate(left, right),
            _ => Err(JError::ExecutionError(format!("Unknown dyadic verb: {}", verb))),
        }
    }

    // MONADIC VERBS

    // Identity function (+ monad)
    fn identity(&self, right: JArray) -> Result<JArray, JError> {
        Ok(right)
    }

    // Size function ({ monad)
    fn size_of(&self, right: JArray) -> Result<JArray, JError> {
        if right.rank == 0 {
            return Ok(JArray::scalar(1.0));
        } else {
            return Ok(JArray::scalar(right.shape[0] as f64));
        }
    }

    // Iota function (~ monad)
    fn iota(&self, right: JArray) -> Result<JArray, JError> {
        if let Some(n) = right.as_scalar() {
            if n < 0.0 || n.fract() != 0.0 {
                return Err(JError::InvalidArgument("Iota requires a non-negative integer".to_string()));
            }
            
            let n = n as usize;
            let data: Vec<f64> = (0..n).map(|i| i as f64).collect();
            return Ok(JArray::vector(data));
        }
        
        Err(JError::InvalidArgument("Iota requires a scalar argument".to_string()))
    }

    // Box function (< monad)
    fn box_array(&self, right: JArray) -> Result<JArray, JError> {
        // In our simplified implementation, just return the array
        Ok(right)
    }

    // Shape function (# monad)
    fn shape(&self, right: JArray) -> Result<JArray, JError> {
        let shape_data: Vec<f64> = right.shape.iter().map(|&s| s as f64).collect();
        Ok(JArray::vector(shape_data))
    }

    // DYADIC VERBS

    // Plus function (+ dyad)
    fn plus(&self, left: JArray, right: JArray) -> Result<JArray, JError> {
        // Handle scalar + scalar
        if left.is_scalar() && right.is_scalar() {
            return Ok(JArray::scalar(left.data[0] + right.data[0]));
        }
        
        // Handle scalar + array
        if left.is_scalar() {
            let scalar = left.data[0];
            let result: Vec<f64> = right.data.iter().map(|&x| scalar + x).collect();
            return Ok(JArray { rank: right.rank, shape: right.shape.clone(), data: result });
        }
        
        // Handle array + scalar
        if right.is_scalar() {
            let scalar = right.data[0];
            let result: Vec<f64> = left.data.iter().map(|&x| x + scalar).collect();
            return Ok(JArray { rank: left.rank, shape: left.shape.clone(), data: result });
        }
        
        // Handle array + array (same shape)
        if left.shape == right.shape {
            let result: Vec<f64> = left.data.iter().zip(right.data.iter())
                .map(|(&a, &b)| a + b)
                .collect();
            return Ok(JArray { rank: left.rank, shape: left.shape.clone(), data: result });
        }
        
        Err(JError::InvalidArgument("Incompatible array shapes for addition".to_string()))
    }

    // From function ({ dyad)
    fn from(&self, left: JArray, right: JArray) -> Result<JArray, JError> {
        if let Some(index) = left.as_scalar() {
            if index < 0.0 || index.fract() != 0.0 {
                return Err(JError::InvalidArgument("Index must be a non-negative integer".to_string()));
            }
            
            let index = index as usize;
            
            if right.rank == 0 {
                return Err(JError::InvalidArgument("Cannot index into a scalar".to_string()));
            }
            
            if right.rank == 1 {
                if index >= right.shape[0] {
                    return Err(JError::InvalidArgument(format!(
                        "Index out of bounds: {} >= {}", index, right.shape[0]
                    )));
                }
                
                return Ok(JArray::scalar(right.data[index]));
            }
            
            // For higher dimensions, select a slice
            if index >= right.shape[0] {
                return Err(JError::InvalidArgument(format!(
                    "Index out of bounds: {} >= {}", index, right.shape[0]
                )));
            }
            
            let slice_size: usize = right.shape.iter().skip(1).product();
            let start = index * slice_size;
            let end = start + slice_size;
            let new_data = right.data[start..end].to_vec();
            let new_shape = right.shape.iter().skip(1).cloned().collect();
            
            Ok(JArray {
                rank: right.rank - 1,
                shape: new_shape,
                data: new_data,
            })
        } else {
            Err(JError::InvalidArgument("From requires a scalar left argument".to_string()))
        }
    }

    // Find function (~ dyad)
    fn find(&self, left: JArray, right: JArray) -> Result<JArray, JError> {
        if left.rank <= 1 && right.rank <= 1 {
            let result: Vec<f64> = left.data.iter()
                .map(|&a| {
                    match right.data.iter().position(|&b| (b - a).abs() < 1e-10) {
                        Some(pos) => pos as f64,
                        None => -1.0, // Not found
                    }
                })
                .collect();
            
            return Ok(JArray::vector(result));
        }
        
        Err(JError::InvalidArgument("Find requires vector arguments".to_string()))
    }

    // Reshape function (# dyad)
    fn reshape(&self, left: JArray, right: JArray) -> Result<JArray, JError> {
        let new_shape: Vec<usize> = if left.is_scalar() {
            vec![left.data[0] as usize]
        } else {
            left.data.iter().map(|&x| x as usize).collect()
        };
        
        // Calculate total size needed
        let total_size: usize = new_shape.iter().product();
        
        // Create new data array with cycling if needed
        let mut new_data = Vec::with_capacity(total_size);
        for i in 0..total_size {
            new_data.push(right.data[i % right.data.len()]);
        }
        
        Ok(JArray {
            rank: new_shape.len(),
            shape: new_shape,
            data: new_data,
        })
    }

    // Concatenate function (, dyad)
    fn concatenate(&self, left: JArray, right: JArray) -> Result<JArray, JError> {
        // Handle special cases for scalars
        let left_data = if left.is_scalar() {
            vec![left.data[0]]
        } else {
            left.data.clone()
        };
        
        let right_data = if right.is_scalar() {
            vec![right.data[0]]
        } else {
            right.data.clone()
        };
        
        // For simplicity, just concatenate the data vectors
        let mut result_data = left_data;
        result_data.extend_from_slice(&right_data);
        
        // Calculate result shape (for simple case)
        let result_shape = if left.rank <= 1 && right.rank <= 1 {
            vec![result_data.len()]
        } else {
            // For higher dimensions, would need more complex logic
            return Err(JError::ExecutionError(
                "Concatenation of multi-dimensional arrays not implemented".to_string()
            ));
        };
        
        Ok(JArray {
            rank: 1,
            shape: result_shape,
            data: result_data,
        })
    }
}

// Generate help text
pub fn get_help_text() -> String {
    r#"
J Language Web REPL - Help
==========================

This J interpreter implements a subset of the J programming language based on 
the original fragment. J is an array programming language particularly well-suited
for mathematical and statistical operations.

VERBS (OPERATORS)
----------------
Verbs can be used in monadic (single argument) or dyadic (two argument) form.

Monadic Verbs (prefix form):
+  (Plus)     Identity function: returns the argument unchanged
               Example: + 1 2 3  ->  [1 2 3]

{  (Brace)    Size: returns the size of the first dimension
               Example: { 1 2 3  ->  3

~  (Tilde)    Iota: generates array [0,1,2,...,n-1]
               Example: ~5  ->  [0 1 2 3 4]

<  (Less)     Box: encapsulate an array
               Example: < 1 2 3  ->  [1 2 3]

#  (Hash)     Shape: returns the dimensions of an array
               Example: # 2 3 # 1 2 3 4 5 6  ->  [2 3]

Dyadic Verbs (infix form):
+  (Plus)     Addition: element-wise addition of arrays
               Example: 1 2 3 + 4 5 6  ->  [5 7 9]

{  (Brace)    From: index selection
               Example: 1 { 7 8 9  ->  8

~  (Tilde)    Find: search for elements
               Example: 2 ~ 1 2 3  ->  1

#  (Hash)     Reshape: change dimensions while preserving data
               Example: 2 3 # 1 2 3 4 5 6  ->  [[1 2 3][4 5 6]]

,  (Comma)    Concatenate: join arrays together
               Example: 1 2 3 , 4 5 6  ->  [1 2 3 4 5 6]

ARRAYS
------
- Scalar: A single value (e.g., 42)
- Vector: A sequence of values separated by spaces (e.g., 1 2 3)
- Matrix: Created using reshape (e.g., 2 3 # 1 2 3 4 5 6)

EXAMPLES
--------
~5                    Generate array [0 1 2 3 4]
1 2 3 + 4 5 6         Add arrays element-wise
2 3 # 1 2 3 4 5 6     Reshape into a 2×3 matrix
1 { 7 8 9             Select element at index 1 (second element)
1 2 3 , 4 5 6         Concatenate arrays
# 2 3 # 1 2 3 4 5 6   Get shape of a matrix
"#.to_string()
}