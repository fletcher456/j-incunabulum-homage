// J Interpreter Module
// Implementation of a J interpreter with AST-based evaluation

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

// AST node structure for representing J expressions
#[derive(Debug, Clone)]
pub enum JNode {
    Literal(JArray),                        // A literal value (scalar or array)
    MonadicVerb(char, Box<JNode>),          // A monadic verb with its argument
    DyadicVerb(char, Box<JNode>, Box<JNode>)// A dyadic verb with left and right arguments
}

// Token types for parsing
#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(i64),
    Verb(char),
    Space,
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

    // Function to tokenize a J expression
    fn tokenize(&self, input: &str) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();
        
        while let Some(&c) = chars.peek() {
            match c {
                '0'..='9' => {
                    let mut number = String::new();
                    while let Some(&c) = chars.peek() {
                        if c.is_digit(10) {
                            number.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    match number.parse::<i64>() {
                        Ok(n) => tokens.push(Token::Number(n)),
                        Err(_) => return Err(format!("Invalid number: {}", number)),
                    }
                },
                '+' | '~' | '#' | '<' | '{' | ',' => {
                    tokens.push(Token::Verb(c));
                    chars.next();
                },
                ' ' => {
                    tokens.push(Token::Space);
                    chars.next();
                },
                _ => {
                    return Err(format!("Unknown token: {}", c));
                }
            }
        }
        
        Ok(tokens)
    }
    
    // Build a vector JArray from consecutive number tokens
    fn build_vector(&self, tokens: &[Token], start_idx: usize) -> Result<(JArray, usize), String> {
        let mut values = Vec::new();
        let mut idx = start_idx;
        
        // First token should be a number
        if idx >= tokens.len() {
            return Err("Expected number for vector, but found end of input".to_string());
        }
        
        if let Token::Number(n) = tokens[idx] {
            values.push(n);
            idx += 1;
        } else {
            return Err("Expected number for vector".to_string());
        }
        
        // Look for more numbers separated by spaces
        while idx + 1 < tokens.len() {
            if let Token::Space = tokens[idx] {
                if let Token::Number(n) = tokens[idx + 1] {
                    values.push(n);
                    idx += 2; // Skip the space and the number
                } else {
                    break; // Not a number after space, end of vector
                }
            } else {
                break; // Not a space, end of vector
            }
        }
        
        // Create the vector JArray
        let array = if values.len() == 1 {
            // Single value, create a scalar
            JArray::new_scalar(values[0])
        } else {
            // Multiple values, create a vector
            JArray::new_integer(1, vec![values.len()], values)
        };
        
        Ok((array, idx))
    }
    
    // Parse tokens into an AST using bottom-up approach with backtracking
    fn parse(&self, tokens: Vec<Token>) -> Result<JNode, String> {
        if tokens.is_empty() {
            return Err("Empty expression".to_string());
        }
        
        // Use the new backtracking parser
        self.parse_with_backtracking(tokens)
    }
    
    // Parse a subexpression - handles common patterns
    fn parse_subexpression(&self, tokens: &[Token]) -> Result<JNode, String> {
        if tokens.is_empty() {
            return Err("Empty subexpression".to_string());
        }
        
        // Single number token
        if tokens.len() == 1 {
            if let Token::Number(n) = tokens[0] {
                return Ok(JNode::Literal(JArray::new_scalar(n)));
            }
        }
        
        // Handle vectors (space-separated numbers)
        if let Token::Number(_) = tokens[0] {
            // Try to parse a vector starting at index 0
            let (vector, end_idx) = self.build_vector(tokens, 0)?;
            
            // If we consumed all tokens, return the vector literal
            if end_idx >= tokens.len() {
                return Ok(JNode::Literal(vector));
            }
        }
        
        // Handle monadic verb patterns
        if tokens.len() >= 2 && matches!(tokens[0], Token::Verb(_)) {
            let verb = if let Token::Verb(v) = tokens[0] {
                v
            } else {
                return Err("Expected verb".to_string());
            };
            
            // Parse the rest of the tokens as the argument to the monadic verb
            let arg_result = self.parse_subexpression(&tokens[1..]);
            if arg_result.is_ok() {
                return Ok(JNode::MonadicVerb(verb, Box::new(arg_result.unwrap())));
            }
            
            // If we couldn't parse a complex expression, try a simple vector
            if matches!(tokens[1], Token::Number(_)) {
                let (arg_vector, _) = self.build_vector(tokens, 1)?;
                return Ok(JNode::MonadicVerb(verb, Box::new(JNode::Literal(arg_vector))));
            }
        }
        
        Err("Could not parse subexpression".to_string())
    }
    
    // Bottom-up parsing with backtracking
    fn parse_with_backtracking(&self, tokens: Vec<Token>) -> Result<JNode, String> {
        println!("DEBUG: Parsing tokens: {:?}", tokens);
        let mut backtrack_count = 0;
        let max_backtrack = 10;
        
        // Start with the whole expression
        let mut right_pos = tokens.len();
        
        while right_pos > 0 && backtrack_count < max_backtrack {
            // Try to parse the rightmost part first
            let right_tokens = &tokens[tokens.len().saturating_sub(right_pos)..];
            println!("DEBUG: Trying to parse right tokens: {:?}", right_tokens);
            let right_result = self.parse_subexpression(right_tokens);
            
            if let Ok(right_node) = right_result {
                println!("DEBUG: Successfully parsed right side: {:?}", right_node);
                // If we parsed the entire expression, we're done
                if right_tokens.len() == tokens.len() {
                    return Ok(right_node);
                }
                
                // Check if there's a potential dyadic operation
                let left_end = tokens.len() - right_tokens.len();
                if left_end > 0 && matches!(tokens[left_end-1], Token::Verb(_)) {
                    // We have a verb before the right expression, try to parse the left side
                    let verb_pos = left_end - 1;
                    let verb = if let Token::Verb(v) = tokens[verb_pos] { v } else { '+' }; // Default shouldn't happen
                    println!("DEBUG: Found verb '{}' at position {}", verb, verb_pos);
                    
                    if verb_pos > 0 {
                        let left_tokens = &tokens[0..verb_pos];
                        println!("DEBUG: Trying to parse left tokens: {:?}", left_tokens);
                        let left_result = self.parse_subexpression(left_tokens);
                        
                        if let Ok(left_node) = left_result {
                            println!("DEBUG: Successfully parsed left side: {:?}", left_node);
                            // We successfully parsed a dyadic operation
                            return Ok(JNode::DyadicVerb(verb, Box::new(left_node), Box::new(right_node)));
                        } else {
                            println!("DEBUG: Failed to parse left side: {:?}", left_result);
                        }
                    }
                }
            } else {
                println!("DEBUG: Failed to parse right side: {:?}", right_result);
            }
            
            // Backtrack by trying to parse a smaller part from the right
            right_pos -= 1;
            backtrack_count += 1;
            println!("DEBUG: Backtracking, new right_pos: {}", right_pos);
        }
        
        if backtrack_count >= max_backtrack {
            return Err("Parse error: Too much backtracking required".to_string());
        }
        
        Err("Could not parse expression".to_string())
    }
    
    // Evaluate an AST node
    fn eval_node(&self, node: &JNode) -> Result<JArray, String> {
        match node {
            JNode::Literal(array) => Ok(array.clone()),
            
            JNode::MonadicVerb(verb, arg) => {
                let arg_value = self.eval_node(arg)?;
                
                match verb {
                    '~' => self.iota(arg_value.data[0]),
                    '+' => self.plus_monadic(&arg_value),
                    _ => Err(format!("Unsupported monadic verb: {}", verb)),
                }
            },
            
            JNode::DyadicVerb(verb, left, right) => {
                let left_value = self.eval_node(left)?;
                let right_value = self.eval_node(right)?;
                
                match verb {
                    '+' => self.plus_dyadic(&left_value, &right_value),
                    _ => Err(format!("Unsupported dyadic verb: {}", verb)),
                }
            },
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
        
        // Try to parse as complex expression with AST
        if input.contains('+') || input.contains('~') {
            // Tokenize the input
            let tokens = self.tokenize(input)?;
            
            // Parse tokens into an AST
            let ast = self.parse(tokens)?;
            
            // Evaluate the AST
            return self.eval_node(&ast);
        }
        
        // Handle simple cases directly
        
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