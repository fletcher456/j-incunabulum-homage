// Help text for the J interpreter

// Return help text
pub fn get_help_text() -> String {
    r#"
J Language Web REPL - Help
==========================

This J interpreter implements a subset of the J programming language based on 
the original fragment. J is an array programming language particularly well-suited
for mathematical and statistical operations.

VERBS (OPERATORS)
----------------
Verbs can be used in monadic (single argument) or dyadic (two argument) form.

Monadic Verbs (prefix form):
+  (Plus)     Identity function: returns the argument unchanged
               Example: + 1 2 3  ->  [1 2 3]

{  (Brace)    Size: returns the size of the first dimension
               Example: { 1 2 3  ->  3

~  (Tilde)    Iota: generates array [0,1,2,...,n-1]
               Example: ~5  ->  [0 1 2 3 4]

<  (Less)     Box: encapsulate an array
               Example: < 1 2 3  ->  <[1 2 3]>

#  (Hash)     Shape: returns the dimensions of an array
               Example: # 2 3 # 1 2 3 4 5 6  ->  [2 3]

Dyadic Verbs (infix form):
+  (Plus)     Addition: element-wise addition of arrays
               Example: 1 2 3 + 4 5 6  ->  [5 7 9]

{  (Brace)    From: index selection
               Example: 1 { 7 8 9  ->  8

~  (Tilde)    Find: search for elements
               Example: 2 ~ 1 2 3  ->  1

#  (Hash)     Reshape: change dimensions while preserving data
               Example: 2 3 # 1 2 3 4 5 6  ->  [[1 2 3][4 5 6]]

,  (Comma)    Concatenate: join arrays together
               Example: 1 2 3 , 4 5 6  ->  [1 2 3 4 5 6]

ARRAYS
------
- Scalar: A single value (e.g., 42)
- Vector: A sequence of values separated by spaces (e.g., 1 2 3)
- Matrix: Created using reshape (e.g., 2 3 # 1 2 3 4 5 6)

EXAMPLES
--------
~5                    Generate array [0 1 2 3 4]
1 2 3 + 4 5 6         Add arrays element-wise
2 3 # 1 2 3 4 5 6     Reshape into a 2×3 matrix
1 { 7 8 9             Select element at index 1 (second element)
1 2 3 , 4 5 6         Concatenate arrays
# 2 3 # 1 2 3 4 5 6   Get shape of a matrix
"#.to_string()
}