// WASM Initialization with Automatic Fallback
async function initializeWasmEngine() {
    try {
        const wasm = await import('./pkg/simple_server.js');
        await wasm.default();
        wasmAdapter = new WasmHttpAdapter(wasm);
        console.log('WASM engine ready');
    } catch (error) {
        console.log('WASM failed, falling back to server:', error.message);
        // Existing server mode continues to work
    }
}

// Auto-initialize when page loads
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initializeWasmEngine);
} else {
    initializeWasmEngine();
}