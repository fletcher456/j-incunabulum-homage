# J Language Interpreter Minimal WASM Adapter Design

## Goal
Convert the Rust J interpreter to WASM while maintaining the existing HTTP request/response interface through a minimal adapter layer. This approach preserves the current client-server architecture while gaining deployment benefits.

## Design Principle
**Minimal Complexity**: Introduce the smallest possible abstraction layer that maintains existing interfaces while enabling WASM execution.

## Architecture Overview

### Current Flow
```
Browser JS → HTTP POST /j_eval → Rust Server → JSON Response → Browser JS
```

### Target Flow
```
Browser JS → HTTP-like Request → WASM Engine → JSON Response → Browser JS
```

## Core Components

### 1. WASM Module Interface (Single Function)
```rust
// lib.rs - Minimal WASM interface
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn handle_j_eval_request(request_body: &str) -> String {
    // Parse form data: "expression=4+4#~16"
    let expression = parse_form_data(request_body);
    
    // Use existing evaluation pipeline (unchanged)
    let result = evaluate_j_expression(&expression);
    
    // Return JSON in exact same format as server
    format!(r#"{{"result": "{}"}}"#, escape_json(&result))
}
```

### 2. HTTP Adapter Layer (Minimal JavaScript)
```javascript
// http-adapter.js - Single file adapter
class WasmHttpAdapter {
    constructor(wasmModule) {
        this.wasm = wasmModule;
    }
    
    // Drop-in replacement for fetch('/j_eval', ...)
    async fetch(url, options) {
        if (url === '/j_eval' && options.method === 'POST') {
            const response = this.wasm.handle_j_eval_request(options.body);
            return {
                json: () => Promise.resolve(JSON.parse(response))
            };
        }
        throw new Error('Unsupported request');
    }
}

// Global replacement - zero changes to existing code
let wasmAdapter;
const originalFetch = window.fetch;
window.fetch = async function(url, options) {
    if (wasmAdapter && url === '/j_eval') {
        return wasmAdapter.fetch(url, options);
    }
    return originalFetch(url, options);
};
```

### 3. Initialization (Minimal Bootstrap)
```javascript
// app-init.js - Single initialization block
async function initializeWasmEngine() {
    try {
        const wasm = await import('./pkg/simple_server.js');
        await wasm.default();
        wasmAdapter = new WasmHttpAdapter(wasm);
        console.log('WASM engine ready');
    } catch (error) {
        console.log('WASM failed, falling back to server');
        // Existing server mode continues to work
    }
}

// Auto-initialize when page loads
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initializeWasmEngine);
} else {
    initializeWasmEngine();
}
```

## Implementation Strategy

### Phase 1: WASM Module Creation
- **Cargo.toml Changes**: Minimal additions for WASM target
- **Code Changes**: Single `lib.rs` file with one public function
- **Existing Code**: Zero modifications to evaluation pipeline
- **Build**: `wasm-pack build --target web`

### Phase 2: Adapter Integration
- **File Structure**: Add 2 small JavaScript files
- **HTML Changes**: Include adapter scripts
- **Existing JS**: Zero modifications to calculator logic
- **Fallback**: Automatic fallback to server if WASM fails

### Phase 3: Deployment Options
- **Development**: Server + WASM side-by-side
- **Production**: Static files + WASM (no server needed)
- **Hybrid**: Graceful degradation between modes

## Complexity Minimization

### Encapsulation Benefits
1. **Interface Preservation**: Existing `fetch('/j_eval')` calls unchanged
2. **Error Handling**: Existing error handling logic unchanged
3. **UI Logic**: Calculator interface completely unchanged
4. **Server Logic**: Evaluation pipeline completely unchanged

### Organizational Simplicity
1. **File Count**: Only 3 new files (`lib.rs`, `http-adapter.js`, `app-init.js`)
2. **Responsibilities**: Clear separation - WASM handles computation, adapter handles protocol
3. **Dependencies**: WASM module is completely self-contained
4. **Testing**: Each component testable in isolation

### Code Complexity
- **Added Code**: ~50 lines total across all files
- **Modified Code**: 0 lines in existing evaluation logic
- **Modified Code**: 2-3 lines in HTML to include scripts

## File Structure
```
simple_server/
├── src/
│   ├── lib.rs              # NEW: WASM interface (15 lines)
│   ├── main.rs             # UNCHANGED: Server logic
│   └── [all other files]   # UNCHANGED: Evaluation pipeline
├── static/
│   ├── j_repl.html         # MINIMAL CHANGE: Include 2 scripts
│   ├── http-adapter.js     # NEW: Protocol adapter (20 lines)
│   └── app-init.js         # NEW: Initialization (15 lines)
└── pkg/                    # GENERATED: WASM build output
```

## Deployment Scenarios

### Development Mode
- Server runs normally on port 5000
- WASM loads and takes over `/j_eval` requests
- Seamless transition with zero downtime

### Static Deployment
- Copy `static/` directory to any web server
- WASM handles all computation locally
- No server infrastructure required

### Fallback Mode
- If WASM fails to load, requests go to server
- Automatic graceful degradation
- No user-visible difference

## Risk Mitigation

### Technical Risks
- **LALRPOP Compatibility**: Test WASM compilation early
- **Dependency Issues**: Isolate WASM build from server build
- **Memory Constraints**: Same limitations as current server

### Complexity Risks
- **Interface Drift**: Adapter maintains exact JSON format
- **Testing Complexity**: Unit test WASM module independently
- **Debugging**: Console logging in both WASM and adapter

## Success Metrics
1. **Zero Breaking Changes**: Existing functionality unchanged
2. **Minimal Code Addition**: <100 lines total added code
3. **Clear Separation**: WASM, adapter, and UI remain independent
4. **Deployment Flexibility**: Works in server, static, and hybrid modes
5. **Fallback Reliability**: Graceful degradation when WASM unavailable

## Alternative Considered: Direct WASM Integration
**Rejected because**: Would require modifying existing `submitExpression()` function and error handling logic, increasing complexity and risk of breaking existing functionality.

## Alternative Considered: Service Worker Proxy
**Rejected because**: Service Workers add deployment complexity and browser compatibility concerns while providing no additional encapsulation benefits.

This design achieves WASM deployment with minimal organizational complexity by preserving all existing interfaces and adding only a thin, well-encapsulated adapter layer.