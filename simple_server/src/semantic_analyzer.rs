// J Semantic Analyzer Module - Enhanced for All Operators
// Context resolution and semantic validation for J expressions

use crate::parser::JNode;
use crate::j_array::ArrayError;
use std::fmt;

// Semantic analysis errors
#[derive(Debug, Clone)]
pub enum SemanticError {
    AmbiguousVerbContext(char, String),
    InvalidVerbUsage(char, String),
    UnresolvedAmbiguity(String),
    ArrayError(ArrayError),
}

impl From<ArrayError> for SemanticError {
    fn from(err: ArrayError) -> Self {
        SemanticError::ArrayError(err)
    }
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
            SemanticError::ArrayError(err) => {
                write!(f, "Array error: {}", err)
            }
        }
    }
}

impl std::error::Error for SemanticError {}

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
                        let resolved_right = self.resolve_context(*right)?;
                        self.validate_monadic_verb(verb)?;
                        Ok(JNode::MonadicVerb(verb, Box::new(resolved_right)))
                    }
                    (Some(left), Some(right)) => {
                        // Verb between expressions - dyadic
                        let resolved_left = self.resolve_context(*left)?;
                        let resolved_right = self.resolve_context(*right)?;
                        self.validate_dyadic_verb(verb)?;
                        Ok(JNode::DyadicVerb(
                            verb,
                            Box::new(resolved_left),
                            Box::new(resolved_right)
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
                let resolved_arg = self.resolve_context(*arg)?;
                self.validate_monadic_verb(verb)?;
                Ok(JNode::MonadicVerb(verb, Box::new(resolved_arg)))
            }
            JNode::DyadicVerb(verb, left, right) => {
                let resolved_left = self.resolve_context(*left)?;
                let resolved_right = self.resolve_context(*right)?;
                self.validate_dyadic_verb(verb)?;
                Ok(JNode::DyadicVerb(
                    verb,
                    Box::new(resolved_left),
                    Box::new(resolved_right)
                ))
            }
        }
    }

    // Validate that a verb can be used monadically
    fn validate_monadic_verb(&self, verb: char) -> Result<(), SemanticError> {
        match verb {
            '+' => Ok(()), // Identity
            '~' => Ok(()), // Iota
            '#' => Ok(()), // Tally
            ',' => Ok(()), // Ravel
            '<' => Ok(()), // Box
            '{' => Err(SemanticError::InvalidVerbUsage(
                '{', 
                "Monadic { (catalog) not supported in this implementation".to_string()
            )),
            _ => Err(SemanticError::InvalidVerbUsage(
                verb, 
                "Unknown monadic verb".to_string()
            )),
        }
    }

    // Validate that a verb can be used dyadically
    fn validate_dyadic_verb(&self, verb: char) -> Result<(), SemanticError> {
        match verb {
            '+' => Ok(()), // Plus
            '#' => Ok(()), // Reshape
            '{' => Ok(()), // From/Index
            ',' => Ok(()), // Concatenate
            '<' => Ok(()), // Less than
            '~' => Err(SemanticError::InvalidVerbUsage(
                '~', 
                "Dyadic ~ not supported in this implementation".to_string()
            )),
            _ => Err(SemanticError::InvalidVerbUsage(
                verb, 
                "Unknown dyadic verb".to_string()
            )),
        }
    }
}