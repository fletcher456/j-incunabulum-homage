# J Interpreter Architecture Strategy

## Overview

This document outlines the modular architecture for the J interpreter, separating concerns into distinct source files with well-defined interfaces.

## Module Structure

### 1. `tokenizer.rs` - Lexical Analysis
**Responsibility**: Convert raw J source code into tokens

**Public Interface**:
```rust
pub struct JTokenizer;

pub enum Token {
    Vector(JArray),
    Verb(char),
}

impl JTokenizer {
    pub fn new() -> Self;
    pub fn tokenize(&self, input: &str) -> Result<Vec<Token>, TokenError>;
}

pub enum TokenError {
    InvalidNumber(String),
    UnknownCharacter(char),
    InvalidVector(String),
}
```

**Key Features**:
- Recognizes space-separated numbers as vector tokens
- Handles individual scalars as single-element vectors
- Supports J verbs: `+`, `~`, `#`, `<`, `{`, `,`
- Provides detailed error reporting with position information

---

### 2. `parser.rs` - Syntactic Analysis
**Responsibility**: Convert tokens into Abstract Syntax Tree (AST)

**Public Interface**:
```rust
pub struct JParser;

#[derive(Debug, Clone)]
pub enum JNode {
    Literal(JArray),
    MonadicVerb(char, Box<JNode>),
    DyadicVerb(char, Box<JNode>, Box<JNode>),
    AmbiguousVerb(char, Option<Box<JNode>>, Option<Box<JNode>>),
}

impl JParser {
    pub fn new() -> Self;
    pub fn parse(&self, tokens: Vec<Token>) -> Result<JNode, ParseError>;
}

pub enum ParseError {
    UnexpectedToken(Token, usize),
    UnexpectedEndOfInput,
    InvalidExpression(String),
}
```

**Key Features**:
- Context-free parsing following formal EBNF grammar
- Creates ambiguous verb nodes for later resolution
- Right-recursive structure to support J's evaluation order
- Comprehensive error reporting with token positions

---

### 3. `semantic_analyzer.rs` - Context Resolution
**Responsibility**: Resolve ambiguous verbs and validate semantic correctness

**Public Interface**:
```rust
pub struct JSemanticAnalyzer;

impl JSemanticAnalyzer {
    pub fn new() -> Self;
    pub fn analyze(&self, ast: JNode) -> Result<JNode, SemanticError>;
}

pub enum SemanticError {
    AmbiguousVerbContext(char, String),
    InvalidVerbUsage(char, String),
    UnresolvedAmbiguity(String),
}
```

**Key Features**:
- Converts AmbiguousVerb nodes to MonadicVerb or DyadicVerb
- Applies J's context rules (leading verb = monadic, etc.)
- Validates semantic correctness without type checking
- Maintains AST structure while resolving ambiguities

---

### 4. `evaluator.rs` - Expression Evaluation
**Responsibility**: Execute the resolved AST and perform computations

**Public Interface**:
```rust
pub struct JEvaluator;

impl JEvaluator {
    pub fn new() -> Self;
    pub fn evaluate(&self, ast: &JNode) -> Result<JArray, EvaluationError>;
}

pub enum EvaluationError {
    UnsupportedVerb(char, String),
    DimensionMismatch(String),
    DomainError(String),
    RankError(String),
}
```

**Key Features**:
- Implements J verb semantics (iota, plus monadic/dyadic, etc.)
- Handles scalar and vector operations with proper broadcasting
- Provides meaningful error messages for runtime failures
- Extensible design for adding new J verbs

---

### 5. `j_array.rs` - Data Structures
**Responsibility**: Core J array data structure and operations

**Public Interface**:
```rust
#[derive(Debug, Clone, PartialEq)]
pub struct JArray {
    pub array_type: JType,
    pub rank: usize,
    pub shape: Vec<usize>,
    pub data: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum JType {
    Integer,
    Box,
}

impl JArray {
    pub fn new_scalar(value: i64) -> Self;
    pub fn new_integer(rank: usize, shape: Vec<usize>, data: Vec<i64>) -> Self;
    pub fn tally(&self) -> usize;
}

impl fmt::Display for JArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
```

**Key Features**:
- Unified representation for scalars and arrays
- Shape and rank information for multi-dimensional arrays
- Display formatting for user output
- Foundation for future J data types (boxes, etc.)

---

### 6. `interpreter.rs` - Main Interface
**Responsibility**: Coordinate all modules and provide unified API

**Public Interface**:
```rust
pub struct JInterpreter {
    tokenizer: JTokenizer,
    parser: JParser,
    semantic_analyzer: JSemanticAnalyzer,
    evaluator: JEvaluator,
}

impl JInterpreter {
    pub fn new() -> Self;
    pub fn execute(&self, input: &str) -> Result<JArray, InterpreterError>;
}

pub enum InterpreterError {
    TokenError(TokenError),
    ParseError(ParseError),
    SemanticError(SemanticError),
    EvaluationError(EvaluationError),
}

pub fn format_result(result: Result<JArray, InterpreterError>) -> String;
```

**Key Features**:
- Orchestrates the complete interpretation pipeline
- Unified error handling and reporting
- Clean API for external use (web server, REPL, etc.)
- Consistent result formatting

---

## Data Flow

```
Input String
     ↓
Tokenizer → Vec<Token>
     ↓
Parser → JNode (with AmbiguousVerb nodes)
     ↓
Semantic Analyzer → JNode (resolved)
     ↓
Evaluator → JArray
     ↓
Formatted Output
```

## Error Handling Strategy

Each module defines its own error types that compose into the main `InterpreterError`. This provides:
- **Precise error location**: Know exactly which phase failed
- **Detailed error context**: Module-specific error information
- **Clean error propagation**: Using `?` operator throughout
- **User-friendly messages**: Each error type implements meaningful descriptions

## Testing Strategy

Each module can be tested independently:
- **Unit tests**: Test individual functions and edge cases
- **Integration tests**: Test module interactions
- **End-to-end tests**: Test complete expressions through the interpreter

## Extension Points

- **New verbs**: Add to evaluator without touching other modules
- **New data types**: Extend JArray and update evaluator
- **New syntax**: Modify tokenizer and parser as needed
- **Optimization**: Replace individual modules without changing interfaces

## Benefits

1. **Maintainability**: Clear separation of concerns
2. **Testability**: Each module can be tested in isolation
3. **Debuggability**: Easy to trace issues to specific modules
4. **Extensibility**: Add features without major refactoring
5. **Reusability**: Modules can be used independently
6. **Code clarity**: Each file has a single, well-defined purpose