# Formal Strategy for Handling Context Sensitivity in J Language Grammar

## The Context Sensitivity Problem

J language has inherent context-sensitive features that cannot be directly expressed in context-free EBNF grammar:

1. **Monadic vs Dyadic Verbs**: The same symbol (e.g., `+`) can be either monadic (identity) or dyadic (addition) depending on context
2. **Right-to-Left Evaluation**: J evaluates expressions from right to left, creating parsing ambiguities
3. **Operator Precedence**: All verbs have the same precedence, but their interpretation depends on syntactic position

## Proposed Strategy: Lexical Context Disambiguation

### Phase 1: Context-Free Parsing with Ambiguous Nodes

Create an EBNF grammar that accepts all syntactically valid J expressions but defers context resolution:

```ebnf
Expression ::= Term (Verb Expression)?
Term ::= Verb Expression | Atom | '(' Expression ')'
Atom ::= Number | Vector
Vector ::= Number (' '+ Number)*
Number ::= [0-9]+
Verb ::= [+~#<{,]
```

This produces an initial parse tree with **ambiguous verb nodes** that don't yet specify monadic/dyadic nature.

### Phase 2: Context Resolution Pass

After the initial parse, apply a **context resolution algorithm** that traverses the tree and determines verb types based on syntactic position:

#### Rule Set for Context Resolution:

1. **Leading Verb Rule**: A verb at the start of an expression is always monadic
   ```
   ~5     → MonadicVerb(~, 5)
   +3 4   → MonadicVerb(+, Vector[3,4])
   ```

2. **Verb After Atom Rule**: A verb immediately following an atom/expression creates a dyadic operation
   ```
   3+4    → DyadicVerb(+, 3, 4)
   2 3+~5 → DyadicVerb(+, Vector[2,3], MonadicVerb(~, 5))
   ```

3. **Verb After Verb Rule**: Consecutive verbs create monadic operations (right-to-left)
   ```
   5++3   → DyadicVerb(+, 5, MonadicVerb(+, 3))
   ~+3    → MonadicVerb(~, MonadicVerb(+, 3))
   ```

### Phase 3: Right-to-Left Restructuring

Transform the left-associative parse tree into J's right-associative evaluation order:

#### Algorithm: Right-Associative Transform

```pseudocode
function transformRightAssociative(node):
    if node is DyadicVerb(op, left, right):
        if right is DyadicVerb(op2, right_left, right_right):
            // Transform: (A op (B op2 C)) → ((A op B) op2 C) 
            // Only if both ops have same precedence (which they do in J)
            return DyadicVerb(op2, 
                     DyadicVerb(op, left, right_left), 
                     transformRightAssociative(right_right))
        else:
            return DyadicVerb(op, left, transformRightAssociative(right))
    else:
        return node
```

## Implementation Strategy

### Step 1: Enhanced EBNF Grammar

```ebnf
/* J Language Grammar - Context-Free Phase */
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

### Step 2: AST Node Types

```rust
enum JNode {
    // Final resolved nodes
    Literal(JArray),
    MonadicVerb(char, Box<JNode>),
    DyadicVerb(char, Box<JNode>, Box<JNode>),
    
    // Intermediate ambiguous nodes
    AmbiguousVerb(char, Option<Box<JNode>>, Option<Box<JNode>>),
}
```

### Step 3: Context Resolution Algorithm

```rust
impl JInterpreter {
    fn resolve_context(&self, node: JNode) -> Result<JNode, String> {
        match node {
            JNode::AmbiguousVerb(verb, left, right) => {
                match (left, right) {
                    (None, Some(right)) => {
                        // Leading verb - monadic
                        Ok(JNode::MonadicVerb(verb, Box::new(self.resolve_context(*right)?)))
                    },
                    (Some(left), Some(right)) => {
                        // Verb between expressions - dyadic
                        Ok(JNode::DyadicVerb(
                            verb,
                            Box::new(self.resolve_context(*left)?),
                            Box::new(self.resolve_context(*right)?)
                        ))
                    },
                    _ => Err("Invalid verb context".to_string())
                }
            },
            other => Ok(other)
        }
    }
}
```

## Advantages of This Strategy

### **Separation of Concerns**
- **Phase 1**: Pure syntax parsing (context-free)
- **Phase 2**: Semantic analysis (context resolution)
- **Phase 3**: J-specific evaluation ordering

### **Formal Correctness**
- EBNF grammar remains context-free and formally verifiable
- Context sensitivity handled in well-defined semantic pass
- Clear rules for ambiguity resolution

### **Extensibility**
- Easy to add new verbs (just update the EBNF verb rule)
- Context resolution rules can be refined independently
- Debugging is simplified with clear phase separation

### **Error Reporting**
- Syntax errors caught in Phase 1 with precise locations
- Context errors (like impossible verb combinations) caught in Phase 2
- Evaluation errors caught in Phase 3

## Handling Edge Cases

### **Complex Expressions**
For expressions like `~3+~3`:
1. **Phase 1**: Parse as `AmbiguousVerb(~) AmbiguousVerb(+) AmbiguousVerb(~) Number(3)`
2. **Phase 2**: Resolve to `MonadicVerb(~, DyadicVerb(+, Number(3), MonadicVerb(~, Number(3))))`
3. **Phase 3**: Transform for right-to-left: `DyadicVerb(+, MonadicVerb(~, Number(3)), MonadicVerb(~, Number(3)))`

### **Parenthetical Expressions**
Parentheses create unambiguous grouping that overrides default precedence, handled naturally in Phase 1.

### **Vector Operations**
Vector tokenization (already implemented) works seamlessly with this strategy.

## Conclusion

This three-phase strategy formally handles J's context sensitivity while maintaining the benefits of EBNF grammar specification. It provides a clean, extensible foundation for correctly parsing all J expressions, including the problematic cases like `~3+~3` that our current parser mishandles.