# J Custom Parser Remaining Phases Strategy (Phase 3-6)

## Overview
This document outlines the complete roadmap for finishing the custom recursive descent parser, building on the successful Phase 1 (literals + addition) and Phase 2 (monadic operations) implementations. Each phase maintains minimal scope with comprehensive testing.

## Architecture Foundation

### Current Status
- ✅ **Phase 0**: Parser selection UI and infrastructure
- ✅ **Phase 1**: Literals and basic addition with left-associativity
- ✅ **Phase 2**: Monadic operations (~, -) with precedence framework
- 🔄 **Phase 3-6**: Complete J operator support with full precedence

### Established Patterns
- **Incremental Development**: Each phase adds minimal, testable functionality
- **AST Compatibility**: Generate identical `JNode` structures as LALRPOP
- **Error Handling**: Clear "not implemented in Phase X" messages
- **Precedence Framework**: Method hierarchy supports multiple precedence levels

---

## Phase 3: Multiple Dyadic Operators
**Goal**: Add remaining arithmetic operators with proper precedence

### Scope
- **Binary Operators**: `*` (multiply), `%` (divide), `-` (subtract)
- **Precedence Rules**: `*` and `%` higher than `+` and `-`
- **Associativity**: Left-to-right for same precedence level
- **Mixed Expressions**: Support complex expressions like `2*3+4`, `10%2-1`

### Implementation
```rust
impl CustomParser {
    fn parse_expression(&mut self) -> Result<JNode, ParseError> {
        // Parse addition/subtraction level (lowest precedence)
        self.parse_additive()
    }
    
    fn parse_additive(&mut self) -> Result<JNode, ParseError> {
        let mut left = self.parse_multiplicative()?;
        
        while self.position < self.tokens.len() {
            match &self.tokens[self.position] {
                Token::Verb('+') | Token::Verb('-') => {
                    let op = // extract operator
                    self.position += 1;
                    let right = self.parse_multiplicative()?;
                    left = JNode::AmbiguousVerb(op, Some(Box::new(left)), Some(Box::new(right)));
                }
                _ => break,
            }
        }
        Ok(left)
    }
    
    fn parse_multiplicative(&mut self) -> Result<JNode, ParseError> {
        let mut left = self.parse_monadic()?;
        
        while self.position < self.tokens.len() {
            match &self.tokens[self.position] {
                Token::Verb('*') | Token::Verb('%') => {
                    // Handle multiplication/division
                }
                _ => break,
            }
        }
        Ok(left)
    }
}
```

### Test Cases
```rust
// Basic operations
"2*3"     → AmbiguousVerb('*', Literal(2), Literal(3))
"10%2"    → AmbiguousVerb('%', Literal(10), Literal(2))
"5-3"     → AmbiguousVerb('-', Literal(5), Literal(3))

// Precedence testing
"2*3+4"   → AmbiguousVerb('+', AmbiguousVerb('*', Literal(2), Literal(3)), Literal(4))
"10+2*3"  → AmbiguousVerb('+', Literal(10), AmbiguousVerb('*', Literal(2), Literal(3)))

// Mixed with monadic
"~2*3"    → AmbiguousVerb('*', AmbiguousVerb('~', None, Literal(2)), Literal(3))
"2*~3"    → AmbiguousVerb('*', Literal(2), AmbiguousVerb('~', None, Literal(3)))
```

### Timeline: 3 hours
- Precedence restructuring: 1 hour
- Multiply/divide implementation: 45 minutes
- Subtract implementation: 30 minutes
- Mixed expression testing: 45 minutes

---

## Phase 4: Array Literals and Vectors
**Goal**: Support basic array operations maintaining J semantics

### Scope
- **Vector Literals**: `1 2 3`, `10 20`
- **Single Elements**: Distinguish `5` from `5` (scalar vs 1-element vector)
- **Basic Operations**: Vector arithmetic `1 2 + 3 4`
- **Shape Awareness**: Maintain array dimensions

### Implementation
```rust
fn parse_literal(&mut self) -> Result<JNode, ParseError> {
    match &self.tokens[self.position] {
        Token::Vector(array) => {
            let node = JNode::Literal(array.clone());
            self.position += 1;
            Ok(node)
        }
        _ => // existing error handling
    }
}
```

### Test Cases
```rust
// Vector literals
"1 2 3"   → Literal(JArray{data: [1,2,3], shape: [3]})
"10 20"   → Literal(JArray{data: [10,20], shape: [2]})

// Vector operations
"1 2 + 3 4"    → AmbiguousVerb('+', Literal([1,2]), Literal([3,4]))
"~1 2 3"       → AmbiguousVerb('~', None, Literal([1,2,3]))

// Mixed scalar/vector
"5 + 1 2"      → AmbiguousVerb('+', Literal([5]), Literal([1,2]))
```

### Timeline: 2.5 hours
- Vector tokenization integration: 45 minutes
- Array literal parsing: 1 hour
- Vector arithmetic testing: 45 minutes

---

## Phase 5: Advanced J Operators
**Goal**: Add core J language operators with proper semantics

### Scope
- **Reshape Operator**: `#` for array reshaping
- **Index Operator**: `{` for array indexing
- **Concatenation**: `,` for joining arrays
- **Boxing**: `<` for creating boxed arrays

### Implementation
```rust
fn parse_j_operators(&mut self) -> Result<JNode, ParseError> {
    let mut left = self.parse_multiplicative()?;
    
    while self.position < self.tokens.len() {
        match &self.tokens[self.position] {
            Token::Verb('#') => // reshape
            Token::Verb('{') => // index
            Token::Verb(',') => // concatenate
            Token::Verb('<') => // box
            _ => break,
        }
    }
    Ok(left)
}
```

### Test Cases
```rust
// Reshape
"2 3 # 1 2 3 4 5 6"  → Reshape 1D array to 2x3 matrix
"3 # 5"              → Create 3-element array of 5s

// Indexing
"1 { 10 20 30"       → Index 1 from array [10,20,30]
"0 2 { 1 2 3 4"      → Index positions 0,2 from array

// Concatenation
"1 2 , 3 4"          → Join arrays to [1,2,3,4]

// Boxing
"< 1 2 3"            → Create boxed array containing [1,2,3]
```

### Timeline: 4 hours
- Reshape operator: 1.5 hours
- Index operator: 1.5 hours
- Concatenation: 45 minutes
- Boxing: 15 minutes

---

## Phase 6: Parentheses and Complex Expressions
**Goal**: Complete parser with grouping and complex expression support

### Scope
- **Parentheses**: `(2+3)*4` grouping support
- **Nested Expressions**: Complex combinations of all operators
- **Error Recovery**: Better error messages with position tracking
- **Complete Feature Parity**: Match LALRPOP parser capabilities

### Implementation
```rust
fn parse_primary(&mut self) -> Result<JNode, ParseError> {
    match &self.tokens[self.position] {
        Token::LeftParen => {
            self.position += 1; // consume '('
            let expr = self.parse_expression()?;
            
            if self.position >= self.tokens.len() || 
               !matches!(&self.tokens[self.position], Token::RightParen) {
                return Err(ParseError::InvalidExpression("Missing closing parenthesis".to_string()));
            }
            
            self.position += 1; // consume ')'
            Ok(expr)
        }
        _ => self.parse_literal(),
    }
}
```

### Test Cases
```rust
// Basic grouping
"(2+3)*4"       → AmbiguousVerb('*', grouped_addition, Literal(4))
"2*(3+4)"       → AmbiguousVerb('*', Literal(2), grouped_addition)

// Complex expressions
"~(2+3)*4"      → Complex precedence with grouping
"(1 2 + 3 4) * 5"  → Vector operations with grouping

// Nested parentheses
"((2+3)*4)+1"   → Multiple nesting levels

// Error cases
"(2+3"          → "Missing closing parenthesis"
"2+3)"          → "Unexpected closing parenthesis"
```

### Timeline: 3 hours
- Parentheses parsing: 1.5 hours
- Complex expression testing: 1 hour
- Error recovery enhancement: 30 minutes

---

## Implementation Sequence

### Total Timeline: 12.5 hours across 4 phases

1. **Phase 3** (3 hours): Multiple dyadic operators with precedence
2. **Phase 4** (2.5 hours): Array literals and vector operations
3. **Phase 5** (4 hours): Advanced J operators (reshape, index, etc.)
4. **Phase 6** (3 hours): Parentheses and complex expressions

### Incremental Testing Strategy
- **After each phase**: Full regression testing of previous phases
- **Cross-parser validation**: Compare output with LALRPOP parser
- **Web interface testing**: Verify UI switching works correctly
- **Error message consistency**: Maintain clear error patterns

## Success Criteria

### Phase 3 Success
- ✅ All arithmetic operators working with correct precedence
- ✅ Mixed monadic/dyadic expressions parse correctly
- ✅ Left-associativity maintained for same-precedence operations

### Phase 4 Success
- ✅ Vector literals parse to correct JArray structures
- ✅ Vector arithmetic operations work properly
- ✅ Scalar/vector mixing handled correctly

### Phase 5 Success
- ✅ Reshape operations create correct array shapes
- ✅ Indexing returns proper array elements
- ✅ Concatenation and boxing work as expected

### Phase 6 Success
- ✅ Parentheses override precedence correctly
- ✅ Complex nested expressions parse properly
- ✅ Complete feature parity with LALRPOP parser

## Risk Mitigation

### Technical Risks
- **Precedence Complexity**: Comprehensive test matrix for operator combinations
- **Array Semantics**: Careful alignment with J language specifications
- **Performance**: Monitor parsing performance with complex expressions

### Development Risks
- **Scope Creep**: Strict adherence to phase boundaries
- **Integration Debt**: Regular testing of all previous functionality
- **Code Complexity**: Keep methods focused and well-documented

## Final Architecture

### Parser Method Hierarchy
```
parse_expression()           // Entry point
├── parse_additive()        // + - operators (lowest precedence)
    ├── parse_multiplicative()  // * % operators
        ├── parse_j_operators()     // # { , < operators
            ├── parse_monadic()         // ~ - monadic operators
                └── parse_primary()         // literals, parentheses (highest precedence)
```

### Precedence Levels (highest to lowest)
1. **Parentheses**: `()` explicit grouping
2. **Monadic**: `~`, `-` (unary operations)
3. **J Operators**: `#`, `{`, `,`, `<`
4. **Multiplicative**: `*`, `%`
5. **Additive**: `+`, `-` (binary operations)

## Post-Completion Benefits

### WASM Readiness
- **No LALRPOP Dependency**: Custom parser eliminates build complexity
- **Smaller Binary**: Reduced dependency footprint for WASM
- **Full Control**: Complete customization for J language needs

### Maintenance Advantages
- **Clear Architecture**: Well-defined precedence and parsing structure
- **Extensible Design**: Easy addition of new J operators
- **Comprehensive Testing**: Full test coverage for all language features

This strategy ensures a complete, production-ready custom parser that maintains the project's principles of minimal complexity, incremental development, and comprehensive testing.