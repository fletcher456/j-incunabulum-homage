# J Custom Parser Next Phases Strategy

## Overview
This document outlines the immediate next phases for the custom recursive descent parser, focusing on features that are currently supported by the UI and J interpreter. Dyadic operator precedence is moved to a future section since `*`, `%`, and `-` operators are not yet implemented in the interface.

## Current Status
- ✅ **Phase 0**: Parser selection UI and infrastructure
- ✅ **Phase 1**: Literals and basic addition with left-associativity  
- ✅ **Phase 2**: Monadic operations (~, -) with precedence framework
- 🔄 **Next Phases**: Focus on array operations and complex expressions

---

## Phase 3: Array Literals and Vectors
**Short Name**: "Array Literals"
**Goal**: Support basic array operations maintaining J semantics

### Scope
- **Vector Literals**: `1 2 3`, `10 20` - multi-element arrays
- **Single Elements**: Properly handle scalar vs vector distinction
- **Vector Operations**: Array arithmetic `1 2 + 3 4`, `~1 2 3`
- **Shape Awareness**: Maintain proper J array dimensions

### Current Support
The tokenizer already handles vector parsing and the evaluator supports vector operations. The custom parser needs to properly handle `Token::Vector` variants.

### Implementation
```rust
fn parse_literal(&mut self) -> Result<JNode, ParseError> {
    if self.position >= self.tokens.len() {
        return Err(ParseError::NotImplemented(
            "Error: Expected number but reached end of input".to_string()
        ));
    }
    
    match &self.tokens[self.position] {
        Token::Vector(array) => {
            // Accept all vector sizes in Phase 3
            let node = JNode::Literal(array.clone());
            self.position += 1;
            Ok(node)
        }
        Token::Verb('~') | Token::Verb('-') => {
            Err(ParseError::NotImplemented(
                "Error: Monadic operator found where literal expected".to_string()
            ))
        }
        _ => {
            Err(ParseError::NotImplemented(
                format!("Error: Expected number at position {}", self.position)
            ))
        }
    }
}
```

### Test Cases
```rust
// Vector literals
"1 2 3"   → Literal(JArray{data: [1,2,3], shape: [3]})
"10 20"   → Literal(JArray{data: [10,20], shape: [2]})
"42"      → Literal(JArray{data: [42], shape: [1]})

// Vector operations with existing operators
"1 2 + 3 4"    → AmbiguousVerb('+', Literal([1,2]), Literal([3,4]))
"~1 2 3"       → AmbiguousVerb('~', None, Literal([1,2,3]))
"1 2 + ~3 4"   → Mixed vector and monadic operations

// Error cases remain the same
"# 2 3"        → "Error: Operator '#' not implemented in Phase 3"
```

### Success Criteria
- ✅ Parse vector literals to correct JArray structures
- ✅ Vector arithmetic with `+` and monadic `~`, `-` works
- ✅ Proper error messages for unsupported operators
- ✅ All Phase 1 and Phase 2 functionality preserved

### Timeline: 1.5 hours
- Remove multi-element array restriction: 15 minutes
- Vector literal testing: 45 minutes
- Vector operation testing: 30 minutes

---

## Phase 4: Advanced J Operators
**Short Name**: "J Array Operators"
**Goal**: Add core J language operators available in the current interface

### Scope
- **Reshape Operator**: `#` for array reshaping (`2 3 # 1 2 3 4 5 6`)
- **Index Operator**: `{` for array indexing (`1 { 10 20 30`)
- **Concatenation**: `,` for joining arrays (`1 2 , 3 4`)
- **Boxing**: `<` for creating boxed arrays (`< 1 2 3`)

### Current Support
These operators are implemented in the evaluator and have calculator buttons in the UI.

### Implementation
Add new precedence level for J-specific operators:
```rust
fn parse_expression(&mut self) -> Result<JNode, ParseError> {
    // Parse left operand (could be monadic expression)
    let mut left = self.parse_j_operators()?;
    
    // Handle dyadic addition (lowest precedence)
    while self.position < self.tokens.len() {
        match &self.tokens[self.position] {
            Token::Verb('+') => {
                self.position += 1;
                let right = self.parse_j_operators()?;
                left = JNode::AmbiguousVerb('+', Some(Box::new(left)), Some(Box::new(right)));
            }
            Token::Verb(op) => {
                return Err(ParseError::NotImplemented(
                    format!("Error: Operator '{}' not implemented in Phase 4", op)
                ));
            }
            _ => break,
        }
    }
    Ok(left)
}

fn parse_j_operators(&mut self) -> Result<JNode, ParseError> {
    let mut left = self.parse_monadic()?;
    
    while self.position < self.tokens.len() {
        match &self.tokens[self.position] {
            Token::Verb('#') => {
                self.position += 1;
                let right = self.parse_monadic()?;
                left = JNode::AmbiguousVerb('#', Some(Box::new(left)), Some(Box::new(right)));
            }
            Token::Verb('{') => {
                self.position += 1;
                let right = self.parse_monadic()?;
                left = JNode::AmbiguousVerb('{', Some(Box::new(left)), Some(Box::new(right)));
            }
            Token::Verb(',') => {
                self.position += 1;
                let right = self.parse_monadic()?;
                left = JNode::AmbiguousVerb(',', Some(Box::new(left)), Some(Box::new(right)));
            }
            Token::Verb('<') => {
                // Box can be monadic or dyadic - handle both
                self.position += 1;
                let right = self.parse_monadic()?;
                left = JNode::AmbiguousVerb('<', Some(Box::new(left)), Some(Box::new(right)));
            }
            _ => break,
        }
    }
    Ok(left)
}
```

### Test Cases
```rust
// Reshape operations
"2 3 # 1 2 3 4 5 6"  → Reshape to 2x3 matrix
"3 # 5"              → Create 3-element array of 5s
"2 # 1 2"            → Reshape [1,2] to shape [2]

// Indexing operations  
"1 { 10 20 30"       → Get element at index 1
"0 2 { 1 2 3 4"      → Get elements at indices 0,2
"1 { < 1 2 3"        → Index into boxed array

// Concatenation
"1 2 , 3 4"          → Join to [1,2,3,4]
"1 , 2 3"            → Join scalar with vector

// Boxing
"< 1 2 3"            → Create boxed array
"1 < 2"              → Dyadic box operation

// Mixed operations with precedence
"~2 3 # 1 2 3 4 5 6"  → Monadic negate has higher precedence
"1 2 + 3 # 4"         → Addition has lower precedence than reshape
```

### Success Criteria
- ✅ All J operators parse with correct precedence
- ✅ Reshape creates proper array dimensions
- ✅ Indexing and concatenation work correctly
- ✅ Boxing operations function properly
- ✅ Mixed expressions with existing operators work

### Timeline: 3 hours
- Reshape operator implementation: 1 hour
- Index operator implementation: 1 hour
- Concatenation and boxing: 1 hour

---

## Phase 5: Parentheses and Complex Expressions  
**Short Name**: "Parentheses Support"
**Goal**: Complete parser with grouping and nested expression support

### Scope
- **Parentheses**: `(2+3) # 4 5` grouping support
- **Nested Expressions**: Complex combinations of all implemented operators
- **Error Recovery**: Better error messages with position tracking
- **Complete Coverage**: Handle all currently supported J operations

### Implementation
Add primary expression parsing with parentheses:
```rust
fn parse_monadic(&mut self) -> Result<JNode, ParseError> {
    // Handle monadic operators first
    if self.position < self.tokens.len() {
        match &self.tokens[self.position] {
            Token::Verb('~') => {
                self.position += 1;
                let operand = self.parse_primary()?;
                return Ok(JNode::AmbiguousVerb('~', None, Some(Box::new(operand))));
            }
            Token::Verb('-') => {
                self.position += 1;
                let operand = self.parse_primary()?;
                return Ok(JNode::AmbiguousVerb('-', None, Some(Box::new(operand))));
            }
            _ => {}
        }
    }
    
    // Fall back to primary parsing
    self.parse_primary()
}

fn parse_primary(&mut self) -> Result<JNode, ParseError> {
    if self.position >= self.tokens.len() {
        return Err(ParseError::NotImplemented(
            "Error: Expected expression but reached end of input".to_string()
        ));
    }
    
    match &self.tokens[self.position] {
        Token::LeftParen => {
            self.position += 1; // consume '('
            let expr = self.parse_expression()?;
            
            if self.position >= self.tokens.len() {
                return Err(ParseError::InvalidExpression(
                    "Error: Missing closing parenthesis".to_string()
                ));
            }
            
            match &self.tokens[self.position] {
                Token::RightParen => {
                    self.position += 1; // consume ')'
                    Ok(expr)
                }
                _ => Err(ParseError::InvalidExpression(
                    "Error: Expected closing parenthesis".to_string()
                ))
            }
        }
        Token::Vector(array) => {
            let node = JNode::Literal(array.clone());
            self.position += 1;
            Ok(node)
        }
        _ => {
            Err(ParseError::NotImplemented(
                format!("Error: Unexpected token at position {}", self.position)
            ))
        }
    }
}
```

### Test Cases
```rust
// Basic grouping
"(1 2) + 3 4"       → Grouped addition
"2 3 # (1 + 2 3)"   → Reshape with grouped expression

// Complex expressions
"~(1 2 + 3 4)"      → Monadic operation on grouped expression
"(1 2 , 3 4) # 2 4" → Concatenation then reshape

// Nested parentheses
"((1 2) + 3) # 3"   → Multiple nesting levels
"~((1 + 2) , 3)"    → Complex nested operations

// Error cases
"(1 2 + 3"          → "Missing closing parenthesis"
"1 2) + 3"          → "Unexpected closing parenthesis"
"()"                → "Expected expression but reached end"
```

### Success Criteria
- ✅ Parentheses override operator precedence correctly
- ✅ Nested expressions parse and evaluate properly
- ✅ Clear error messages for mismatched parentheses
- ✅ All existing functionality preserved

### Timeline: 2 hours
- Parentheses parsing implementation: 1 hour
- Complex expression testing: 45 minutes
- Error recovery enhancement: 15 minutes

---

## Updated Architecture

### Parser Method Hierarchy
```
parse_expression()           // Entry point - handles dyadic +
├── parse_j_operators()     // # { , < operators (J-specific precedence)
    ├── parse_monadic()         // ~ - monadic operators (highest precedence)
        └── parse_primary()         // literals, parentheses
```

### Precedence Levels (highest to lowest)
1. **Parentheses**: `()` explicit grouping
2. **Monadic**: `~`, `-` (unary operations)  
3. **J Operators**: `#`, `{`, `,`, `<`
4. **Addition**: `+` (only dyadic operator currently supported)

---

## Total Implementation Timeline: 6.5 hours

1. **Phase 3** (1.5 hours): Array Literals and Vectors
2. **Phase 4** (3 hours): Advanced J Operators  
3. **Phase 5** (2 hours): Parentheses and Complex Expressions

---

## Future Development

### Phase 6: Dyadic Operator Precedence
**Short Name**: "Dyadic Operator Precedence"
**Goal**: Add remaining arithmetic operators when UI support is ready

#### Scope (Future)
- **Binary Operators**: `*` (multiply), `%` (divide), `-` (subtract)
- **Precedence Rules**: `*` and `%` higher than `+` and `-`  
- **UI Integration**: Add calculator buttons for these operators
- **Complex Arithmetic**: Support expressions like `2*3+4`, `10%2-1`

#### Prerequisites
- Calculator button implementation for `*`, `%`, `-`
- Evaluator support verification for these operations
- Decision on operator precedence in J context

#### Estimated Timeline: 3 hours
- UI button implementation
- Precedence level restructuring  
- Comprehensive arithmetic testing

### Additional Future Considerations
- **Advanced J Features**: More complex J operations as they become available
- **Performance Optimization**: Parser performance tuning for complex expressions
- **Error Recovery**: Enhanced error messages and recovery strategies
- **WASM Migration**: Complete transition away from LALRPOP dependency

---

## Success Metrics

### Immediate Phases (3-5)
- Complete feature parity with current LALRPOP parser for implemented operations
- All array operations working correctly through custom parser
- Parentheses providing proper precedence override
- Zero regression in existing Phase 1-2 functionality

### Long-term Goals
- Foundation ready for dyadic operator precedence when UI is ready
- Parser architecture supporting easy addition of new J operators
- Performance suitable for complex J expressions
- Clear path to WASM deployment without LALRPOP dependency