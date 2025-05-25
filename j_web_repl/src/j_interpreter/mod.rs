// J Interpreter Implementation
// Based on the feature list extracted from the J fragment

pub mod array;
pub mod parser;
pub mod verbs;
pub mod help;

use array::JArray;
use parser::parse;
use std::fmt;
use verbs::execute_verb;

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

// Execute a J expression and return the result
pub fn execute(expression: &str) -> Result<String, JError> {
    // Special case for help command
    if expression.trim() == "help" {
        return Ok(help::get_help_text());
    }

    // Parse the expression
    let parsed = parse(expression)?;
    
    // Execute the parsed expression
    match parsed {
        // For now, we'll handle simple cases
        (Some(left), Some(verb), Some(right)) => {
            // Dyadic verb case
            let result = execute_verb(Some(&left), verb, &right)?;
            Ok(format!("{}", result))
        }
        (None, Some(verb), Some(right)) => {
            // Monadic verb case
            let result = execute_verb(None, verb, &right)?;
            Ok(format!("{}", result))
        }
        (None, None, Some(right)) => {
            // Just a value
            Ok(format!("{}", right))
        }
        _ => Err(JError::ParseError("Invalid expression structure".to_string())),
    }
}

// Test functions for the J interpreter
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_expression() {
        assert!(execute("1 2 3").is_ok());
        assert!(execute("1 2 3 + 4 5 6").is_ok());
    }
}