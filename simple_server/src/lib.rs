use wasm_bindgen::prelude::*;
use crate::tokenizer::JTokenizer;
use crate::custom_parser::CustomParser;
use crate::semantic_analyzer::JSemanticAnalyzer;
use crate::evaluator::JEvaluator;

// TEMPORARILY UNUSED - Complex J interpreter
// Module declarations
pub mod tokenizer;
pub mod semantic_analyzer;
pub mod evaluator;
pub mod j_array;
pub mod parser;
// pub mod test_suite;
// pub mod visualizer;
pub mod custom_parser;

// use tokenizer::JTokenizer;
// use custom_parser::CustomParser;
// use semantic_analyzer::JSemanticAnalyzer;
// use evaluator::JEvaluator;

// STUB INTERPRETER - Always returns "foo" for WASM analysis
#[wasm_bindgen]
pub fn evaluate_j_expression(expression: &str) -> String {
    console_error_panic_hook::set_once();
    
    // Phase 1: Complete J expression evaluation pipeline
    let tokenizer = JTokenizer::new();
    let mut parser = CustomParser::new();
    let semantic_analyzer = JSemanticAnalyzer::new();
    let evaluator = JEvaluator::new();
    
    match tokenizer.tokenize(expression) {
        Ok(tokens) => {
            match parser.parse(tokens) {
                Ok(ast) => {
                    match semantic_analyzer.analyze(ast) {
                        Ok(resolved_ast) => {
                            match evaluator.evaluate(&resolved_ast) {
                                Ok(result) => format!("{}", result),
                                Err(e) => format!("Evaluation error: {}", e)
                            }
                        },
                        Err(e) => format!("Semantic error: {}", e)
                    }
                },
                Err(e) => format!("Parse error: {}", e)
            }
        },
        Err(e) => format!("Tokenization error: {}", e)
    }
}

// TEMPORARILY UNUSED - Complex evaluation logic
/*
// Main WASM entry point for direct expression evaluation
#[wasm_bindgen]
pub fn evaluate_j_expression(expression: &str) -> String {
    console_error_panic_hook::set_once();
    
    let tokenizer = JTokenizer::new();
    let mut custom_parser = CustomParser::new();
    let semantic_analyzer = JSemanticAnalyzer::new();
    let evaluator = JEvaluator::new();
    
    let result = match tokenizer.tokenize(expression) {
        Ok(tokens) => {
            match custom_parser.parse(tokens) {
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
    
    result
}
*/

// JSON-compatible interface for web integration
#[wasm_bindgen]
pub fn handle_j_eval_request(request_body: &str) -> String {
    console_error_panic_hook::set_once();
    
    // Parse form data: "expression=4+4#~16"
    let expression = match parse_form_data(request_body) {
        Some(expr) => expr,
        None => return r#"{"result": "Error: Invalid request format"}"#.to_string(),
    };
    
    let result = evaluate_j_expression(&expression);
    format!(r#"{{"result": "{}"}}"#, escape_json(&result))
}

fn parse_form_data(body: &str) -> Option<String> {
    if let Some(expr_start) = body.find("expression=") {
        let expr_part = &body[expr_start + 11..];
        let expr_end = expr_part.find('&').unwrap_or(expr_part.len());
        let encoded_expr = &expr_part[..expr_end];
        
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