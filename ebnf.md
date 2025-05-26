# Extended Backus-Naur Form (EBNF) Notation

## Overview

Extended Backus-Naur Form (EBNF) notation is used to define grammar rules. Each rule in the grammar defines one symbol, in the form:

```
symbol ::= expression
```

**Symbol Naming Convention:**
- Symbols are written with an **initial capital letter** if they are the start symbol of a regular language
- Otherwise written with an **initial lowercase letter** 
- Literal strings are quoted

## Character and String Matching

Within the expression on the right-hand side of a rule, the following expressions are used to match strings of one or more characters:

### Unicode Character Reference
```
#xN
```
Where `N` is a hexadecimal integer, the expression matches the character whose number (code point) in ISO/IEC 10646 is N. The number of leading zeros in the `#xN` form is insignificant.

### Character Ranges
```
[a-zA-Z]
[#xN-#xN]
```
Matches any character with a value in the range(s) indicated (inclusive).

### Character Enumeration
```
[abc]
[#xN#xN#xN]
```
Matches any character with a value among the characters enumerated. Enumerations and ranges can be mixed in one set of brackets.

### Negated Character Ranges
```
[^a-z]
[^#xN-#xN]
```
Matches any character with a value outside the range indicated.

### Negated Character Enumeration
```
[^abc]
[^#xN#xN#xN]
```
Matches any character with a value not among the characters given. Enumerations and ranges of forbidden values can be mixed in one set of brackets.

### Literal Strings
```
"string"    // Double quotes
'string'    // Single quotes
```
Matches a literal string matching that given inside the quotes.

## Complex Pattern Combination

These symbols may be combined to match more complex patterns as follows, where `A` and `B` represent simple expressions:

### Grouping
```
(expression)
```
Expression is treated as a unit and may be combined as described in this list.

### Optional
```
A?
```
Matches `A` or nothing; optional `A`.

### Concatenation
```
A B
```
Matches `A` followed by `B`. This operator has higher precedence than alternation; thus `A B | C D` is identical to `(A B) | (C D)`.

### Alternation
```
A | B
```
Matches `A` or `B`.

### Subtraction
```
A - B
```
Matches any string that matches `A` but does not match `B`.

### One or More
```
A+
```
Matches one or more occurrences of `A`. Concatenation has higher precedence than alternation; thus `A+ | B+` is identical to `(A+) | (B+)`.

### Zero or More
```
A*
```
Matches zero or more occurrences of `A`. Concatenation has higher precedence than alternation; thus `A* | B*` is identical to `(A*) | (B*)`.

## Additional Notations

### Comments
```
/* ... */
```
Used for comments within the grammar definition.

## Operator Precedence

From highest to lowest precedence:
1. Grouping `()`
2. Repetition `+`, `*`, `?`
3. Concatenation (implicit)
4. Alternation `|`
5. Subtraction `-`