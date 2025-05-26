// LALRPOP Generated Parser Wrapper
use crate::j_array::JArray;
use crate::parser::{JNode, ParseError};
use crate::tokenizer::Token;
use lalrpop_util::ParseError as LalrpopParseError;

// Include the generated parser
lalrpop_mod!(pub j_grammar); // This will include the generated parser

pub struct LalrParser;

impl LalrParser {
    pub fn new() -> Self {
        LalrParser
    }
    
    pub fn parse(&self, tokens: Vec<Token>) -> Result<JNode, ParseError> {
        let parser = j_grammar::JExpressionParser::new();
        
        match parser.parse(tokens.iter()) {
            Ok(ast) => Ok(ast),
            Err(err) => {
                let error_msg = format!("LALRPOP Parse Error: {:?}", err);
                Err(ParseError::InvalidExpression(error_msg))
            }
        }
    }
}