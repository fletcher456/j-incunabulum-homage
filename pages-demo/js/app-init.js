// WASM Initialization for GitHub Pages
async function initializeWasmEngine() {
    try {
        console.log('Initializing WASM for GitHub Pages...');
        
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
        
        console.log('WASM engine ready for GitHub Pages');
        
    } catch (error) {
        console.log('WASM failed, using server fallback:', error.message);
        
        // For testing in Replit, create a working server adapter
        if (location.hostname === 'localhost' || location.hostname.includes('replit')) {
            window.wasmLoader = {
                isReady: () => true,
                evaluateExpression: async (expr) => {
                    try {
                        const response = await fetch('/evaluate', {
                            method: 'POST',
                            headers: { 'Content-Type': 'application/json' },
                            body: JSON.stringify({ expression: expr, parser: 'custom' })
                        });
                        const data = await response.json();
                        return data.result || data.error || 'No result';
                    } catch (e) {
                        return 'Error: ' + e.message;
                    }
                }
            };
        } else {
            // Pure fallback mode for GitHub Pages without WASM
            window.wasmLoader = {
                isReady: () => false,
                evaluateExpression: () => { throw new Error('WASM not available'); }
            };
        }
    }
}

// Auto-initialize when page loads
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initializeWasmEngine);
} else {
    initializeWasmEngine();
}