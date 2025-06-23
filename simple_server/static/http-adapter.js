// HTTP Adapter for WASM J Interpreter
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