// J Parser Module
// Syntactic analysis for J language expressions

use crate::j_array::JArray;
use crate::tokenizer::Token;
use std::fmt;

// AST node structure for representing J expressions
#[derive(Debug, Clone)]
pub enum JNode {
    // Final resolved nodes
    Literal(JArray),
    MonadicVerb(char, Box<JNode>),
    DyadicVerb(char, Box<JNode>, Box<JNode>),
    
    // Intermediate ambiguous nodes for context resolution
    AmbiguousVerb(char, Option<Box<JNode>>, Option<Box<JNode>>),
}

// Parse errors
#[derive(Debug, Clone)]
pub enum ParseError {
    UnexpectedToken(String, usize),
    UnexpectedEndOfInput,
    InvalidExpression(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken(token, pos) => write!(f, "Unexpected token '{}' at position {}", token, pos),
            ParseError::UnexpectedEndOfInput => write!(f, "Unexpected end of input"),
            ParseError::InvalidExpression(msg) => write!(f, "Invalid expression: {}", msg),
        }
    }
}

// J Parser
pub struct JParser;

impl JParser {
    pub fn new() -> Self {
        JParser
    }

    // Parse tokens into an AST using context-free approach
    pub fn parse(&self, tokens: Vec<Token>) -> Result<JNode, ParseError> {
        if tokens.is_empty() {
            return Err(ParseError::InvalidExpression("Empty expression".to_string()));
        }
        
        let (node, pos) = self.parse_expression(&tokens, 0)?;
        
        // Ensure we consumed all tokens
        if pos < tokens.len() {
            return Err(ParseError::UnexpectedToken(
                format!("{:?}", tokens[pos]), 
                pos
            ));
        }
        
        Ok(node)
    }
    
    // Recursive descent parser for expressions
    fn parse_expression(&self, tokens: &[Token], pos: usize) -> Result<(JNode, usize), ParseError> {
        if pos >= tokens.len() {
            return Err(ParseError::UnexpectedEndOfInput);
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
                    return Err(ParseError::InvalidExpression("Expected expression after verb".to_string()));
                }
            }
        }
        
        Ok((term, new_pos))
    }
    
    // Parse a term (verb + expression, atom, or grouped expression)
    fn parse_term(&self, tokens: &[Token], pos: usize) -> Result<(JNode, usize), ParseError> {
        if pos >= tokens.len() {
            return Err(ParseError::UnexpectedEndOfInput);
        }
        
        match &tokens[pos] {
            Token::Verb(verb) => {
                // Leading verb - will be resolved as monadic in semantic analysis
                let (expr, new_pos) = self.parse_expression(tokens, pos + 1)?;
                Ok((JNode::AmbiguousVerb(*verb, None, Some(Box::new(expr))), new_pos))
            }
            Token::Vector(jarray) => {
                // Literal vector/scalar
                Ok((JNode::Literal(jarray.clone()), pos + 1))
            }
        }
    }
}