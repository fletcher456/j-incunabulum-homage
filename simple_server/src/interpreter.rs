// J Interpreter Module
// Main interface that coordinates all modules

use crate::j_array::JArray;
use crate::tokenizer::{JTokenizer, TokenError};
use crate::parser::{JParser, ParseError, JNode};
use crate::semantic_analyzer::{JSemanticAnalyzer, SemanticError};
use crate::evaluator::{JEvaluator, EvaluationError};
use crate::visualizer::ParseTreeVisualizer;
use std::fmt;

// Unified interpreter error type
#[derive(Debug, Clone)]
pub enum InterpreterError {
    TokenError(TokenError),
    ParseError(ParseError),
    SemanticError(SemanticError),
    EvaluationError(EvaluationError),
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InterpreterError::TokenError(e) => write!(f, "Token error: {}", e),
            InterpreterError::ParseError(e) => write!(f, "Parse error: {}", e),
            InterpreterError::SemanticError(e) => write!(f, "Semantic error: {}", e),
            InterpreterError::EvaluationError(e) => write!(f, "Evaluation error: {}", e),
        }
    }
}

// Convert from individual error types
impl From<TokenError> for InterpreterError {
    fn from(error: TokenError) -> Self {
        InterpreterError::TokenError(error)
    }
}

impl From<ParseError> for InterpreterError {
    fn from(error: ParseError) -> Self {
        InterpreterError::ParseError(error)
    }
}

impl From<SemanticError> for InterpreterError {
    fn from(error: SemanticError) -> Self {
        InterpreterError::SemanticError(error)
    }
}

impl From<EvaluationError> for InterpreterError {
    fn from(error: EvaluationError) -> Self {
        InterpreterError::EvaluationError(error)
    }
}

// Main J Interpreter
pub struct JInterpreter {
    tokenizer: JTokenizer,
    parser: JParser,
    semantic_analyzer: JSemanticAnalyzer,
    evaluator: JEvaluator,
    visualizer: ParseTreeVisualizer,
}

impl JInterpreter {
    // Create a new interpreter
    pub fn new() -> Self {
        JInterpreter {
            tokenizer: JTokenizer::new(),
            parser: JParser::new(),
            semantic_analyzer: JSemanticAnalyzer::new(),
            evaluator: JEvaluator::new(),
            visualizer: ParseTreeVisualizer::new(),
        }
    }

    // Execute a J expression through the complete pipeline
    pub fn execute(&self, input: &str) -> Result<JArray, InterpreterError> {
        let (result, _) = self.execute_with_debug(input)?;
        Ok(result)
    }

    // Execute with debug information including parse tree visualization
    pub fn execute_with_debug(&self, input: &str) -> Result<(JArray, String), InterpreterError> {
        // Phase 1: Tokenization
        let tokens = self.tokenizer.tokenize(input)?;
        
        // Phase 2: Parsing
        let ast = self.parser.parse(tokens)?;
        
        // Generate parse tree visualization
        let parse_tree_text = format!("Parse Tree:\n{}", self.visualizer.visualize(&ast));
        
        // Phase 3: Semantic Analysis
        let resolved_ast = self.semantic_analyzer.analyze(ast)?;
        
        // Phase 4: Evaluation
        let result = self.evaluator.evaluate(&resolved_ast)?;
        
        Ok((result, parse_tree_text))
    }
}

// Format result for display
pub fn format_result(result: Result<JArray, InterpreterError>) -> String {
    match result {
        Ok(array) => array.to_string(),
        Err(error) => format!("Error: {}", error),
    }
}