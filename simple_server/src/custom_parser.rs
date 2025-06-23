// Custom Recursive Descent Parser - Phase 2 Implementation
// Supports: literals, basic addition, and monadic operations (~, -)

use crate::parser::{JNode, ParseError};
use crate::tokenizer::Token;

pub struct CustomParser {
    tokens: Vec<Token>,
    position: usize,
}

impl CustomParser {
    pub fn new() -> Self {
        CustomParser {
            tokens: Vec::new(),
            position: 0,
        }
    }
    
    pub fn parse(&mut self, tokens: Vec<Token>) -> Result<JNode, ParseError> {
        self.tokens = tokens;
        self.position = 0;
        
        if self.tokens.is_empty() {
            return Err(ParseError::NotImplemented("Error: Empty expression".to_string()));
        }
        
        self.parse_expression()
    }
    
    fn parse_expression(&mut self) -> Result<JNode, ParseError> {
        // Parse left operand (could be monadic expression)
        let mut left = self.parse_monadic()?;
        
        // Handle left-associative dyadic operations
        while self.position < self.tokens.len() {
            match &self.tokens[self.position] {
                Token::Verb('+') => {
                    self.position += 1; // consume '+'
                    let right = self.parse_monadic()?; // Right operand can also be monadic
                    left = JNode::AmbiguousVerb('+', Some(Box::new(left)), Some(Box::new(right)));
                }
                Token::Verb(op) => {
                    return Err(ParseError::NotImplemented(
                        format!("Error: Operator '{}' not implemented in Phase 2", op)
                    ));
                }
                Token::Vector(_) => {
                    return Err(ParseError::NotImplemented(
                        "Error: Array literals not implemented".to_string()
                    ));
                }
                _ => {
                    return Err(ParseError::NotImplemented(
                        format!("Error: Unexpected token at position {}", self.position)
                    ));
                }
            }
        }
        
        Ok(left)
    }
    
    fn parse_monadic(&mut self) -> Result<JNode, ParseError> {
        // Handle monadic operators (higher precedence than dyadic)
        if self.position < self.tokens.len() {
            match &self.tokens[self.position] {
                Token::Verb('~') => {
                    self.position += 1; // consume '~'
                    let operand = self.parse_literal()?;
                    return Ok(JNode::AmbiguousVerb('~', None, Some(Box::new(operand))));
                }
                Token::Verb('-') => {
                    self.position += 1; // consume '-'
                    let operand = self.parse_literal()?;
                    return Ok(JNode::AmbiguousVerb('-', None, Some(Box::new(operand))));
                }
                _ => {}
            }
        }
        
        // Fall back to literal parsing
        self.parse_literal()
    }
    
    fn parse_literal(&mut self) -> Result<JNode, ParseError> {
        if self.position >= self.tokens.len() {
            return Err(ParseError::NotImplemented(
                "Error: Expected number but reached end of input".to_string()
            ));
        }
        
        match &self.tokens[self.position] {
            Token::Vector(array) => {
                // For Phase 1, only support single integer literals
                if array.data.len() == 1 {
                    let node = JNode::Literal(array.clone());
                    self.position += 1;
                    Ok(node)
                } else {
                    Err(ParseError::NotImplemented(
                        "Error: Multi-element arrays not implemented in Phase 1".to_string()
                    ))
                }
            }
            Token::Verb('~') | Token::Verb('-') => {
                Err(ParseError::NotImplemented(
                    "Error: Monadic operator found where literal expected".to_string()
                ))
            }
            _ => {
                Err(ParseError::NotImplemented(
                    format!("Error: Expected number at position {}", self.position)
                ))
            }
        }
    }
}