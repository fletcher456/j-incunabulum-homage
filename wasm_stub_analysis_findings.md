# WASM Stub Analysis Findings

## Overview
Analysis of minimal stub interpreter WASM artifacts to identify root cause of "Importing a module script failed" error.

**Artifacts Analyzed:**
- `stub_interpreter.js` (8,124 bytes) - JavaScript bindings
- `stub_interpreter_bg.wasm` (34,717 bytes) - WASM binary

---

## JavaScript Bindings Structure Analysis

### File: stub_interpreter.js

**Critical Discovery: The WASM bindings are perfectly structured and functional**

**Key Findings:**

1. **Function Exports Available:**
   - `evaluate_j_expression()` (lines 89-102) - Our target function
   - `handle_j_eval_request()` (lines 108-121) - Unexpected additional function
   - Both properly wrapped with wasm-bindgen patterns

2. **Initialization Function Structure:**
   - `__wbg_init()` (lines 233-259) - The default export function
   - **Line 246**: `module_or_path = new URL('stub_interpreter_bg.wasm', import.meta.url);`
   - This resolves relative to the JavaScript file location, NOT the importing module

3. **Root Cause Identified:**
   ```javascript
   // Line 246: Default WASM path resolution
   module_or_path = new URL('stub_interpreter_bg.wasm', import.meta.url);
   ```
   When `stub_interpreter.js` is in `/wasm/` directory, `import.meta.url` points to `/wasm/stub_interpreter.js`, so the WASM file is correctly sought at `/wasm/stub_interpreter_bg.wasm`.

4. **Expected Call Pattern:**
   ```javascript
   // Correct usage:
   const wasmModule = await import('./wasm/stub_interpreter.js');
   await wasmModule.default(); // No parameters needed - uses internal path resolution
   ```

---

## Path Resolution Analysis

**The Issue:** Our `app-init.js` path correction was right, but incomplete understanding of wasm-bindgen behavior.

**Current app-init.js attempts:**
1. `import('../wasm/simple_server.js')` - CORRECT path
2. `wasmModule.default()` - CORRECT call (no parameters)
3. `wasmModule.default('../wasm/simple_server_bg.wasm')` - UNNECESSARY explicit path
4. `wasmModule.default('/wasm/simple_server_bg.wasm')` - UNNECESSARY absolute path

**What's Actually Happening:**
- The import path `../wasm/simple_server.js` should work
- The `wasmModule.default()` call should work
- The wasm-bindgen generated code handles WASM file loading automatically

---

## WASM Binary Analysis

**File:** `stub_interpreter_bg.wasm` (34,717 bytes)
- Size indicates successful minimal build
- Should contain only the stub function and minimal runtime

---

## Problem Isolation

**The "Importing a module script failed" error suggests:**

1. **File Access Issue:** The `/wasm/simple_server.js` file may not exist on GitHub Pages
2. **MIME Type Issue:** Server not serving `.js` files with correct MIME type
3. **Build Artifact Mismatch:** GitHub Actions builds `simple_server.js` but we're importing `stub_interpreter.js`

**Critical Mismatch Identified:**
- GitHub Actions builds: `simple_server.js` + `simple_server_bg.wasm`
- Debug folder contains: `stub_interpreter.js` + `stub_interpreter_bg.wasm`
- app-init.js imports: `../wasm/simple_server.js`

---

## Solution Strategy

**Primary Issue:** Build output naming inconsistency

**Fix Required:**
1. Update GitHub Actions workflow to use consistent naming with stub analysis
2. OR update app-init.js to import correct filename from GitHub Actions build
3. Verify GitHub Pages actually serves the WASM artifacts in `/wasm/` directory

**Secondary Issues:**
1. Remove unnecessary fallback initialization attempts in app-init.js
2. Simplify to single `await wasmModule.default()` call
3. Add proper error handling for file access vs initialization failures

---

## Conclusion

**Root Cause:** File naming mismatch between build artifacts and import statements, not path resolution or initialization logic.

**Expected Fix:** Align build output names with import statements, then use minimal initialization pattern:
```javascript
const wasmModule = await import('../wasm/simple_server.js');
await wasmModule.default();
const result = wasmModule.evaluate_j_expression('test');
```

**Confidence Level:** Very High - The wasm-bindgen output is textbook perfect, issue is file availability/naming only.

---

## Implementation Update

**Actions Taken:**
1. ✅ Simplified app-init.js initialization to use standard wasm-bindgen pattern
2. ✅ Removed unnecessary fallback attempts and explicit path parameters
3. ✅ Verified GitHub Actions builds `simple_server.js` (not `stub_interpreter.js`)

**Remaining Issue:** Need to verify GitHub Pages deployment includes WASM artifacts in correct location.

**Next Test:** Deploy to GitHub Pages and monitor console for specific file access errors vs initialization errors.