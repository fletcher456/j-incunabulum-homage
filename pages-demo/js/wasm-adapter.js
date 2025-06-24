// WASM Adapter for GitHub Pages J Language Interpreter
class WasmAdapter {
    constructor(wasmModule) {
        this.wasm = wasmModule;
        this.isInitialized = false;
        
        // Verify initialization
        if (wasmModule && typeof wasmModule.evaluate_j_expression === 'function') {
            this.isInitialized = true;
            console.log('WasmAdapter initialized successfully');
        } else {
            console.error('WasmAdapter initialization failed - function not found');
        }
    }
    
    // Direct WASM function call adapter
    evaluateExpression(expression) {
        if (!this.isInitialized) {
            throw new Error('WASM adapter not properly initialized');
        }
        
        try {
            const result = this.wasm.evaluate_j_expression(expression);
            console.log('WASM evaluation result:', result);
            return result;
        } catch (error) {
            console.error('WASM evaluation error:', error);
            throw new Error('WASM evaluation failed: ' + error.message);
        }
    }
    
    // Check if adapter is ready
    isReady() {
        return this.isInitialized;
    }
}

// Global WASM adapter instance
let wasmAdapter;