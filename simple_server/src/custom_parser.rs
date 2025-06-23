// Custom Recursive Descent Parser - Phase 4 Implementation
// Supports: array literals, basic addition, monadic operations (~, -), and J array operators (#, {, ,, <)

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
        // Parse left operand (could be J operators)
        let mut left = self.parse_j_operators()?;
        
        // Handle dyadic addition (lowest precedence)
        while self.position < self.tokens.len() {
            match &self.tokens[self.position] {
                Token::Verb('+') => {
                    self.position += 1; // consume '+'
                    let right = self.parse_j_operators()?; // Right operand can also be J operators
                    left = JNode::AmbiguousVerb('+', Some(Box::new(left)), Some(Box::new(right)));
                }
                Token::Verb(op) => {
                    return Err(ParseError::NotImplemented(
                        format!("Error: Operator '{}' not implemented in Phase 4", op)
                    ));
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_j_operators(&mut self) -> Result<JNode, ParseError> {
        let mut left = self.parse_monadic()?;
        
        while self.position < self.tokens.len() {
            match &self.tokens[self.position] {
                Token::Verb('#') => {
                    self.position += 1;
                    let right = self.parse_monadic()?;
                    left = JNode::AmbiguousVerb('#', Some(Box::new(left)), Some(Box::new(right)));
                }
                Token::Verb('{') => {
                    self.position += 1;
                    let right = self.parse_monadic()?;
                    left = JNode::AmbiguousVerb('{', Some(Box::new(left)), Some(Box::new(right)));
                }
                Token::Verb(',') => {
                    self.position += 1;
                    let right = self.parse_monadic()?;
                    left = JNode::AmbiguousVerb(',', Some(Box::new(left)), Some(Box::new(right)));
                }
                Token::Verb('<') => {
                    self.position += 1;
                    let right = self.parse_monadic()?;
                    left = JNode::AmbiguousVerb('<', Some(Box::new(left)), Some(Box::new(right)));
                }
                _ => break,
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
                // Phase 3: Accept all vector sizes
                let node = JNode::Literal(array.clone());
                self.position += 1;
                Ok(node)
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