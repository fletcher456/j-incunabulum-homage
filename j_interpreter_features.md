# J Interpreter Features

Based on the provided J interpreter fragment, here are the key features and components that a J interpreter should include:

## Core Data Structures

- **Array Structure**: A fundamental data type that can represent scalars, vectors, matrices, and higher-dimensional arrays
- **Type System**: Support for different data types (numeric and boxed arrays)
- **Rank System**: Ability to handle arrays of different ranks (dimensions)

## Memory Management

- **Memory Allocation**: Functions to allocate memory for arrays and data
- **Memory Copy**: Functionality to copy data between arrays
- **Array Creation**: Utilities to create arrays with specific dimensions and types

## Array Operations

### Unary Verbs (Single Argument)
Based on the fragment's verb table `C vt[]="+{~<#,"` and the monadic function pointer array `A(*vm[])()={0,id,size,iota,box,sha,0}`:

- **Identity (Monad `+`)**: Return the argument unchanged (maps to `id` function, index 1)
- **Size (Monad `{`)**: Return the size of the first dimension (maps to `size` function, index 2)
- **Iota (Monad `~`)**: Generate an array of consecutive integers starting from 0 (maps to `iota` function, index 3)
- **Box (Monad `<`)**: Encapsulate an array inside a scalar box (maps to `box` function, index 4)
- **Shape (Monad `#`)**: Return the dimensions of an array (maps to `sha` function, index 5)

### Binary Verbs (Two Arguments)
Based on the fragment's verb table and the dyadic function pointer array `A(*vd[])()={0,plus,from,find,0,rsh,cat}`:

- **Plus (Dyad `+`)**: Element-wise addition of arrays (maps to `plus` function, index 1)
- **From (Dyad `{`)**: Index selection (select items from an array) (maps to `from` function, index 2)
- **Find (Dyad `~`)**: Search for elements (maps to `find` function, index 3)
- **Reshape (Dyad `#`)**: Change the dimensions of an array while preserving data (maps to `rsh` function, index 5)
- **Concatenate (Dyad `,`)**: Join arrays together (maps to `cat` function, index 6)

### User Input Examples

In the REPL, a user could enter:
- `~5` to generate an array of integers from 0 to 4 (iota function)
- `1 2 3 + 4 5 6` for element-wise addition of arrays
- `2 3 # 1 2 3 4 5 6` to reshape a vector into a 2×3 matrix
- `1 { 7 8 9` to select the second element (index 1) from the array
- `1 2 3 , 4 5 6` to concatenate two arrays

## Parsing and Execution

- **Lexical Analysis**: Convert source code into tokens
- **Symbol Tables**: Store variable values
- **Expression Evaluation**: Recursive evaluation of expressions
- **Operator Precedence**: Handle the J language's right-to-left execution order

## Input/Output

- **Array Printing**: Format and display arrays of different ranks
- **Pretty Printing**: Properly indent and format nested structures
- **REPL Interface**: Read-Evaluate-Print Loop for interactive use

## Language Features

- **Variable Assignment**: Store values in named variables
- **Function Composition**: Build complex operations from simpler ones
- **Tacit Programming**: Support for point-free style of programming
- **Error Handling**: Detect and report execution errors

## Primitive Functions

- **Arithmetic Operations**: Addition, subtraction, multiplication, division
- **Array Manipulation**: Reshaping, slicing, joining
- **Structural Operations**: Transposition, reversal, rotation

## Execution Model

- **Right-to-Left Execution**: The J execution model evaluates from right to left
- **Monadic/Dyadic Distinction**: Functions behave differently based on number of arguments