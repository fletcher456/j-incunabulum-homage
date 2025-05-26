// J Semantic Analyzer Module
// Context resolution and semantic validation for J expressions

use crate::parser::JNode;
use std::fmt;

// Semantic analysis errors
#[derive(Debug, Clone)]
pub enum SemanticError {
    AmbiguousVerbContext(char, String),
    InvalidVerbUsage(char, String),
    UnresolvedAmbiguity(String),
}

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SemanticError::AmbiguousVerbContext(verb, msg) => {
                write!(f, "Ambiguous context for verb '{}': {}", verb, msg)
            }
            SemanticError::InvalidVerbUsage(verb, msg) => {
                write!(f, "Invalid usage of verb '{}': {}", verb, msg)
            }
            SemanticError::UnresolvedAmbiguity(msg) => {
                write!(f, "Unresolved ambiguity: {}", msg)
            }
        }
    }
}

// J Semantic Analyzer
pub struct JSemanticAnalyzer;

impl JSemanticAnalyzer {
    pub fn new() -> Self {
        JSemanticAnalyzer
    }

    // Analyze and resolve context for the AST
    pub fn analyze(&self, ast: JNode) -> Result<JNode, SemanticError> {
        self.resolve_context(ast)
    }

    // Context resolution - convert ambiguous verbs to monadic/dyadic
    fn resolve_context(&self, node: JNode) -> Result<JNode, SemanticError> {
        match node {
            JNode::AmbiguousVerb(verb, left, right) => {
                match (left, right) {
                    (None, Some(right)) => {
                        // Leading verb - monadic
                        Ok(JNode::MonadicVerb(verb, Box::new(self.resolve_context(*right)?)))
                    }
                    (Some(left), Some(right)) => {
                        // Verb between expressions - dyadic
                        Ok(JNode::DyadicVerb(
                            verb,
                            Box::new(self.resolve_context(*left)?),
                            Box::new(self.resolve_context(*right)?)
                        ))
                    }
                    (Some(_), None) => {
                        Err(SemanticError::InvalidVerbUsage(
                            verb, 
                            "Verb cannot have only left operand".to_string()
                        ))
                    }
                    (None, None) => {
                        Err(SemanticError::InvalidVerbUsage(
                            verb, 
                            "Verb must have at least one operand".to_string()
                        ))
                    }
                }
            }
            JNode::Literal(array) => Ok(JNode::Literal(array)),
            JNode::MonadicVerb(verb, arg) => {
                Ok(JNode::MonadicVerb(verb, Box::new(self.resolve_context(*arg)?)))
            }
            JNode::DyadicVerb(verb, left, right) => {
                Ok(JNode::DyadicVerb(
                    verb,
                    Box::new(self.resolve_context(*left)?),
                    Box::new(self.resolve_context(*right)?)
                ))
            }
        }
    }
}