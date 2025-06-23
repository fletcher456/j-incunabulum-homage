# Custom Recursive Descent Parser Strategy for J Language

## Executive Summary

This document outlines a comprehensive strategy for replacing LALRPOP with a custom recursive descent parser to achieve complete WASM compilation compatibility. The approach prioritizes feature completeness, maintainability, and architectural soundness while eliminating all LALRPOP dependencies.

## Strategic Rationale

### Why Replace LALRPOP
1. **WASM Compilation Bottleneck**: LALRPOP consistently times out during WASM builds (60+ minutes)
2. **Dependency Chain Issues**: Cargo's dependency resolution includes LALRPOP regardless of conditional compilation
3. **Toolchain Limitations**: Fundamental incompatibility with WASM build environments
4. **Build Performance**: 80+ transitive dependencies from LALRPOP ecosystem

### Benefits of Custom Parser
1. **Zero External Dependencies**: Pure Rust implementation using only std library
2. **WASM Compatibility**: No compilation bottlenecks or toolchain conflicts
3. **Tailored Performance**: Optimized specifically for J language constructs
4. **Complete Control**: Full customization of error handling, recovery, and optimization
5. **Future-Proof**: No reliance on external parser generator evolution

## Current J Language Grammar Analysis

### Operator Precedence (Lowest to Highest)
```
1. Assignment (=:)
2. Composition (∘, @)
3. Conjunction (&, |)
4. Relational (=, ≠, <, ≤, >, ≥)
5. Arithmetic (+, -, *, %, ^)
6. Reshape (#)
7. Indexing ({)
8. Boxing (<)
9. Concatenation (,)
10. Atomic values (numbers, arrays)
```

### Grammar Productions (EBNF)
```ebnf
JExpression ::= Assignment | Composition

Assignment ::= JExpression "=:" JExpression
             | Composition

Composition ::= Composition "∘" Conjunction
              | Composition "@" Conjunction  
              | Conjunction

Conjunction ::= Conjunction "&" Relational
              | Conjunction "|" Relational
              | Relational

Relational ::= Relational "=" Arithmetic
             | Relational "≠" Arithmetic
             | Relational "<" Arithmetic
             | Relational "≤" Arithmetic
             | Relational ">" Arithmetic
             | Relational "≥" Arithmetic
             | Arithmetic

Arithmetic ::= Arithmetic "+" Reshape
             | Arithmetic "-" Reshape
             | Arithmetic "*" Reshape
             | Arithmetic "%" Reshape
             | Arithmetic "^" Reshape
             | Reshape

Reshape ::= Reshape "#" Indexing
          | Indexing

Indexing ::= Indexing "{" JExpression "}"
           | Boxing

Boxing ::= "<" Concatenation
         | Concatenation

Concatenation ::= Concatenation "," Atomic
                | Atomic

Atomic ::= Number
         | Array
         | "(" JExpression ")"
         | MonadicOp Atomic
         | DyadicOp

MonadicOp ::= "+" | "-" | "*" | "%" | "#" | "{" | "<" | "," | "~"

DyadicOp ::= "+" | "-" | "*" | "%" | "#" | "{" | "<" | ","

Number ::= INTEGER | FLOAT

Array ::= "[" (JExpression ("," JExpression)*)? "]"

INTEGER ::= [0-9]+
FLOAT ::= [0-9]+ "." [0-9]+
```

## Architecture Design

### Core Components

#### 1. Enhanced Tokenizer (`tokenizer.rs`)
```rust
pub struct JTokenizer {
    input: String,
    position: usize,
    line: usize,
    column: usize,
    lookahead: Option<Token>,
}

impl JTokenizer {
    // Position tracking for error reporting
    pub fn current_position(&self) -> Position
    pub fn peek(&mut self) -> Option<&Token>
    pub fn advance(&mut self) -> Option<Token>
    pub fn expect(&mut self, expected: TokenType) -> Result<Token, ParseError>
    
    // Enhanced error recovery
    pub fn skip_to_delimiter(&mut self) -> Vec<Token>
    pub fn synchronize(&mut self) -> Result<(), ParseError>
}
```

#### 2. Recursive Descent Parser (`recursive_parser.rs`)
```rust
pub struct JRecursiveParser {
    tokenizer: JTokenizer,
    current_token: Option<Token>,
    error_recovery: ErrorRecovery,
    context_stack: Vec<ParseContext>,
}

impl JRecursiveParser {
    // Core parsing methods
    pub fn parse(&mut self) -> Result<JNode, ParseError>
    
    // Expression parsing (precedence climbing)
    fn parse_expression(&mut self, min_precedence: u8) -> Result<JNode, ParseError>
    fn parse_assignment(&mut self) -> Result<JNode, ParseError>
    fn parse_composition(&mut self) -> Result<JNode, ParseError>
    fn parse_conjunction(&mut self) -> Result<JNode, ParseError>
    fn parse_relational(&mut self) -> Result<JNode, ParseError>
    fn parse_arithmetic(&mut self) -> Result<JNode, ParseError>
    fn parse_reshape(&mut self) -> Result<JNode, ParseError>
    fn parse_indexing(&mut self) -> Result<JNode, ParseError>
    fn parse_boxing(&mut self) -> Result<JNode, ParseError>
    fn parse_concatenation(&mut self) -> Result<JNode, ParseError>
    fn parse_atomic(&mut self) -> Result<JNode, ParseError>
    
    // Operator handling
    fn parse_monadic_op(&mut self) -> Result<JNode, ParseError>
    fn parse_dyadic_op(&mut self, left: JNode) -> Result<JNode, ParseError>
    
    // Utility methods
    fn advance(&mut self) -> Result<Token, ParseError>
    fn peek(&self) -> Option<&Token>
    fn expect(&mut self, token_type: TokenType) -> Result<Token, ParseError>
    fn match_token(&mut self, token_type: TokenType) -> bool
    
    // Error handling
    fn error(&self, message: String) -> ParseError
    fn synchronize(&mut self) -> Result<(), ParseError>
}
```

#### 3. Operator Precedence Engine
```rust
pub struct PrecedenceEngine {
    precedence_table: HashMap<TokenType, u8>,
    associativity_table: HashMap<TokenType, Associativity>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Associativity {
    Left,
    Right,
    None,
}

impl PrecedenceEngine {
    pub fn get_precedence(&self, token: &Token) -> u8
    pub fn get_associativity(&self, token: &Token) -> Associativity
    pub fn should_continue(&self, current_op: &Token, next_op: &Token) -> bool
}
```

#### 4. Context-Sensitive Disambiguation
```rust
pub struct DisambiguationEngine {
    context_stack: Vec<ParseContext>,
    operator_context: HashMap<char, OperatorContext>,
}

#[derive(Clone, Debug)]
pub enum ParseContext {
    Expression,
    MonadicPosition,
    DyadicPosition,
    ArrayLiteral,
    FunctionCall,
}

#[derive(Clone, Debug)]
pub struct OperatorContext {
    pub monadic_form: Option<MonadicOperator>,
    pub dyadic_form: Option<DyadicOperator>,
    pub precedence_monadic: u8,
    pub precedence_dyadic: u8,
}
```

#### 5. Enhanced Error Handling
```rust
#[derive(Debug, Clone)]
pub enum ParseError {
    UnexpectedToken {
        expected: Vec<TokenType>,
        found: Token,
        position: Position,
        context: String,
    },
    UnexpectedEndOfInput {
        expected: Vec<TokenType>,
        position: Position,
    },
    InvalidOperatorUsage {
        operator: char,
        context: String,
        position: Position,
        suggestion: Option<String>,
    },
    AmbiguousExpression {
        expression: String,
        position: Position,
        alternatives: Vec<String>,
    },
    MalformedArray {
        issue: String,
        position: Position,
    },
    RecursionLimit {
        position: Position,
    },
}

pub struct ErrorRecovery {
    max_errors: usize,
    current_errors: usize,
    synchronization_tokens: HashSet<TokenType>,
}
```

#### 6. AST Enhancement
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum JNode {
    // Enhanced with position information
    Literal(JArray, Position),
    MonadicOp(char, Box<JNode>, Position),
    DyadicOp(char, Box<JNode>, Box<JNode>, Position),
    Array(Vec<JNode>, Position),
    Parenthesized(Box<JNode>, Position),
    Assignment(String, Box<JNode>, Position),
    Composition(Box<JNode>, Box<JNode>, Position),
    
    // New node types for enhanced functionality
    Conjunction(Box<JNode>, Box<JNode>, Position),
    Relational(RelationalOp, Box<JNode>, Box<JNode>, Position),
    Boxing(Box<JNode>, Position),
    Indexing(Box<JNode>, Box<JNode>, Position),
}

#[derive(Debug, Clone, PartialEq)]
pub enum RelationalOp {
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}
```

## Implementation Strategy

### Phase 1: Foundation (Week 1-2)
**Objective**: Build core parsing infrastructure

#### 1.1 Enhanced Tokenizer
- Position tracking for error reporting
- Lookahead capability for disambiguation
- Error recovery at token level
- Unicode support for J operators

#### 1.2 Basic Recursive Descent Parser
- Simple expression parsing
- Basic operator precedence
- Fundamental error handling
- AST construction

#### 1.3 Testing Framework
- Unit tests for each grammar production
- Error case testing
- Performance benchmarks
- Regression test suite

### Phase 2: Core Functionality (Week 3-4)
**Objective**: Implement complete J language parsing

#### 2.1 Operator Precedence Engine
- Precedence climbing algorithm
- Associativity handling
- Context-sensitive precedence
- Performance optimization

#### 2.2 Disambiguation Engine
- Monadic/dyadic operator resolution
- Context stack management
- Ambiguity detection and resolution
- Error messaging for ambiguous cases

#### 2.3 Array Parsing
- Multi-dimensional array literals
- Nested array support
- Type inference for array elements
- Memory-efficient representation

### Phase 3: Advanced Features (Week 5-6)
**Objective**: Advanced parsing features and optimization

#### 3.1 Error Recovery
- Panic mode recovery
- Phrase-level recovery
- Error synchronization
- Multiple error reporting

#### 3.2 Performance Optimization
- Memoization for recursive productions
- Tail call optimization
- Memory pool allocation
- Benchmark-driven optimization

#### 3.3 Enhanced Error Messages
- Context-aware error reporting
- Suggestion system
- Color-coded error output
- Position highlighting

### Phase 4: Integration and Testing (Week 7-8)
**Objective**: Full system integration and validation

#### 4.1 Integration with Existing Components
- Replace LALRPOP parser in interpreter
- Maintain compatibility with semantic analyzer
- Update evaluator integration
- Preserve web interface functionality

#### 4.2 Comprehensive Testing
- Full J language test suite
- Performance regression testing
- WASM compilation verification
- Cross-platform compatibility testing

#### 4.3 Documentation and Maintenance
- API documentation
- Grammar specification
- Performance characteristics
- Maintenance guidelines

## Detailed Implementation Plan

### Tokenizer Enhancement

#### Current Issues to Address
1. Limited error recovery
2. No position tracking
3. Basic operator recognition
4. No lookahead capability

#### Proposed Improvements
```rust
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

pub struct EnhancedTokenizer {
    input: String,
    chars: Peekable<Enumerate<Chars>>,
    current_position: Position,
    lookahead_buffer: VecDeque<Token>,
    error_recovery: bool,
}

impl EnhancedTokenizer {
    pub fn new(input: String) -> Self {
        Self {
            chars: input.chars().enumerate().peekable(),
            input,
            current_position: Position { line: 1, column: 1, offset: 0 },
            lookahead_buffer: VecDeque::new(),
            error_recovery: false,
        }
    }
    
    pub fn peek_ahead(&mut self, n: usize) -> Option<&Token> {
        while self.lookahead_buffer.len() <= n {
            if let Some(token) = self.next_token() {
                self.lookahead_buffer.push_back(token);
            } else {
                break;
            }
        }
        self.lookahead_buffer.get(n)
    }
    
    pub fn advance(&mut self) -> Option<Token> {
        if let Some(token) = self.lookahead_buffer.pop_front() {
            Some(token)
        } else {
            self.next_token()
        }
    }
    
    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        
        let start_pos = self.current_position;
        
        match self.chars.peek() {
            Some(&(_, '0'..='9')) => self.read_number(start_pos),
            Some(&(_, '[')) => self.read_array_start(start_pos),
            Some(&(_, ']')) => self.read_array_end(start_pos),
            Some(&(_, '(')) => self.read_lparen(start_pos),
            Some(&(_, ')')) => self.read_rparen(start_pos),
            Some(&(_, '+')) => self.read_plus(start_pos),
            Some(&(_, '-')) => self.read_minus(start_pos),
            Some(&(_, '*')) => self.read_multiply(start_pos),
            Some(&(_, '%')) => self.read_divide(start_pos),
            Some(&(_, '#')) => self.read_reshape(start_pos),
            Some(&(_, '{')) => self.read_index(start_pos),
            Some(&(_, '<')) => self.read_box(start_pos),
            Some(&(_, ',')) => self.read_comma(start_pos),
            Some(&(_, '~')) => self.read_tilde(start_pos),
            None => None,
            Some(&(_, ch)) => {
                self.advance_char();
                Some(Token::new(TokenType::Unknown(ch), start_pos))
            }
        }
    }
    
    fn read_number(&mut self, start: Position) -> Option<Token> {
        let mut value = String::new();
        let mut is_float = false;
        
        while let Some(&(_, ch)) = self.chars.peek() {
            match ch {
                '0'..='9' => {
                    value.push(ch);
                    self.advance_char();
                }
                '.' if !is_float => {
                    is_float = true;
                    value.push(ch);
                    self.advance_char();
                }
                _ => break,
            }
        }
        
        if is_float {
            value.parse::<f64>()
                .map(|f| Token::new(TokenType::Float(f), start))
                .ok()
        } else {
            value.parse::<i32>()
                .map(|i| Token::new(TokenType::Integer(i), start))
                .ok()
        }
    }
    
    fn advance_char(&mut self) {
        if let Some((offset, ch)) = self.chars.next() {
            self.current_position.offset = offset;
            if ch == '\n' {
                self.current_position.line += 1;
                self.current_position.column = 1;
            } else {
                self.current_position.column += 1;
            }
        }
    }
}
```

### Recursive Descent Parser Implementation

#### Core Parser Structure
```rust
pub struct JRecursiveParser {
    tokenizer: EnhancedTokenizer,
    current_token: Option<Token>,
    precedence_engine: PrecedenceEngine,
    disambiguation_engine: DisambiguationEngine,
    error_recovery: ErrorRecovery,
    recursion_depth: usize,
    max_recursion_depth: usize,
}

impl JRecursiveParser {
    pub fn new(input: String) -> Self {
        let mut tokenizer = EnhancedTokenizer::new(input);
        let current_token = tokenizer.advance();
        
        Self {
            tokenizer,
            current_token,
            precedence_engine: PrecedenceEngine::new(),
            disambiguation_engine: DisambiguationEngine::new(),
            error_recovery: ErrorRecovery::new(),
            recursion_depth: 0,
            max_recursion_depth: 1000,
        }
    }
    
    pub fn parse(&mut self) -> Result<JNode, ParseError> {
        self.parse_expression(0)
    }
    
    fn parse_expression(&mut self, min_precedence: u8) -> Result<JNode, ParseError> {
        self.check_recursion_limit()?;
        self.recursion_depth += 1;
        
        let result = self.parse_expression_impl(min_precedence);
        
        self.recursion_depth -= 1;
        result
    }
    
    fn parse_expression_impl(&mut self, min_precedence: u8) -> Result<JNode, ParseError> {
        let mut left = self.parse_atomic()?;
        
        while let Some(token) = &self.current_token {
            let precedence = self.precedence_engine.get_precedence(token);
            
            if precedence < min_precedence {
                break;
            }
            
            let op_token = self.advance()?;
            let associativity = self.precedence_engine.get_associativity(&op_token);
            
            let next_min_precedence = match associativity {
                Associativity::Left => precedence + 1,
                Associativity::Right => precedence,
                Associativity::None => precedence + 1,
            };
            
            let right = self.parse_expression(next_min_precedence)?;
            left = self.create_binary_node(op_token, left, right)?;
        }
        
        Ok(left)
    }
    
    fn parse_atomic(&mut self) -> Result<JNode, ParseError> {
        match &self.current_token {
            Some(Token { token_type: TokenType::Integer(i), position }) => {
                let value = *i;
                let pos = *position;
                self.advance()?;
                Ok(JNode::Literal(JArray::scalar(value), pos))
            }
            Some(Token { token_type: TokenType::Float(f), position }) => {
                let value = *f;
                let pos = *position;
                self.advance()?;
                Ok(JNode::Literal(JArray::scalar_float(value), pos))
            }
            Some(Token { token_type: TokenType::LParen, position }) => {
                let pos = *position;
                self.advance()?; // consume '('
                let expr = self.parse_expression(0)?;
                self.expect(TokenType::RParen)?;
                Ok(JNode::Parenthesized(Box::new(expr), pos))
            }
            Some(Token { token_type: TokenType::LBracket, position }) => {
                let pos = *position;
                self.parse_array_literal(pos)
            }
            Some(token) if self.is_monadic_operator(token) => {
                self.parse_monadic_expression()
            }
            Some(token) => Err(ParseError::UnexpectedToken {
                expected: vec![
                    TokenType::Integer(0),
                    TokenType::Float(0.0),
                    TokenType::LParen,
                    TokenType::LBracket,
                ],
                found: token.clone(),
                position: token.position,
                context: "Expected atomic expression".to_string(),
            }),
            None => Err(ParseError::UnexpectedEndOfInput {
                expected: vec![TokenType::Integer(0), TokenType::Float(0.0)],
                position: Position { line: 0, column: 0, offset: 0 },
            }),
        }
    }
    
    fn parse_array_literal(&mut self, start_pos: Position) -> Result<JNode, ParseError> {
        self.advance()?; // consume '['
        
        let mut elements = Vec::new();
        
        if self.match_token(TokenType::RBracket) {
            return Ok(JNode::Array(elements, start_pos));
        }
        
        loop {
            elements.push(self.parse_expression(0)?);
            
            if self.match_token(TokenType::Comma) {
                continue;
            } else if self.match_token(TokenType::RBracket) {
                break;
            } else {
                return Err(ParseError::MalformedArray {
                    issue: "Expected ',' or ']' in array literal".to_string(),
                    position: self.current_position(),
                });
            }
        }
        
        Ok(JNode::Array(elements, start_pos))
    }
    
    fn parse_monadic_expression(&mut self) -> Result<JNode, ParseError> {
        let op_token = self.advance()?;
        let operand = self.parse_atomic()?;
        
        let op_char = match op_token.token_type {
            TokenType::Plus => '+',
            TokenType::Minus => '-',
            TokenType::Multiply => '*',
            TokenType::Divide => '%',
            TokenType::Reshape => '#',
            TokenType::Index => '{',
            TokenType::Box => '<',
            TokenType::Comma => ',',
            TokenType::Tilde => '~',
            _ => return Err(ParseError::InvalidOperatorUsage {
                operator: '?',
                context: "Monadic context".to_string(),
                position: op_token.position,
                suggestion: None,
            }),
        };
        
        Ok(JNode::MonadicOp(op_char, Box::new(operand), op_token.position))
    }
    
    fn create_binary_node(&mut self, op_token: Token, left: JNode, right: JNode) -> Result<JNode, ParseError> {
        let op_char = match op_token.token_type {
            TokenType::Plus => '+',
            TokenType::Minus => '-',
            TokenType::Multiply => '*',
            TokenType::Divide => '%',
            TokenType::Reshape => '#',
            TokenType::Index => '{',
            TokenType::Box => '<',
            TokenType::Comma => ',',
            _ => return Err(ParseError::InvalidOperatorUsage {
                operator: '?',
                context: "Dyadic context".to_string(),
                position: op_token.position,
                suggestion: None,
            }),
        };
        
        Ok(JNode::DyadicOp(op_char, Box::new(left), Box::new(right), op_token.position))
    }
    
    fn advance(&mut self) -> Result<Token, ParseError> {
        if let Some(token) = self.current_token.take() {
            self.current_token = self.tokenizer.advance();
            Ok(token)
        } else {
            Err(ParseError::UnexpectedEndOfInput {
                expected: vec![],
                position: self.current_position(),
            })
        }
    }
    
    fn expect(&mut self, expected: TokenType) -> Result<Token, ParseError> {
        if let Some(token) = &self.current_token {
            if std::mem::discriminant(&token.token_type) == std::mem::discriminant(&expected) {
                self.advance()
            } else {
                Err(ParseError::UnexpectedToken {
                    expected: vec![expected],
                    found: token.clone(),
                    position: token.position,
                    context: "Expected specific token".to_string(),
                })
            }
        } else {
            Err(ParseError::UnexpectedEndOfInput {
                expected: vec![expected],
                position: self.current_position(),
            })
        }
    }
    
    fn match_token(&mut self, token_type: TokenType) -> bool {
        if let Some(token) = &self.current_token {
            if std::mem::discriminant(&token.token_type) == std::mem::discriminant(&token_type) {
                self.advance().is_ok()
            } else {
                false
            }
        } else {
            false
        }
    }
    
    fn is_monadic_operator(&self, token: &Token) -> bool {
        matches!(token.token_type, 
            TokenType::Plus | TokenType::Minus | TokenType::Multiply | 
            TokenType::Divide | TokenType::Reshape | TokenType::Index |
            TokenType::Box | TokenType::Comma | TokenType::Tilde
        )
    }
    
    fn check_recursion_limit(&self) -> Result<(), ParseError> {
        if self.recursion_depth >= self.max_recursion_depth {
            Err(ParseError::RecursionLimit {
                position: self.current_position(),
            })
        } else {
            Ok(())
        }
    }
    
    fn current_position(&self) -> Position {
        self.current_token
            .as_ref()
            .map(|t| t.position)
            .unwrap_or(Position { line: 0, column: 0, offset: 0 })
    }
}
```

### Precedence Engine Implementation

```rust
pub struct PrecedenceEngine {
    precedence_table: HashMap<TokenType, u8>,
    associativity_table: HashMap<TokenType, Associativity>,
}

impl PrecedenceEngine {
    pub fn new() -> Self {
        let mut precedence_table = HashMap::new();
        let mut associativity_table = HashMap::new();
        
        // Assignment (lowest precedence)
        precedence_table.insert(TokenType::Assign, 1);
        associativity_table.insert(TokenType::Assign, Associativity::Right);
        
        // Composition
        precedence_table.insert(TokenType::Compose, 2);
        associativity_table.insert(TokenType::Compose, Associativity::Left);
        
        // Conjunction
        precedence_table.insert(TokenType::And, 3);
        precedence_table.insert(TokenType::Or, 3);
        associativity_table.insert(TokenType::And, Associativity::Left);
        associativity_table.insert(TokenType::Or, Associativity::Left);
        
        // Relational
        precedence_table.insert(TokenType::Equal, 4);
        precedence_table.insert(TokenType::NotEqual, 4);
        precedence_table.insert(TokenType::Less, 4);
        precedence_table.insert(TokenType::LessEqual, 4);
        precedence_table.insert(TokenType::Greater, 4);
        precedence_table.insert(TokenType::GreaterEqual, 4);
        associativity_table.insert(TokenType::Equal, Associativity::Left);
        associativity_table.insert(TokenType::NotEqual, Associativity::Left);
        associativity_table.insert(TokenType::Less, Associativity::Left);
        associativity_table.insert(TokenType::LessEqual, Associativity::Left);
        associativity_table.insert(TokenType::Greater, Associativity::Left);
        associativity_table.insert(TokenType::GreaterEqual, Associativity::Left);
        
        // Arithmetic
        precedence_table.insert(TokenType::Plus, 5);
        precedence_table.insert(TokenType::Minus, 5);
        precedence_table.insert(TokenType::Multiply, 6);
        precedence_table.insert(TokenType::Divide, 6);
        associativity_table.insert(TokenType::Plus, Associativity::Left);
        associativity_table.insert(TokenType::Minus, Associativity::Left);
        associativity_table.insert(TokenType::Multiply, Associativity::Left);
        associativity_table.insert(TokenType::Divide, Associativity::Left);
        
        // Reshape
        precedence_table.insert(TokenType::Reshape, 7);
        associativity_table.insert(TokenType::Reshape, Associativity::Left);
        
        // Indexing
        precedence_table.insert(TokenType::Index, 8);
        associativity_table.insert(TokenType::Index, Associativity::Left);
        
        // Boxing
        precedence_table.insert(TokenType::Box, 9);
        associativity_table.insert(TokenType::Box, Associativity::Right);
        
        // Concatenation
        precedence_table.insert(TokenType::Comma, 10);
        associativity_table.insert(TokenType::Comma, Associativity::Left);
        
        Self {
            precedence_table,
            associativity_table,
        }
    }
    
    pub fn get_precedence(&self, token: &Token) -> u8 {
        self.precedence_table.get(&token.token_type).copied().unwrap_or(0)
    }
    
    pub fn get_associativity(&self, token: &Token) -> Associativity {
        self.associativity_table.get(&token.token_type).copied().unwrap_or(Associativity::Left)
    }
    
    pub fn should_continue(&self, current_op: &Token, next_op: &Token) -> bool {
        let current_prec = self.get_precedence(current_op);
        let next_prec = self.get_precedence(next_op);
        
        match self.get_associativity(current_op) {
            Associativity::Left => current_prec > next_prec,
            Associativity::Right => current_prec >= next_prec,
            Associativity::None => current_prec != next_prec,
        }
    }
}
```

## Testing Strategy

### Unit Testing Framework
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_arithmetic() {
        let mut parser = JRecursiveParser::new("1 + 2 * 3".to_string());
        let result = parser.parse().unwrap();
        
        match result {
            JNode::DyadicOp('+', left, right, _) => {
                assert_eq!(*left, JNode::Literal(JArray::scalar(1), Position::default()));
                match *right {
                    JNode::DyadicOp('*', left2, right2, _) => {
                        assert_eq!(*left2, JNode::Literal(JArray::scalar(2), Position::default()));
                        assert_eq!(*right2, JNode::Literal(JArray::scalar(3), Position::default()));
                    }
                    _ => panic!("Expected multiplication"),
                }
            }
            _ => panic!("Expected addition at top level"),
        }
    }
    
    #[test]
    fn test_operator_precedence() {
        let test_cases = vec![
            ("1 + 2 * 3", "1 + (2 * 3)"),
            ("1 * 2 + 3", "(1 * 2) + 3"),
            ("1 + 2 + 3", "(1 + 2) + 3"),
            ("1 - 2 - 3", "(1 - 2) - 3"),
            ("2 ^ 3 ^ 4", "2 ^ (3 ^ 4)"),
        ];
        
        for (input, expected_structure) in test_cases {
            let mut parser = JRecursiveParser::new(input.to_string());
            let result = parser.parse().unwrap();
            assert_eq!(result.to_string(), expected_structure);
        }
    }
    
    #[test]
    fn test_monadic_operators() {
        let test_cases = vec![
            ("+1", "MonadicOp('+', 1)"),
            ("-1", "MonadicOp('-', 1)"),
            ("~1", "MonadicOp('~', 1)"),
            ("+ 1 2 3", "MonadicOp('+', [1, 2, 3])"),
        ];
        
        for (input, expected) in test_cases {
            let mut parser = JRecursiveParser::new(input.to_string());
            let result = parser.parse().unwrap();
            assert_eq!(result.to_debug_string(), expected);
        }
    }
    
    #[test]
    fn test_array_literals() {
        let test_cases = vec![
            ("[]", "Array([])"),
            ("[1]", "Array([1])"),
            ("[1, 2, 3]", "Array([1, 2, 3])"),
            ("[[1, 2], [3, 4]]", "Array([Array([1, 2]), Array([3, 4])])"),
        ];
        
        for (input, expected) in test_cases {
            let mut parser = JRecursiveParser::new(input.to_string());
            let result = parser.parse().unwrap();
            assert_eq!(result.to_debug_string(), expected);
        }
    }
    
    #[test]
    fn test_error_handling() {
        let error_cases = vec![
            ("1 +", "UnexpectedEndOfInput"),
            ("1 + + 2", "UnexpectedToken"),
            ("[1, 2", "MalformedArray"),
            ("((1)", "UnexpectedToken"),
        ];
        
        for (input, expected_error) in error_cases {
            let mut parser = JRecursiveParser::new(input.to_string());
            let result = parser.parse();
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains(expected_error));
        }
    }
    
    #[test]
    fn test_position_tracking() {
        let mut parser = JRecursiveParser::new("1\n + \n2".to_string());
        let result = parser.parse().unwrap();
        
        // Verify positions are correctly tracked
        match result {
            JNode::DyadicOp('+', left, right, pos) => {
                assert_eq!(pos.line, 2);
                assert_eq!(pos.column, 2);
            }
            _ => panic!("Expected dyadic operation"),
        }
    }
}
```

### Integration Testing
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::evaluator::JEvaluator;
    use crate::j_array::JArray;
    
    #[test]
    fn test_full_pipeline() {
        let input = "1 + 2 * 3";
        let mut parser = JRecursiveParser::new(input.to_string());
        let ast = parser.parse().unwrap();
        
        let mut evaluator = JEvaluator::new();
        let result = evaluator.evaluate(&ast).unwrap();
        
        assert_eq!(result, JArray::scalar(7));
    }
    
    #[test]
    fn test_complex_expressions() {
        let test_cases = vec![
            ("2 + 3 * 4", JArray::scalar(14)),
            ("(2 + 3) * 4", JArray::scalar(20)),
            ("+ 1 2 3", JArray::from_vec(vec![1, 2, 3])),
            ("1 2 3 + 4 5 6", JArray::from_vec(vec![5, 7, 9])),
        ];
        
        for (input, expected) in test_cases {
            let mut parser = JRecursiveParser::new(input.to_string());
            let ast = parser.parse().unwrap();
            
            let mut evaluator = JEvaluator::new();
            let result = evaluator.evaluate(&ast).unwrap();
            
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }
}
```

### Performance Testing
```rust
#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;
    
    #[test]
    fn test_parsing_performance() {
        let large_expression = "1".to_string() + &" + 1".repeat(10000);
        
        let start = Instant::now();
        let mut parser = JRecursiveParser::new(large_expression);
        let result = parser.parse();
        let duration = start.elapsed();
        
        assert!(result.is_ok());
        assert!(duration.as_millis() < 1000); // Should parse in under 1 second
    }
    
    #[test]
    fn test_deep_nesting_performance() {
        let deep_expression = "(".repeat(1000) + "1" + &")".repeat(1000);
        
        let start = Instant::now();
        let mut parser = JRecursiveParser::new(deep_expression);
        let result = parser.parse();
        let duration = start.elapsed();
        
        assert!(result.is_ok());
        assert!(duration.as_millis() < 500); // Should handle deep nesting efficiently
    }
}
```

## Migration Strategy

### Phase 1: Parallel Implementation
1. Implement custom parser alongside existing LALRPOP parser
2. Use feature flags to switch between parsers during development
3. Maintain full compatibility with existing AST structure
4. Run comprehensive test suite against both parsers

### Phase 2: Gradual Replacement
1. Replace LALRPOP parser in development builds
2. Extensive testing with existing test suite
3. Performance benchmarking and optimization
4. Bug fixes and feature parity verification

### Phase 3: Complete Migration
1. Remove LALRPOP dependencies from Cargo.toml
2. Update build scripts and CI configuration
3. Remove feature flags and LALRPOP-specific code
4. Update documentation and examples

### Phase 4: Optimization and Enhancement
1. Performance optimization based on real-world usage
2. Enhanced error messages and recovery
3. Additional J language features if needed
4. Long-term maintenance and evolution

## Risk Mitigation

### Technical Risks
1. **Parsing Complexity**: Comprehensive test suite and gradual implementation
2. **Performance Regression**: Continuous benchmarking and optimization
3. **Feature Incompatibility**: Parallel implementation with comparison testing
4. **Maintenance Burden**: Clear documentation and modular design

### Project Risks
1. **Timeline Overrun**: Phased approach with working fallbacks
2. **Resource Constraints**: Focus on core functionality first
3. **Scope Creep**: Strict feature parity requirements
4. **Integration Issues**: Extensive integration testing

## Success Metrics

### Primary Metrics
1. **WASM Build Success**: 100% successful WASM compilation
2. **Build Time**: <10 minutes for WASM builds (vs current 60+ minutes)
3. **Feature Parity**: All existing J language features working
4. **Test Coverage**: >95% test coverage for parser

### Secondary Metrics
1. **Error Quality**: Improved error messages and suggestions
2. **Performance**: Parsing performance comparable to or better than LALRPOP
3. **Memory Usage**: Efficient memory usage for large expressions
4. **Maintainability**: Clean, well-documented codebase

## Long-term Vision

### Extensibility
- Plugin architecture for custom operators
- Support for additional J language features
- Configurable parsing options and behaviors
- Integration with external tools and editors

### Performance
- Incremental parsing for large files
- Parallel parsing for independent expressions
- Memory pooling and allocation optimization
- Profile-guided optimization

### Tooling
- Syntax highlighting support
- Language server protocol integration
- Debugging and profiling tools
- Documentation generation

## Conclusion

This comprehensive strategy provides a complete roadmap for replacing LALRPOP with a custom recursive descent parser. The approach prioritizes:

1. **Reliability**: Extensive testing and gradual migration
2. **Performance**: Optimized for WASM and native compilation
3. **Maintainability**: Clean architecture and comprehensive documentation
4. **Extensibility**: Designed for future enhancements and modifications

The estimated timeline of 8 weeks provides a realistic path to achieve complete LALRPOP independence while maintaining full functionality and improving build performance. The modular design ensures that each component can be developed, tested, and optimized independently, reducing project risk and enabling parallel development.

This strategy positions the J language interpreter for long-term success with complete control over the parsing pipeline, optimal WASM compatibility, and a foundation for future language enhancements.