# J Custom Parser Phase 1 Implementation Strategy

## Overview
Phase 1 focuses on creating a minimal, testable custom recursive descent parser that can handle the most basic J expressions. This establishes the foundation for incremental development while maintaining full compatibility with the existing LALRPOP parser.

## Goals
- **Minimal Viable Parser**: Handle literal numbers and basic binary operations
- **Testable Implementation**: Clear success/failure criteria with existing test cases
- **Incremental Foundation**: Architecture that supports gradual feature addition
- **Zero Breaking Changes**: Existing LALRPOP functionality remains untouched

## Phase 1 Scope

### Core Features to Implement
1. **Literal Numbers**: Parse integer literals (e.g., `42`, `123`)
2. **Binary Addition**: Handle simple addition expressions (e.g., `1+2`)
3. **Basic Precedence**: Left-to-right evaluation for same precedence
4. **Error Handling**: Clear error messages for unsupported constructs

### Explicitly Out of Scope
- Monadic operations (e.g., `~3`, `-5`)
- Multiple operators (e.g., `1+2*3`)
- Arrays and vectors (e.g., `1 2 3`)
- Complex expressions with parentheses
- All advanced J features

## Technical Implementation

### 1. Parser Architecture
```rust
pub struct CustomParser {
    tokens: Vec<Token>,
    position: usize,
}

impl CustomParser {
    pub fn parse(&mut self, tokens: Vec<Token>) -> Result<JNode, ParseError> {
        self.tokens = tokens;
        self.position = 0;
        self.parse_expression()
    }
    
    fn parse_expression(&mut self) -> Result<JNode, ParseError> {
        // Implement left-to-right binary operations
    }
    
    fn parse_literal(&mut self) -> Result<JNode, ParseError> {
        // Parse integer literals
    }
}
```

### 2. Supported Grammar
```
Expression → Literal | Expression '+' Literal
Literal    → Integer
Integer    → [0-9]+
```

### 3. Test Cases for Phase 1
```rust
// Should parse successfully
"42"     → Literal(42)
"1+2"    → BinaryOp('+', Literal(1), Literal(2))
"1+2+3"  → BinaryOp('+', BinaryOp('+', Literal(1), Literal(2)), Literal(3))

// Should return clear error messages
"~3"     → "Error: Monadic operations not implemented"
"1*2"    → "Error: Multiplication not implemented"
"1 2"    → "Error: Array literals not implemented"
""       → "Error: Empty expression"
```

## Implementation Steps

### Step 1: Basic Infrastructure (30 minutes)
- Implement `CustomParser` struct with token management
- Add position tracking and basic error handling
- Create helper methods for token consumption

### Step 2: Literal Parsing (20 minutes)
- Implement `parse_literal()` for integer tokens
- Add validation for numeric literals
- Test with simple numbers: `42`, `123`, `0`

### Step 3: Binary Addition (40 minutes)
- Implement `parse_expression()` with left-associative parsing
- Handle `+` operator between literals
- Support chained additions: `1+2+3`

### Step 4: Error Handling (20 minutes)
- Add descriptive error messages for unsupported features
- Implement position tracking for error reporting
- Test error cases comprehensively

### Step 5: Integration Testing (10 minutes)
- Verify parser selection UI works with new implementation
- Test both success and failure cases through web interface
- Confirm LALRPOP parser remains unaffected

## Success Criteria

### Functional Requirements
- ✅ Parse literal integers correctly
- ✅ Parse simple addition expressions (`1+2`)
- ✅ Parse chained additions (`1+2+3`)
- ✅ Return appropriate errors for unsupported features
- ✅ Maintain existing LALRPOP functionality

### Technical Requirements
- ✅ Clean, readable code with proper error handling
- ✅ Consistent with existing codebase patterns
- ✅ Zero impact on existing parser functionality
- ✅ Comprehensive test coverage for implemented features

## Integration Points

### 1. Parser Selection Logic
```rust
let ast_result = match parser_choice {
    "custom" => {
        let mut custom_parser = CustomParser::new();
        custom_parser.parse(tokens)
    }
    _ => {
        let lalr_parser = LalrParser::new();
        lalr_parser.parse(tokens)
    }
};
```

### 2. Error Message Format
- Maintain consistency with LALRPOP error format
- Use "Custom Parse Error:" prefix for clarity
- Include position information where possible

### 3. AST Compatibility
- Generate same `JNode` structures as LALRPOP parser
- Ensure downstream components (semantic analyzer, evaluator) work unchanged
- Maintain identical tree structure for supported operations

## Risk Mitigation

### Technical Risks
- **Token Compatibility**: Ensure custom parser uses same token format as LALRPOP
- **AST Structure**: Maintain exact compatibility with existing evaluator
- **Error Handling**: Avoid panics, return proper error types

### Development Risks
- **Scope Creep**: Strictly limit to defined features
- **Testing Gaps**: Test all error conditions thoroughly
- **Integration Issues**: Verify parser switching works in all scenarios

## Future Phases Preview

### Phase 2: Monadic Operations
- Add support for `~` (negate) and `-` (negative)
- Implement proper precedence handling

### Phase 3: Multiple Operators
- Add `*`, `%`, `-` binary operations
- Implement precedence climbing algorithm

### Phase 4: Arrays and Vectors
- Support array literals (`1 2 3`)
- Implement reshape and indexing operations

## Deliverables
1. **Updated `custom_parser.rs`**: Complete Phase 1 implementation
2. **Test Results**: Documentation of all test cases passed
3. **Integration Verification**: Screenshots of web interface working
4. **Phase 2 Planning**: Brief outline of next implementation phase

## Timeline
**Total Estimated Time**: 2 hours
- Infrastructure: 30 minutes
- Literal parsing: 20 minutes  
- Binary addition: 40 minutes
- Error handling: 20 minutes
- Integration testing: 10 minutes

This Phase 1 approach ensures a solid foundation for the custom parser while maintaining the project's principle of minimal complexity and comprehensive testing.