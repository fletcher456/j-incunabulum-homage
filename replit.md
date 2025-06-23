# J Language Interpreter Project

## Overview
Rust-based web server with J language interpreter implementing array programming language features. Started as C webserver, evolved to full J interpreter with calculator interface and WASM deployment capability.

## Recent Changes
- **Jun 23, 2025**: Discovered LALRPOP configuration issue - conditional exclusion not working for WASM builds
- **Jun 23, 2025**: Completed LALRPOP-free build analysis revealing dependency leakage problem
- **Jun 23, 2025**: Confirmed pre-generated parser approach is sound but needs dependency fixes
- **Jun 23, 2025**: Documented comprehensive timeout analysis and configuration issues

## Project Architecture

### Core Components
- **Tokenizer** (`tokenizer.rs`): Lexical analysis for J expressions
- **LALRPOP Parser** (`j_grammar.lalrpop`): Grammar-based parsing with precedence
- **Semantic Analyzer** (`semantic_analyzer.rs`): Context resolution for ambiguous operators
- **Evaluator** (`evaluator.rs`): Expression evaluation with array operations
- **J Array System** (`j_array.rs`): Multi-dimensional array data structures
- **Web Interface** (`j_repl.html`): Calculator-style input with matrix display

### WASM Integration
- **Hybrid Compilation**: LALRPOP generates parser natively, compiles to WASM
- **HTTP Adapter**: Minimal layer intercepting `/j_eval` requests for WASM
- **Graceful Fallback**: Automatic server fallback when WASM unavailable
- **Zero Breaking Changes**: Existing functionality preserved entirely

### Supported Operations
- Basic arithmetic: `+`, `-`, `*`, `%` (with monadic/dyadic forms)
- Array operations: reshape `#`, indexing `{`, concatenation `,`, boxing `<`
- Advanced features: Multi-dimensional arrays, vector operations, compound expressions

## Implementation Status
- ✅ Core J interpreter with all major operators
- ✅ Web interface with calculator buttons and matrix formatting
- ✅ LALRPOP parser integration with proper precedence
- ✅ Multi-dimensional array support with enhanced data structures
- ✅ WASM adapter architecture with fallback system
- ✅ Hybrid WASM compilation approach (LALRPOP native, execution WASM)
- 🔄 WASM build reaches 78/82 components consistently (environment timeout limits)

## User Preferences
- Prioritize minimal complexity over minimal code
- Value organizational simplicity and encapsulation
- Prefer comprehensive analysis documents before implementation
- Focus on maintaining existing functionality while adding capabilities

## Technical Decisions
- **LALRPOP over hand-written parser**: Better grammar maintenance and correctness
- **Hybrid WASM compilation**: Confirmed necessary due to LALRPOP's fundamental WASM timeout issues
- **HTTP adapter pattern**: Preserves existing interfaces during WASM transition
- **Server-first deployment**: Primary strategy validated by isolated build analysis
- **Matrix formatting with `<pre>` tags**: Solves browser CSS inheritance issues