// J Interpreter Module
// Implementation of a J interpreter with AST-based evaluation

use std::fmt;

// J array types
#[derive(Debug, Clone, PartialEq)]
pub enum JType {
    Integer,
    Box,
}

// J array structure (similar to the C version's 'A' struct)
#[derive(Debug, Clone, PartialEq)]
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
    // Final resolved nodes
    Literal(JArray),                        // A literal value (scalar or array)
    MonadicVerb(char, Box<JNode>),          // A monadic verb with its argument
    DyadicVerb(char, Box<JNode>, Box<JNode>),// A dyadic verb with left and right arguments
    
    // Intermediate ambiguous nodes for context resolution
    AmbiguousVerb(char, Option<Box<JNode>>, Option<Box<JNode>>), // Verb with optional left and right operands
}

// Token types for parsing
#[derive(Debug, Clone, PartialEq)]
enum Token {
    Vector(JArray),
    Verb(char),
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

    // Function to tokenize a J expression into vectors and verbs
    fn tokenize(&self, input: &str) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();
        
        while let Some(&c) = chars.peek() {
            match c {
                '0'..='9' => {
                    // Parse a vector (space-separated numbers)
                    let mut numbers = Vec::new();
                    
                    // Parse the first number
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
                        Ok(n) => numbers.push(n),
                        Err(_) => return Err(format!("Invalid number: {}", number)),
                    }
                    
                    // Look for more space-separated numbers
                    while let Some(&next_char) = chars.peek() {
                        if next_char == ' ' {
                            chars.next(); // consume the space
                            
                            // Skip any additional spaces
                            while let Some(&c) = chars.peek() {
                                if c == ' ' {
                                    chars.next();
                                } else {
                                    break;
                                }
                            }
                            
                            // Check if the next character starts a number
                            if let Some(&c) = chars.peek() {
                                if c.is_digit(10) {
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
                                        Ok(n) => numbers.push(n),
                                        Err(_) => return Err(format!("Invalid number: {}", number)),
                                    }
                                } else {
                                    break; // Not a number, end of vector
                                }
                            } else {
                                break; // End of input
                            }
                        } else {
                            break; // Not a space, end of vector
                        }
                    }
                    
                    // Create JArray token
                    let jarray = if numbers.len() == 1 {
                        JArray::new_scalar(numbers[0])
                    } else {
                        JArray::new_integer(1, vec![numbers.len()], numbers)
                    };
                    tokens.push(Token::Vector(jarray));
                },
                '+' | '~' | '#' | '<' | '{' | ',' => {
                    tokens.push(Token::Verb(c));
                    chars.next();
                },
                ' ' => {
                    // Skip standalone spaces (they're handled in number parsing)
                    chars.next();
                },
                _ => {
                    return Err(format!("Unknown token: {}", c));
                }
            }
        }
        
        Ok(tokens)
    }
    

    
    // Parse tokens into an AST using three-phase strategy
    fn parse(&self, tokens: Vec<Token>) -> Result<JNode, String> {
        if tokens.is_empty() {
            return Err("Empty expression".to_string());
        }
        
        // Phase 1: Context-free parsing with ambiguous verbs
        let ambiguous_ast = self.parse_context_free(tokens)?;
        
        // Phase 2: Context resolution
        let resolved_ast = self.resolve_context(ambiguous_ast)?;
        
        // Phase 3: Right-to-left restructuring
        let final_ast = self.restructure_right_associative(resolved_ast)?;
        
        Ok(final_ast)
    }
    

    
    // Phase 1: Context-free parsing - creates ambiguous verb nodes
    fn parse_context_free(&self, tokens: Vec<Token>) -> Result<JNode, String> {
        if tokens.is_empty() {
            return Err("Empty expression".to_string());
        }
        
        self.parse_expression(&tokens, 0).map(|(node, _)| node)
    }
    
    // Recursive descent parser for expressions
    fn parse_expression(&self, tokens: &[Token], pos: usize) -> Result<(JNode, usize), String> {
        if pos >= tokens.len() {
            return Err("Unexpected end of input".to_string());
        }
        
        let (term, mut new_pos) = self.parse_term(tokens, pos)?;
        
        // Check if there's a verb followed by another expression
        if new_pos < tokens.len() {
            if let Token::Verb(verb) = tokens[new_pos] {
                new_pos += 1;
                if new_pos < tokens.len() {
                    let (right_expr, final_pos) = self.parse_expression(tokens, new_pos)?;
                    return Ok((JNode::AmbiguousVerb(verb, Some(Box::new(term)), Some(Box::new(right_expr))), final_pos));
                } else {
                    return Err("Expected expression after verb".to_string());
                }
            }
        }
        
        Ok((term, new_pos))
    }
    
    // Parse a term (verb + expression, atom, or grouped expression)
    fn parse_term(&self, tokens: &[Token], pos: usize) -> Result<(JNode, usize), String> {
        if pos >= tokens.len() {
            return Err("Unexpected end of input".to_string());
        }
        
        match &tokens[pos] {
            Token::Verb(verb) => {
                // Leading verb - will be resolved as monadic in Phase 2
                let (expr, new_pos) = self.parse_expression(tokens, pos + 1)?;
                Ok((JNode::AmbiguousVerb(*verb, None, Some(Box::new(expr))), new_pos))
            }
            Token::Vector(jarray) => {
                // Literal vector/scalar
                Ok((JNode::Literal(jarray.clone()), pos + 1))
            }
        }
    }
    
    // Phase 2: Context resolution - convert ambiguous verbs to monadic/dyadic
    fn resolve_context(&self, node: JNode) -> Result<JNode, String> {
        match node {
            JNode::AmbiguousVerb(verb, left, right) => {
                match (left, right) {
                    (None, Some(right)) => {
                        // Leading verb - monadic
                        Ok(JNode::MonadicVerb(verb, Box::new(self.resolve_context(*right)?)))
                    }
                    (Some(left), Some(right)) => {
                        // Verb between expressions - dyadic
                        Ok(JNode::DyadicVerb(
                            verb,
                            Box::new(self.resolve_context(*left)?),
                            Box::new(self.resolve_context(*right)?)
                        ))
                    }
                    _ => Err("Invalid verb context".to_string())
                }
            }
            JNode::Literal(array) => Ok(JNode::Literal(array)),
            JNode::MonadicVerb(verb, arg) => {
                Ok(JNode::MonadicVerb(verb, Box::new(self.resolve_context(*arg)?)))
            }
            JNode::DyadicVerb(verb, left, right) => {
                Ok(JNode::DyadicVerb(
                    verb,
                    Box::new(self.resolve_context(*left)?),
                    Box::new(self.resolve_context(*right)?)
                ))
            }
        }
    }
    
    // Phase 3: Right-to-left restructuring for J's evaluation order
    fn restructure_right_associative(&self, node: JNode) -> Result<JNode, String> {
        match node {
            JNode::DyadicVerb(op, left, right) => {
                let resolved_left = self.restructure_right_associative(*left)?;
                let resolved_right = self.restructure_right_associative(*right)?;
                
                // J evaluates right-to-left, so we keep the current structure
                // The right-associative nature is inherent in our recursive parsing
                Ok(JNode::DyadicVerb(op, Box::new(resolved_left), Box::new(resolved_right)))
            }
            JNode::MonadicVerb(verb, arg) => {
                Ok(JNode::MonadicVerb(verb, Box::new(self.restructure_right_associative(*arg)?)))
            }
            other => Ok(other)
        }
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
            
            JNode::AmbiguousVerb(_, _, _) => {
                Err("Internal error: AmbiguousVerb node should have been resolved before evaluation".to_string())
            }
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