// LALRPOP Generated Parser Wrapper
use crate::parser::{JNode, ParseError};
use crate::tokenizer::Token;
use lalrpop_util::lalrpop_mod;

// Include the generated parser
lalrpop_mod!(pub j_grammar);

pub struct LalrParser;

impl LalrParser {
    pub fn new() -> Self {
        LalrParser
    }
    
    pub fn parse(&self, tokens: Vec<Token>) -> Result<JNode, ParseError> {
        let parser = j_grammar::JExpressionParser::new();
        
        // Convert tokens to LALRPOP format with position information
        let positioned_tokens: Vec<(usize, Token, usize)> = tokens
            .into_iter()
            .enumerate()
            .map(|(i, token)| (i, token, i + 1))
            .collect();
        
        match parser.parse(positioned_tokens.iter().cloned()) {
            Ok(ast) => Ok(ast),
            Err(err) => {
                let error_msg = format!("LALRPOP Parse Error: {:?}", err);
                Err(ParseError::InvalidExpression(error_msg))
            }
        }
    }
}