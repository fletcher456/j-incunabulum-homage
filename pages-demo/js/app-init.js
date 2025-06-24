// WASM Initialization for GitHub Pages
async function initializeWasmEngine() {
    try {
        console.log('🚀 Initializing WASM for GitHub Pages...');
        
        // Import WASM module from GitHub Pages path
        const wasm = await import('./wasm/simple_server.js');
        await wasm.default();
        
        // Create adapter and make available globally
        wasmAdapter = new WasmAdapter(wasm);
        
        // Create compatible interface for existing code
        window.wasmLoader = {
            isReady: () => true,
            evaluateExpression: (expr) => wasmAdapter.evaluateExpression(expr)
        };
        
        console.log('✅ WASM engine ready for GitHub Pages');
        
    } catch (error) {
        console.log('❌ WASM failed, server fallback mode:', error.message);
        // Fallback mode - existing server code will handle requests
        window.wasmLoader = {
            isReady: () => false,
            evaluateExpression: () => { throw new Error('WASM not available'); }
        };
    }
}

// Auto-initialize when page loads
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initializeWasmEngine);
} else {
    initializeWasmEngine();
}