# J Order of Operations

This document explains the abstract sequence of transformations that must take place to evaluate a J expression like `1+~5`.

## J Execution Model

In J, expressions are generally evaluated from right to left, which is different from most programming languages. This means that in a sequence of verbs, the rightmost verb is applied first, and then the result is passed to the verb to its left, and so on.

Additionally, J distinguishes between monadic (one argument) and dyadic (two argument) verbs. The same symbol can represent different operations depending on whether it's used monadically or dyadically.

## Example: Evaluating `1+~5`

The expression `1+~5` involves the monadic iota verb (`~`) and the dyadic plus verb (`+`). Let's break down the evaluation steps:

### Step 1: Parsing and Tokenization

The expression `1+~5` is parsed into tokens:
- `1`: A numeric literal (scalar)
- `+`: The plus verb
- `~`: The iota verb
- `5`: A numeric literal (scalar)

### Step 2: Building the Abstract Syntax Tree (AST)

In J's right-to-left execution model, we build the AST starting from the right:

```
   +
  / \
 1   ~
     |
     5
```

### Step 3: Evaluating Monadic Verbs

Starting from the rightmost operation, we evaluate `~5` (the monadic iota applied to 5):
1. `~5` creates an array of integers from 0 to 4: `[0, 1, 2, 3, 4]`

So now our expression is effectively: `1 + [0, 1, 2, 3, 4]`

### Step 4: Evaluating Dyadic Verbs

Now we evaluate the dyadic plus operation `1 + [0, 1, 2, 3, 4]`:
1. The scalar `1` is added to each element of the array `[0, 1, 2, 3, 4]`
2. This results in the array `[1, 2, 3, 4, 5]`

### Step 5: Returning the Result

The final result of evaluating `1+~5` is the array `[1, 2, 3, 4, 5]`.

## Implementation Considerations

To implement this evaluation process in our J interpreter, we need:

1. **Parser**: A function to parse the input string into tokens
2. **AST Builder**: A function to build an abstract syntax tree from the tokens
3. **Evaluator**: A function to evaluate the AST in the correct order (right-to-left)

For monadic verbs like `~`, we need to identify when a verb is being used monadically (no left argument) and apply the appropriate monadic function.

For dyadic verbs like `+`, we need to identify when a verb is being used dyadically (has both left and right arguments) and apply the appropriate dyadic function.

The key challenge is handling the parsing and evaluation in the correct order, ensuring that monadic verbs are evaluated before their results are used as arguments to dyadic verbs.