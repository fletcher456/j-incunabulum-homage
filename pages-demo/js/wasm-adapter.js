// WASM Adapter for GitHub Pages J Language Interpreter
class WasmAdapter {
    constructor(wasmModule) {
        this.wasm = wasmModule;
    }
    
    // Direct WASM function call adapter
    evaluateExpression(expression) {
        if (!this.wasm || !this.wasm.evaluate_j_expression) {
            throw new Error('WASM module not properly initialized');
        }
        return this.wasm.evaluate_j_expression(expression);
    }
}

// Global WASM adapter instance
let wasmAdapter;