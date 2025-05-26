# J Parser Generator Strategy

## Overview

Our current J interpreter has identified a critical parsing issue: expressions like `~3+~3` are being parsed as `~(3+~3)` instead of the correct `(~3)+(~3)`. This happens because our hand-written recursive descent parser doesn't properly implement the formal EBNF grammar we designed for J's context-sensitive expressions.

## The Problem

### Current Parser Limitations
1. **Incorrect Precedence**: Our parser treats consecutive verbs incorrectly
2. **Missing EBNF Compliance**: The hand-written parser doesn't follow our formal grammar
3. **Context Resolution Gaps**: We need better separation between syntax and semantics

### Evidence from Parse Trees
```
Expression: ~3+~3
Current Parse Tree:
AmbiguousVerb: '~'
Right:
  AmbiguousVerb: '+'
  Left:
    Literal: 3
  Right:
    AmbiguousVerb: '~'
    Right:
      Literal: 3
Result: 0 1 2  (WRONG - should be 0 2 4)
```

The issue is clear: `~3+~3` should parse as two separate monadic operations joined by dyadic addition, not as a single monadic operation on the result of `3+~3`.

## Proposed Solution: Custom Parser Generator

### Why We Need a Parser Generator

1. **EBNF Compliance**: Automatically generate a parser that exactly follows our formal grammar
2. **Correctness**: Eliminate hand-coding errors in precedence and associativity
3. **Maintainability**: Changes to grammar automatically update the parser
4. **Debugging**: Clear mapping between EBNF rules and generated code

### Parser Generator Architecture

```
EBNF Grammar → Parser Generator → Context-Free Parser → Semantic Analyzer → Evaluator
```

### Implementation Strategy

#### Phase 1: Create Simple Parser Generator
Build a custom parser generator specifically for our J grammar that:
- Takes EBNF input
- Generates Rust parser code
- Creates the exact AST structure we need
- Handles our specific token types (vectors, verbs, literals)

#### Phase 2: Generate J Parser
Use our parser generator with the formal J EBNF grammar:

```ebnf
jExpression ::= expression
expression  ::= term (verb expression)?
term        ::= verb expression | atom | groupedExpr
groupedExpr ::= '(' expression ')'
atom        ::= vector | number
vector      ::= number (space+ number)*
number      ::= digit+
digit       ::= [0-9]
verb        ::= '+' | '~' | '#' | '<' | '{' | ','
space       ::= ' '
```

#### Phase 3: Integration
Replace our current hand-written parser with the generated one, maintaining our existing:
- Tokenizer (already working correctly)
- Semantic analyzer (context resolution)
- Evaluator (verb implementations)
- Visualizer (parse tree display)

## Technical Specifications

### Parser Generator Features
1. **LL(1) or LR(1) Generation**: Choose based on our grammar's complexity
2. **Error Recovery**: Generate meaningful parse error messages
3. **AST Generation**: Create our `JNode` structures directly
4. **Token Integration**: Work with our existing `Token` enum

### Generated Parser Interface
```rust
pub struct GeneratedJParser {
    // Generated parser state
}

impl GeneratedJParser {
    pub fn new() -> Self;
    pub fn parse(&self, tokens: Vec<Token>) -> Result<JNode, ParseError>;
}
```

### Integration Points
- **Input**: Our existing tokenizer output (`Vec<Token>`)
- **Output**: Our existing AST format (`JNode`)
- **Errors**: Compatible with our existing error handling

## Benefits

### Correctness
- **Grammar Compliance**: Parser exactly matches formal EBNF specification
- **Precedence Accuracy**: Automatically handles operator precedence rules
- **Associativity**: Correctly implements left/right associative rules

### Maintainability
- **Single Source of Truth**: Grammar is the definitive specification
- **Automatic Updates**: Parser changes when grammar changes
- **Clear Documentation**: EBNF serves as both spec and implementation guide

### Debugging
- **Parse Tree Accuracy**: Generated parser creates correct parse trees
- **Rule Tracing**: Can trace which grammar rules are being applied
- **Error Precision**: Better error messages with grammar rule context

## Implementation Plan

### Step 1: Design Parser Generator
Create a simple but effective parser generator that:
- Reads EBNF grammar from a file
- Generates Rust parser code
- Creates AST nodes matching our `JNode` enum
- Handles our specific token types

### Step 2: Generate J Parser
- Input our formal J grammar into the generator
- Generate a new `generated_parser.rs` module
- Test the generated parser against our existing test cases

### Step 3: Replace Current Parser
- Update `interpreter.rs` to use the generated parser
- Verify all existing functionality still works
- Test specifically that `~3+~3` now parses correctly

### Step 4: Validation
- Run comprehensive tests on various J expressions
- Verify parse trees match expected grammar rules
- Ensure no regression in working expressions

## Success Criteria

### Functional Requirements
1. **Correct Parsing**: `~3+~3` should parse as `(~3)+(~3)` not `~(3+~3)`
2. **Backward Compatibility**: All currently working expressions continue to work
3. **Grammar Compliance**: Parse trees exactly match EBNF rules

### Quality Requirements
1. **Error Handling**: Clear, actionable parse error messages
2. **Performance**: No significant slowdown in parsing
3. **Maintainability**: Easy to modify grammar and regenerate parser

## Risk Mitigation

### Complexity Risk
- Start with a simple parser generator focused only on our specific needs
- Use proven parsing algorithms (LL or LR)
- Incremental development with frequent testing

### Integration Risk
- Maintain existing interfaces to minimize changes to other modules
- Comprehensive testing before replacing the current parser
- Keep the old parser as a fallback during development

### Performance Risk
- Profile generated parser performance
- Optimize generator output if needed
- Consider hand-optimization of critical paths

## Conclusion

A custom parser generator is the most effective solution to our J parsing problems. It will ensure our parser exactly follows the formal EBNF grammar we've designed, fixing the `~3+~3` issue and providing a solid foundation for future J language features.

The generator approach gives us both correctness and maintainability, turning our formal grammar specification into working code that we can trust and extend.