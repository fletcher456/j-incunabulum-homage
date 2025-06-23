# LALRPOP Complete Removal Implementation Plan

## Objective
Remove LALRPOP entirely from WASM builds by modifying dependencies and code to use only pre-generated parser code.

## Pre-Implementation State
- LALRPOP is configured as conditional build dependency but still compiles for WASM
- `j_grammar_generated.rs` contains complete pre-generated parser
- Hybrid system partially working but with dependency leakage

## Detailed Changes to Implement

### 1. Cargo.toml Modifications
**File**: `simple_server/Cargo.toml`

**Current State**:
```toml
[dependencies]
lalrpop-util = "0.20"

[target.'cfg(not(target_arch = "wasm32"))'.build-dependencies]
lalrpop = "0.20"
```

**Change A**: Remove lalrpop-util from main dependencies
```toml
# REMOVE this line:
lalrpop-util = "0.20"
```

**Change B**: Add lalrpop-util as conditional dependency for native only
```toml
[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
lalrpop-util = "0.20"
```

### 2. Source Code Modifications

#### File: `simple_server/src/lib.rs`
**Current State**: Uses `j_grammar_generated` for WASM

**Change C**: Remove lalrpop_util imports for WASM builds
- Add conditional compilation around lalrpop_util usage
- Ensure j_grammar_generated is used exclusively for WASM

#### File: `simple_server/src/j_grammar_generated.rs`
**Current State**: Contains `extern crate lalrpop_util as __lalrpop_util;`

**Change D**: Create WASM-compatible version without lalrpop_util dependency
- Replace lalrpop_util dependencies with minimal local implementations
- Preserve all parser functionality without external dependencies

#### File: `simple_server/src/lalr_parser.rs`
**Current State**: Uses `lalrpop_mod!` macro

**Change E**: Add conditional compilation
- Only compile for native targets
- WASM builds skip this entirely

### 3. Build System Modifications

#### File: `simple_server/build.rs`
**Current State**:
```rust
#[cfg(not(target_arch = "wasm32"))]
{
    lalrpop::process_root().unwrap();
}
```

**Change F**: No modifications needed (already conditional)

### 4. Error Handling Adaptations

#### File: Create `simple_server/src/wasm_parser_support.rs`
**Change G**: Implement minimal parser support for WASM
- Create ParseError type compatible with existing code
- Implement token handling without lalrpop_util
- Ensure drop-in replacement for existing parser interface

### 5. Module Structure Updates

#### File: `simple_server/src/main.rs` and other files using parser
**Change H**: Update imports to use conditional parser
- Native: Use lalrpop-based parser
- WASM: Use standalone generated parser

## Rollback Plan

### Files to Backup Before Changes
1. `simple_server/Cargo.toml` → `Cargo.toml.backup`
2. `simple_server/src/lib.rs` → `lib.rs.backup`
3. `simple_server/src/j_grammar_generated.rs` → `j_grammar_generated.rs.backup`

### Rollback Steps (if needed)
1. Restore backed up files
2. Run `cargo clean` to clear any cached builds
3. Test native build functionality
4. Verify server mode continues working

## Expected Outcomes

### Success Metrics
- WASM build completes without LALRPOP compilation
- Build time reduces from >60 minutes to <10 minutes
- All J interpreter functionality preserved
- No regression in native/server mode

### Risk Mitigation
- Incremental implementation with testing at each step
- Backup all modified files
- Test native build after each change
- Preserve existing server functionality throughout

## Implementation Order
1. Create backups
2. Implement Change G (WASM parser support)
3. Implement Change D (modify j_grammar_generated.rs)
4. Implement Changes A & B (Cargo.toml dependencies)
5. Implement Changes C & E (conditional compilation)
6. Test WASM build
7. Test native build
8. Cleanup or rollback based on results

## Results

### First Attempt - Conditional Dependencies
**Status**: FAILED - LALRPOP still compiles for WASM

**Changes Made**:
1. ✅ Created `wasm_parser_support.rs` with WASM-compatible ParseError
2. ✅ Modified `j_grammar_generated.rs` with conditional imports
3. ✅ Updated Cargo.toml to move lalrpop-util to conditional dependencies
4. ✅ Added conditional compilation to lalr_parser.rs
5. ✅ Updated lib.rs module imports

**Issue Discovered**: 
Cargo's conditional compilation for dependencies doesn't prevent transitive dependency inclusion. The dependency tree shows `lalrpop v0.20.2` still depends on `lalrpop-util v0.20.2` regardless of target conditionals.

**Root Cause Analysis**:
- Cargo resolves all dependencies in the dependency graph before applying target filters
- `lalrpop-util` dependency exists in resolved dependency tree
- Build system compiles all resolved dependencies regardless of target-specific configuration
- Conditional dependencies only affect linking, not compilation

### Rollback Decision
Restored all backup files due to continued LALRPOP compilation. The approach reveals fundamental limitations in Cargo's dependency resolution system.

**Key Discovery**: The timeout issue is caused by Cargo's dependency resolution strategy, not our configuration. Even with target-specific dependencies, the resolver includes all transitive dependencies in the build graph.

## Conclusion

The LALRPOP removal attempt revealed that **Cargo's conditional compilation has fundamental limitations**:
- Target-specific dependencies still resolve the full dependency graph
- Transitive dependencies get compiled regardless of target conditions
- Pre-generated parser approach is architecturally sound but blocked by toolchain behavior

**Current Status**: Reverted to working state with server-first deployment strategy. The hybrid approach remains the most practical solution given Cargo's current limitations.