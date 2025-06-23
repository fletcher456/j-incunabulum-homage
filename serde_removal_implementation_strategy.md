# Serde Removal Implementation Strategy
## Enabling WASM Compilation for J Language Interpreter

**Date**: June 23, 2025  
**Goal**: Remove serde dependencies to enable WASM compilation  
**Duration**: 30 minutes implementation  
**Risk Level**: LOW  

## Executive Summary

Based on feasibility analysis, serde removal is the most viable path to WASM compilation success. The J language interpreter uses only basic JSON operations that can be replaced with lightweight manual parsing, eliminating the primary compilation bottleneck.

## Implementation Plan

### Phase 1: Manual JSON Parsing Implementation (15 minutes)

#### 1.1 Create JSON Parsing Functions
**Location**: `simple_server/src/main.rs`

```rust
// Replace serde_json parsing with manual extraction
fn parse_j_eval_request(body: &str) -> (String, String) {
    // Handle JSON format: {"expression": "...", "parser": "..."}
    if body.trim_start().starts_with('{') {
        let expression = extract_json_field(body, "expression").unwrap_or_default();
        let parser = extract_json_field(body, "parser").unwrap_or("custom".to_string());
        (expression, parser)
    } else {
        // Handle form data: expression=...
        if let Some(expr) = body.strip_prefix("expression=") {
            (url_decode(expr), "custom".to_string())
        } else {
            (body.trim().to_string(), "custom".to_string())
        }
    }
}

fn extract_json_field(json: &str, field: &str) -> Option<String> {
    let field_pattern = format!(r#""{}""#, field);
    if let Some(start) = json.find(&field_pattern) {
        let after_field = &json[start + field_pattern.len()..];
        if let Some(colon_pos) = after_field.find(':') {
            let after_colon = after_field[colon_pos + 1..].trim_start();
            if after_colon.starts_with('"') {
                let content = &after_colon[1..];
                if let Some(end_quote) = content.find('"') {
                    // Handle basic JSON escape sequences
                    return Some(json_unescape(&content[..end_quote]));
                }
            }
        }
    }
    None
}

fn json_unescape(s: &str) -> String {
    s.replace("\\\"", "\"")
     .replace("\\\\", "\\")
     .replace("\\n", "\n")
     .replace("\\r", "\r")
     .replace("\\t", "\t")
}

fn url_decode(input: &str) -> String {
    input
        .replace("+", " ")
        .replace("%23", "#")
        .replace("%20", " ")
        .replace("%2B", "+")
        .replace("%26", "&")
        .replace("%3D", "=")
        .replace("%22", "\"")
        .replace("%28", "(")
        .replace("%29", ")")
}
```

#### 1.2 Replace Existing serde_json Calls
**Current Code (lines 72-82):**
```rust
let request_data: serde_json::Value = serde_json::from_str(&body).unwrap_or_else(|_| {
    // Fallback for legacy form data
    if let Some(expression) = body.strip_prefix("expression=") {
        serde_json::json!({"expression": url_decode(expression), "parser": "lalrpop"})
    } else {
        serde_json::json!({"expression": body.trim(), "parser": "lalrpop"})
    }
});

let expression = request_data["expression"].as_str().unwrap_or("");
let _parser_choice = request_data["parser"].as_str().unwrap_or("custom");
```

**New Code:**
```rust
let (expression, _parser_choice) = parse_j_eval_request(&body);
```

### Phase 2: Dependency Removal (5 minutes)

#### 2.1 Update Cargo.toml
**Remove these lines:**
```toml
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

**Final dependencies section:**
```toml
[dependencies]
wasm-bindgen = "0.2"
console_error_panic_hook = "0.1"

# Server-only dependencies
[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
tiny_http = "0.12"
```

#### 2.2 Remove Import Statements
**From main.rs, remove:**
```rust
use serde_json;
```

### Phase 3: WASM Build Validation (10 minutes)

#### 3.1 Build Test Sequence
```bash
# Clean previous builds
cd simple_server && cargo clean

# Test regular build
cargo build --lib

# Test WASM build  
cargo build --target wasm32-unknown-unknown --lib --release

# Attempt wasm-pack build
wasm-pack build --target web --out-dir static/pkg
```

#### 3.2 Functional Testing
**Test Cases:**
1. JSON request: `{"expression":"~3+~3","parser":"custom"}`
2. Form request: `expression=2%203%23~6`
3. Simple request: `1+2+3`
4. Error cases: malformed JSON, empty requests

**Validation Script:**
```bash
# Test server functionality
curl -X POST http://localhost:5000/j_eval \
  -H "Content-Type: application/json" \
  -d '{"expression":"~3+~3","parser":"custom"}'

curl -X POST http://localhost:5000/j_eval \
  -H "Content-Type: application/x-www-form-urlencoded" \
  -d 'expression=2%203%23~6'
```

## Implementation Steps

### Step 1: Backup Current State
```bash
cd simple_server
cp src/main.rs src/main_serde_backup.rs
cp Cargo.toml Cargo_serde_backup.toml
```

### Step 2: Implement Manual JSON Parsing
1. Add new parsing functions to main.rs
2. Replace serde_json calls with manual parsing
3. Test with existing functionality

### Step 3: Remove Dependencies
1. Update Cargo.toml (remove serde lines)
2. Remove import statements
3. Clean build test

### Step 4: WASM Build Attempt
1. Attempt cargo WASM build
2. Try wasm-pack if cargo succeeds
3. Document results and any remaining blockers

### Step 5: Validation
1. Functional testing with curl
2. UI testing with browser interface
3. Performance comparison

## Error Handling Strategy

### JSON Parsing Errors
**Approach**: Graceful degradation with fallbacks
```rust
fn parse_j_eval_request(body: &str) -> (String, String) {
    // Try JSON first, fall back to form data, then raw text
    if body.trim_start().starts_with('{') {
        if let Some(expr) = extract_json_field(body, "expression") {
            let parser = extract_json_field(body, "parser").unwrap_or("custom".to_string());
            return (expr, parser);
        }
    }
    
    // Form data fallback
    if let Some(expr) = body.strip_prefix("expression=") {
        return (url_decode(expr), "custom".to_string());
    }
    
    // Raw text fallback
    (body.trim().to_string(), "custom".to_string())
}
```

### Build Failures
**Contingency Plans:**
1. **WASM build fails**: Document specific error, investigate next dependency
2. **Functionality breaks**: Restore from backup, implement fixes
3. **Performance issues**: Optimize manual parsing if needed

## Expected Outcomes

### Success Scenario
- **Build Time**: WASM compilation completes within timeout
- **Functionality**: 100% preservation of existing features
- **Performance**: <5% impact on request processing
- **Binary Size**: Reduced WASM output

### Partial Success
- **Native Build**: Works without serde
- **WASM Build**: Still blocked by other dependencies
- **Analysis**: Clear path to next dependency removal

### Failure Scenario
- **Functionality Loss**: JSON parsing edge cases not handled
- **Rollback**: Restore serde with lessons learned
- **Alternative**: Consider lightweight JSON library

## Validation Criteria

### Functional Requirements
- [ ] JSON requests parse correctly
- [ ] Form data requests work
- [ ] Response format unchanged
- [ ] Error handling preserved
- [ ] All J language operations functional

### Technical Requirements  
- [ ] Clean build without serde
- [ ] WASM compilation progresses further
- [ ] No new compilation errors
- [ ] Performance within acceptable range

### Integration Requirements
- [ ] Web interface functions normally
- [ ] Server endpoints respond correctly
- [ ] Browser console shows no new errors
- [ ] Fallback mechanisms work

## Rollback Plan

### Quick Rollback (< 5 minutes)
```bash
cd simple_server
cp src/main_serde_backup.rs src/main.rs
cp Cargo_serde_backup.toml Cargo.toml
cargo build
```

### Selective Rollback
1. **Keep manual parsing**: Restore serde for complex cases only
2. **Hybrid approach**: Manual parsing for WASM, serde for server
3. **Incremental migration**: Remove serde gradually

## Post-Implementation Analysis

### Metrics to Track
1. **Build Performance**: Compilation time comparison
2. **Runtime Performance**: Request processing speed
3. **WASM Progress**: How much further compilation proceeds
4. **Code Maintainability**: Complexity of manual parsing

### Next Steps Planning
1. **If Successful**: Proceed with remaining WASM dependencies
2. **If Blocked**: Analyze next compilation bottleneck
3. **If Issues**: Optimize manual parsing implementation

## Risk Mitigation

### Low Risk Items
- **Simple JSON**: Our JSON structure is minimal
- **Existing Tests**: Manual parsing can be thoroughly tested
- **Rollback Ready**: Easy restoration of working state

### Medium Risk Items
- **Edge Cases**: Unusual JSON formatting might break
- **Performance**: Manual parsing slightly slower
- **Maintenance**: More code to maintain than serde

### Mitigation Strategies
- **Comprehensive Testing**: Cover all JSON variations
- **Performance Monitoring**: Benchmark before/after
- **Documentation**: Clear code comments for manual parsing
- **Fallback Logic**: Multiple parsing strategies

## Success Definition

**Primary Goal**: WASM compilation proceeds past serde-related timeouts  
**Secondary Goal**: All functionality preserved with manual JSON parsing  
**Tertiary Goal**: Foundation established for remaining dependency optimization  

This implementation strategy provides a clear, low-risk path to removing the primary WASM compilation bottleneck while maintaining full functionality of the J language interpreter.