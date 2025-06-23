# J Language Interpreter Project

## Overview
Rust-based web server with J language interpreter implementing array programming language features. Started as C webserver, evolved to full J interpreter with calculator interface and WASM deployment capability.

## Recent Changes
- **Jun 23, 2025**: Successfully completed Phase 1 - Custom parser with literals and basic addition
- **Jun 23, 2025**: Verified Phase 1 functionality: "42", "1+2", "1+2+3" all working correctly
- **Jun 23, 2025**: Confirmed appropriate error handling for unsupported features ("~3", "1 2")
- **Jun 23, 2025**: Created Phase 2 strategy document for monadic operations with precedence handling
- **Jun 23, 2025**: Successfully implemented Phase 0 - Custom parser stub with parser selection UI

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
- ✅ Phase 0: Custom parser stub with parser selection UI working
- ✅ Phase 1: Custom parser supporting literals and addition operations (fully functional)
- 🔄 Phase 2: Monadic operations (~, -) with precedence handling (ready for implementation)

## User Preferences
- Prioritize minimal complexity over minimal code
- Value organizational simplicity and encapsulation
- Prefer comprehensive analysis documents before implementation
- Focus on maintaining existing functionality while adding capabilities

## Technical Decisions
- **LALRPOP over hand-written parser**: Better grammar maintenance and correctness (current)
- **Custom recursive descent parser**: Comprehensive 8-week strategy for complete LALRPOP replacement
- **Server-first deployment**: Primary strategy due to Cargo dependency resolution limitations
- **HTTP adapter pattern**: Preserves existing interfaces with automatic WASM fallback
- **Matrix formatting with `<pre>` tags**: Solves browser CSS inheritance issues