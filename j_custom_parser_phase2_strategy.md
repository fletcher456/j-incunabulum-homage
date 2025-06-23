# J Custom Parser Phase 2 Implementation Strategy

## Overview
Phase 2 builds on the successful Phase 1 foundation by adding monadic operations support. This maintains the incremental development approach while introducing proper precedence handling for the first time.

## Goals
- **Monadic Operations**: Support `~` (negate) and `-` (negative) operators
- **Precedence Handling**: Implement basic operator precedence rules
- **Minimal Scope**: Focus only on monadic operations with literals and addition
- **Foundation for Phase 3**: Establish precedence architecture for multiple operators

## Phase 2 Scope

### Core Features to Implement
1. **Monadic Negate** (`~`): Handle expressions like `~3`, `~42`
2. **Monadic Negative** (`-`): Handle expressions like `-5`, `-10`
3. **Mixed Expressions**: Support `~3+2`, `-5+10`, `1+~2`
4. **Basic Precedence**: Monadic operations bind tighter than dyadic operations

### Explicitly Out of Scope
- Multiple dyadic operators (e.g., `1+2*3`)
- Parentheses and grouping
- Complex precedence beyond monadic vs dyadic
- Arrays and vectors
- All other J operators

## Technical Implementation

### 1. Enhanced Parser Architecture
```rust
impl CustomParser {
    fn parse_expression(&mut self) -> Result<JNode, ParseError> {
        // Parse left operand (could be monadic expression)
        let mut left = self.parse_monadic()?;
        
        // Handle dyadic operations
        while self.position < self.tokens.len() {
            if let Token::Verb('+') = &self.tokens[self.position] {
                self.position += 1;
                let right = self.parse_monadic()?; // Right operand can also be monadic
                left = JNode::AmbiguousVerb('+', Some(Box::new(left)), Some(Box::new(right)));
            } else {
                break;
            }
        }
        
        Ok(left)
    }
    
    fn parse_monadic(&mut self) -> Result<JNode, ParseError> {
        // Handle monadic operators
        if self.position < self.tokens.len() {
            match &self.tokens[self.position] {
                Token::Verb('~') => {
                    self.position += 1;
                    let operand = self.parse_literal()?;
                    return Ok(JNode::AmbiguousVerb('~', None, Some(Box::new(operand))));
                }
                Token::Verb('-') => {
                    self.position += 1;
                    let operand = self.parse_literal()?;
                    return Ok(JNode::AmbiguousVerb('-', None, Some(Box::new(operand))));
                }
                _ => {}
            }
        }
        
        // Fall back to literal parsing
        self.parse_literal()
    }
    
    fn parse_literal(&mut self) -> Result<JNode, ParseError> {
        // Existing literal parsing logic
    }
}
```

### 2. Supported Grammar (Extended)
```
Expression → Monadic | Expression '+' Monadic
Monadic    → '~' Literal | '-' Literal | Literal
Literal    → Integer
Integer    → [0-9]+
```

### 3. Precedence Rules
- **Monadic operators** (`~`, `-`) have higher precedence than dyadic operators
- **Left-to-right** evaluation for operators of same precedence
- **Explicit precedence**: `~3+2` = `(~3)+2`, not `~(3+2)`

## Test Cases for Phase 2

### Monadic Operations
```rust
// Should parse successfully
"~3"     → AmbiguousVerb('~', None, Literal(3))
"-5"     → AmbiguousVerb('-', None, Literal(5))
"~42"    → AmbiguousVerb('~', None, Literal(42))

// Should evaluate correctly
"~3"     → "-3" (negate: 0 1 2)
"-5"     → "-5" (negative literal)
```

### Mixed Expressions with Precedence
```rust
// Precedence testing
"~3+2"   → AmbiguousVerb('+', AmbiguousVerb('~', None, Literal(3)), Literal(2))
"1+~2"   → AmbiguousVerb('+', Literal(1), AmbiguousVerb('~', None, Literal(2)))
"-5+10"  → AmbiguousVerb('+', AmbiguousVerb('-', None, Literal(5)), Literal(10))
"~1+~2"  → AmbiguousVerb('+', AmbiguousVerb('~', None, Literal(1)), AmbiguousVerb('~', None, Literal(2)))

// Complex chaining
"~1+2+3" → Left-associative: ((~1)+2)+3
```

### Error Cases
```rust
// Should return clear error messages
"*3"     → "Error: Operator '*' not implemented in Phase 2"
"1*2"    → "Error: Operator '*' not implemented in Phase 2"
"1 2"    → "Error: Multi-element arrays not implemented"
"~~3"    → "Error: Nested monadic operations not implemented"
```

## Implementation Steps

### Step 1: Restructure Parser Methods (45 minutes)
- Extract `parse_monadic()` method from existing `parse_expression()`
- Modify `parse_expression()` to call `parse_monadic()` for operands
- Update precedence handling in expression parsing
- Test basic restructuring with existing Phase 1 functionality

### Step 2: Implement Monadic Negate (25 minutes)
- Add `~` operator support in `parse_monadic()`
- Create test cases for simple negate operations
- Verify AST structure matches LALRPOP parser output
- Test evaluation through existing pipeline

### Step 3: Implement Monadic Negative (15 minutes)
- Add `-` operator support in `parse_monadic()`
- Handle disambiguation between negative literals and subtraction
- Test basic negative operations
- Verify semantic analyzer compatibility

### Step 4: Mixed Expression Support (30 minutes)
- Test `~3+2`, `1+~2` type expressions
- Verify correct precedence: monadic binds tighter than dyadic
- Test chained operations: `~1+2+3`
- Debug any precedence issues

### Step 5: Error Handling Enhancement (15 minutes)
- Add specific error messages for unsupported operators
- Test edge cases and malformed expressions
- Verify error consistency with Phase 1 patterns
- Test position tracking in error messages

### Step 6: Integration Testing (10 minutes)
- Test all Phase 1 functionality still works
- Verify parser selection UI with new features
- Test both success and error cases through web interface
- Confirm LALRPOP parser remains unaffected

## Success Criteria

### Functional Requirements
- ✅ Parse monadic negate operations (`~3`)
- ✅ Parse monadic negative operations (`-5`)
- ✅ Handle mixed expressions with correct precedence (`~3+2`)
- ✅ Support chained operations (`~1+2+3`)
- ✅ Maintain all Phase 1 functionality
- ✅ Return appropriate errors for unsupported features

### Technical Requirements
- ✅ Proper precedence handling (monadic > dyadic)
- ✅ AST compatibility with existing semantic analyzer
- ✅ Clean separation of parsing concerns
- ✅ Comprehensive error handling

## Architecture Enhancements

### 1. Precedence Framework
The Phase 2 implementation establishes a precedence framework that will scale to Phase 3:
```rust
// Precedence levels (higher number = higher precedence)
// Level 3: Monadic operators (~, -)
// Level 2: Dyadic operators (+, -, *, %)
// Level 1: Assignment and other low-precedence operations
```

### 2. Parser Method Hierarchy
```
parse_expression()     // Handles lowest precedence (dyadic +)
├── parse_monadic()    // Handles highest precedence (monadic ~, -)
    └── parse_literal() // Handles terminals (numbers)
```

### 3. Error Message Patterns
Maintain consistent error format:
- Phase context: "Error: [feature] not implemented in Phase 2"
- Position tracking: Include position information where possible
- Clear guidance: Suggest LALRPOP parser for advanced features

## Risk Mitigation

### Technical Risks
- **Precedence Conflicts**: Extensive testing of mixed expressions
- **AST Compatibility**: Verify semantic analyzer handles new structures
- **Regression Risk**: Comprehensive testing of Phase 1 features

### Development Risks
- **Scope Creep**: Strictly limit to `~` and `-` operators only
- **Complexity Growth**: Keep parser methods focused and testable
- **Integration Issues**: Test parser switching thoroughly

## Future Phase Integration

### Phase 3 Preparation
- **Operator Framework**: Phase 2 establishes pattern for adding operators
- **Precedence Architecture**: Framework ready for multiple precedence levels
- **Testing Patterns**: Established comprehensive testing approach

### Architectural Benefits
- **Modular Design**: Each precedence level in separate method
- **Extensible Pattern**: Easy to add new operators at appropriate precedence
- **Clean Separation**: Distinct handling of monadic vs dyadic operations

## Deliverables
1. **Updated `custom_parser.rs`**: Complete Phase 2 implementation
2. **Test Results**: Documentation of all test cases passed
3. **Precedence Verification**: Comparison with LALRPOP parser behavior
4. **Integration Screenshots**: Web interface testing results
5. **Phase 3 Planning**: Brief outline of multi-operator implementation

## Timeline
**Total Estimated Time**: 2.5 hours
- Parser restructuring: 45 minutes
- Monadic negate: 25 minutes
- Monadic negative: 15 minutes
- Mixed expressions: 30 minutes
- Error handling: 15 minutes
- Integration testing: 10 minutes

This Phase 2 approach maintains the project's principles of minimal complexity and incremental development while establishing the architectural foundation needed for Phase 3's multi-operator support.