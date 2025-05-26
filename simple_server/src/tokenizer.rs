// J Tokenizer Module
// Lexical analysis for J language expressions

use crate::j_array::JArray;
use std::fmt;

// Token types for parsing
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Vector(JArray),
    Verb(char),
    LeftParen,
    RightParen,
}

// Tokenization errors
#[derive(Debug, Clone)]
pub enum TokenError {
    InvalidNumber(String),
    UnknownCharacter(char),
    InvalidVector(String),
}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenError::InvalidNumber(s) => write!(f, "Invalid number: {}", s),
            TokenError::UnknownCharacter(c) => write!(f, "Unknown character: {}", c),
            TokenError::InvalidVector(s) => write!(f, "Invalid vector: {}", s),
        }
    }
}

// J Tokenizer
pub struct JTokenizer;

impl JTokenizer {
    pub fn new() -> Self {
        JTokenizer
    }

    // Function to tokenize a J expression into vectors and verbs
    pub fn tokenize(&self, input: &str) -> Result<Vec<Token>, TokenError> {
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
                        Err(_) => return Err(TokenError::InvalidNumber(number)),
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
                                        Err(_) => return Err(TokenError::InvalidNumber(number)),
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
                '(' => {
                    tokens.push(Token::LeftParen);
                    chars.next();
                },
                ')' => {
                    tokens.push(Token::RightParen);
                    chars.next();
                },
                ' ' => {
                    // Skip standalone spaces (they're handled in number parsing)
                    chars.next();
                },
                _ => {
                    return Err(TokenError::UnknownCharacter(c));
                }
            }
        }
        
        Ok(tokens)
    }
}