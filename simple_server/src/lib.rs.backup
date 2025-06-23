use wasm_bindgen::prelude::*;

// Module declarations
pub mod tokenizer;
pub mod semantic_analyzer;
pub mod evaluator;
pub mod j_array;
pub mod parser;
pub mod lalr_parser_test;
pub mod test_suite;
pub mod visualizer;

// Conditional LALRPOP parser inclusion
#[cfg(not(target_arch = "wasm32"))]
pub mod lalr_parser;

#[cfg(target_arch = "wasm32")]
mod j_grammar_generated;

#[cfg(target_arch = "wasm32")]
pub mod lalr_parser {
    use crate::j_grammar_generated;
    
    pub struct LalrParser;
    
    impl LalrParser {
        pub fn new() -> Self {
            LalrParser
        }
        
        pub fn parse(&self, tokens: Vec<crate::tokenizer::Token>) -> Result<crate::parser::JNode, String> {
            // Use the generated parser directly with tokens
            j_grammar_generated::JExpressionParser::new()
                .parse(tokens.into_iter())
                .map_err(|e| format!("Parse error: {:?}", e))
        }
    }
}

use tokenizer::JTokenizer;
use lalr_parser::LalrParser;
use semantic_analyzer::JSemanticAnalyzer;
use evaluator::JEvaluator;

#[wasm_bindgen]
pub fn handle_j_eval_request(request_body: &str) -> String {
    // Set panic hook for better error messages in browser console
    console_error_panic_hook::set_once();
    
    // Parse form data: "expression=4+4#~16"
    let expression = match parse_form_data(request_body) {
        Some(expr) => expr,
        None => return r#"{"result": "Error: Invalid request format"}"#.to_string(),
    };
    
    // Use existing evaluation pipeline (unchanged)
    let tokenizer = JTokenizer::new();
    let lalr_parser = LalrParser::new();
    let semantic_analyzer = JSemanticAnalyzer::new();
    let evaluator = JEvaluator::new();
    
    let result = match tokenizer.tokenize(&expression) {
        Ok(tokens) => {
            match lalr_parser.parse(tokens) {
                Ok(ast) => {
                    match semantic_analyzer.analyze(ast) {
                        Ok(resolved_ast) => {
                            match evaluator.evaluate(&resolved_ast) {
                                Ok(result_array) => format!("{}", result_array),
                                Err(eval_err) => format!("Evaluation Error: {}", eval_err),
                            }
                        }
                        Err(semantic_err) => format!("Semantic Error: {}", semantic_err),
                    }
                }
                Err(parse_err) => format!("Parse Error: {}", parse_err),
            }
        }
        Err(token_err) => format!("Tokenization Error: {}", token_err),
    };
    
    // Return JSON in exact same format as server
    format!(r#"{{"result": "{}"}}"#, escape_json(&result))
}

fn parse_form_data(body: &str) -> Option<String> {
    // Parse "expression=..." form data
    if let Some(expr_start) = body.find("expression=") {
        let expr_part = &body[expr_start + 11..]; // Skip "expression="
        let expr_end = expr_part.find('&').unwrap_or(expr_part.len());
        let encoded_expr = &expr_part[..expr_end];
        
        // Simple URL decoding for common cases
        Some(encoded_expr.replace('+', " ").replace("%23", "#"))
    } else {
        None
    }
}

fn escape_json(s: &str) -> String {
    s.replace('\\', "\\\\")
     .replace('"', "\\\"")
     .replace('\n', "\\n")
     .replace('\r', "\\r")
     .replace('\t', "\\t")
}