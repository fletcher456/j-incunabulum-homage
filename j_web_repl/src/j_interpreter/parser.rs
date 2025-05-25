// Parser for J expressions
use crate::j_interpreter::JError;
use crate::j_interpreter::array::JArray;
use std::str::FromStr;

// Verb characters from the feature list
const VERB_CHARS: &[char] = &['+', '{', '~', '<', '#', ','];

// Parse a J expression into a tuple of (left operand, verb, right operand)
// Any of them can be None if not present in the expression
pub fn parse(expression: &str) -> Result<(Option<JArray>, Option<char>, Option<JArray>), JError> {
    let expression = expression.trim();
    
    if expression.is_empty() {
        return Err(JError::ParseError("Empty expression".to_string()));
    }
    
    // Split the expression by verb characters
    let mut parts: Vec<&str> = Vec::new();
    let mut verb: Option<char> = None;
    
    let mut current_part = String::new();
    let mut in_number = false;
    
    for (i, c) in expression.chars().enumerate() {
        if VERB_CHARS.contains(&c) && !in_number {
            // We found a verb
            if !current_part.is_empty() {
                parts.push(&expression[i - current_part.len()..i]);
                current_part.clear();
            }
            
            verb = Some(c);
            
            // Skip the verb character for the next part
            continue;
        }
        
        // Track if we're inside a number (to avoid splitting on - sign inside numbers)
        if c.is_digit(10) || c == '.' {
            in_number = true;
        } else if c.is_whitespace() {
            in_number = false;
        }
        
        current_part.push(c);
    }
    
    // Add the last part if not empty
    if !current_part.is_empty() {
        parts.push(&expression[expression.len() - current_part.len()..]);
    }
    
    // Handle different cases based on number of parts
    match parts.len() {
        0 => {
            // Only a verb
            if let Some(v) = verb {
                return Ok((None, Some(v), None));
            }
            Err(JError::ParseError("Invalid expression".to_string()))
        }
        1 => {
            // A single array or a monadic verb with an array
            let right = parse_array(parts[0])?;
            Ok((None, verb, Some(right)))
        }
        2 => {
            // A dyadic verb with left and right operands
            if verb.is_none() {
                return Err(JError::ParseError("Missing verb between operands".to_string()));
            }
            
            let left = parse_array(parts[0])?;
            let right = parse_array(parts[1])?;
            Ok((Some(left), verb, Some(right)))
        }
        _ => Err(JError::ParseError("Too many parts in expression".to_string())),
    }
}

// Parse a string into a JArray
fn parse_array(s: &str) -> Result<JArray, JError> {
    let s = s.trim();
    
    if s.is_empty() {
        return Err(JError::ParseError("Empty array".to_string()));
    }
    
    // Check if it's a single number
    if let Ok(value) = f64::from_str(s) {
        return Ok(JArray::scalar(value));
    }
    
    // Split by whitespace for a vector
    let parts: Vec<&str> = s.split_whitespace().collect();
    
    if parts.is_empty() {
        return Err(JError::ParseError("Invalid array format".to_string()));
    }
    
    // Parse each part as a number
    let mut values = Vec::with_capacity(parts.len());
    
    for part in parts {
        match f64::from_str(part) {
            Ok(value) => values.push(value),
            Err(_) => return Err(JError::ParseError(format!("Invalid number: {}", part))),
        }
    }
    
    Ok(JArray::vector(values))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_scalar() {
        let result = parse_array("42").unwrap();
        assert!(result.is_scalar());
    }

    #[test]
    fn test_parse_vector() {
        let result = parse_array("1 2 3").unwrap();
        assert!(result.is_vector());
        assert_eq!(result.size(), 3);
    }

    #[test]
    fn test_parse_expression() {
        let (left, verb, right) = parse("1 2 3 + 4 5 6").unwrap();
        assert!(left.is_some());
        assert_eq!(verb, Some('+'));
        assert!(right.is_some());
    }

    #[test]
    fn test_parse_monadic() {
        let (left, verb, right) = parse("~5").unwrap();
        assert!(left.is_none());
        assert_eq!(verb, Some('~'));
        assert!(right.is_some());
    }
}