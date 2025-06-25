// WASM Initialization for GitHub Pages
async function initializeWasmEngine() {
    try {
        console.log('Initializing WASM for GitHub Pages...');
        
        // Check if we're on GitHub Pages
        const isGitHubPages = location.hostname.includes('github.io');
        const isReplit = location.hostname.includes('replit');
        
        if (isGitHubPages) {
            // GitHub Pages WASM loading
            console.log('Loading WASM module from GitHub Pages...');
            console.log('Current script location:', window.location.pathname);
            const wasmModule = await import('../wasm/simple_server.js');
            
            // Initialize WASM (wasm-bindgen handles path resolution automatically)
            console.log('Initializing WASM module...');
            await wasmModule.default();
            console.log('WASM initialized successfully');
            
            // Verify the function exists
            if (typeof wasmModule.evaluate_j_expression !== 'function') {
                throw new Error('evaluate_j_expression function not found in WASM module');
            }
            
            console.log('WASM module loaded successfully');
            console.log('Available functions:', Object.keys(wasmModule).filter(key => typeof wasmModule[key] === 'function'));
            console.log('Module keys:', Object.keys(wasmModule));
            console.log('evaluate_j_expression type:', typeof wasmModule.evaluate_j_expression);
            
            // Create compatible interface
            window.wasmLoader = {
                isReady: () => true,
                evaluateExpression: (expr) => {
                    try {
                        return wasmModule.evaluate_j_expression(expr);
                    } catch (error) {
                        return 'Error: ' + error.message;
                    }
                }
            };
            
            console.log('WASM engine ready for GitHub Pages');
            
        } else if (isReplit) {
            // Replit server fallback
            console.log('Using Replit server fallback mode');
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
            throw new Error('Unsupported environment');
        }
        
    } catch (error) {
        console.error('WASM initialization failed:', error);
        console.error('Error details:', error.stack);
        
        // Fallback mode
        window.wasmLoader = {
            isReady: () => false,
            evaluateExpression: () => 'Error: WASM module failed to load'
        };
    }
}

// Auto-initialize when page loads
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initializeWasmEngine);
} else {
    initializeWasmEngine();
}