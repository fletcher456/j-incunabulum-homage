/**
 * WebAssembly Module Loader for J Language Interpreter
 * Handles WASM initialization, function exports, and error management
 */

class WasmLoader {
    constructor() {
        this.wasmModule = null;
        this.isLoaded = false;
        this.isLoading = false;
        this.loadPromise = null;
        this.performanceMetrics = {
            loadStart: 0,
            loadEnd: 0,
            initializationTime: 0
        };
    }
    
    /**
     * Initialize the WebAssembly module
     * @returns {Promise<boolean>} Success status
     */
    async initialize() {
        if (this.isLoaded) {
            return true;
        }
        
        if (this.isLoading) {
            return this.loadPromise;
        }
        
        this.isLoading = true;
        this.performanceMetrics.loadStart = performance.now();
        
        this.loadPromise = this._loadWasmModule();
        const result = await this.loadPromise;
        
        this.isLoading = false;
        return result;
    }
    
    /**
     * Internal method to load and initialize WASM module
     * @private
     */
    async _loadWasmModule() {
        try {
            console.log('🚀 Loading WebAssembly module...');
            
            // Check WebAssembly support
            if (!this._checkWasmSupport()) {
                throw new Error('WebAssembly not supported in this browser');
            }
            
            // Import the generated JavaScript bindings  
            console.log('📦 Importing WASM bindings...');
            const wasmModule = await import('./wasm/simple_server.js');
            
            // Initialize the WASM module
            console.log('⚙️ Initializing WASM instance...');
            await wasmModule.default();
            
            // Verify required functions are available
            this._verifyExports(wasmModule);
            
            this.wasmModule = wasmModule;
            this.isLoaded = true;
            
            this.performanceMetrics.loadEnd = performance.now();
            this.performanceMetrics.initializationTime = 
                this.performanceMetrics.loadEnd - this.performanceMetrics.loadStart;
            
            console.log(`✅ WASM module loaded successfully in ${this.performanceMetrics.initializationTime.toFixed(2)}ms`);
            
            // Log module information
            this._logModuleInfo();
            
            return true;
            
        } catch (error) {
            console.error('❌ Failed to load WASM module:', error);
            console.error('Stack trace:', error.stack);
            
            // Provide helpful error messages
            this._handleLoadError(error);
            
            return false;
        }
    }
    
    /**
     * Check if WebAssembly is supported
     * @private
     */
    _checkWasmSupport() {
        if (typeof WebAssembly === 'undefined') {
            console.error('WebAssembly not supported');
            return false;
        }
        
        if (!WebAssembly.instantiate) {
            console.error('WebAssembly.instantiate not available');
            return false;
        }
        
        return true;
    }
    
    /**
     * Verify that required exports are available
     * @private
     */
    _verifyExports(wasmModule) {
        const requiredExports = ['evaluate_j_expression'];
        const availableExports = Object.keys(wasmModule);
        
        console.log('📋 Available WASM exports:', availableExports);
        
        for (const exportName of requiredExports) {
            if (!(exportName in wasmModule)) {
                throw new Error(`Required export '${exportName}' not found in WASM module`);
            }
            
            if (typeof wasmModule[exportName] !== 'function') {
                throw new Error(`Export '${exportName}' is not a function`);
            }
        }
        
        console.log('✅ All required exports verified');
    }
    
    /**
     * Log module information for debugging
     * @private
     */
    _logModuleInfo() {
        if (this.wasmModule) {
            console.log('📊 WASM Module Information:');
            console.log('  - Exports:', Object.keys(this.wasmModule));
            console.log('  - Load time:', this.performanceMetrics.initializationTime.toFixed(2) + 'ms');
            
            // Memory information if available
            if (this.wasmModule.memory) {
                const memoryMB = (this.wasmModule.memory.buffer.byteLength / 1024 / 1024).toFixed(2);
                console.log('  - Memory allocated:', memoryMB + 'MB');
            }
        }
    }
    
    /**
     * Handle load errors with helpful messages
     * @private
     */
    _handleLoadError(error) {
        const errorMessage = error.message.toLowerCase();
        
        if (errorMessage.includes('network')) {
            console.error('💡 Network error - check if WASM files are accessible');
        } else if (errorMessage.includes('cors')) {
            console.error('💡 CORS error - ensure server allows WASM file access');
        } else if (errorMessage.includes('not found') || errorMessage.includes('404')) {
            console.error('💡 File not found - verify WASM files are in /wasm/ directory');
        } else if (errorMessage.includes('compile')) {
            console.error('💡 WASM compilation error - check WASM file integrity');
        } else {
            console.error('💡 Unknown error - check browser console for details');
        }
    }
    
    /**
     * Evaluate a J language expression
     * @param {string} expression - The J expression to evaluate
     * @returns {string} The evaluation result
     */
    evaluateExpression(expression) {
        if (!this.isLoaded || !this.wasmModule) {
            throw new Error('WASM module not loaded. Call initialize() first.');
        }
        
        if (typeof expression !== 'string') {
            throw new Error('Expression must be a string');
        }
        
        if (expression.trim().length === 0) {
            throw new Error('Expression cannot be empty');
        }
        
        try {
            console.log(`🧮 Evaluating: "${expression}"`);
            const startTime = performance.now();
            
            // Call the WASM function
            const result = this.wasmModule.evaluate_j_expression(expression);
            
            const endTime = performance.now();
            const evaluationTime = endTime - startTime;
            
            console.log(`✅ Evaluation completed in ${evaluationTime.toFixed(3)}ms`);
            console.log(`📤 Result: ${result}`);
            
            return result;
            
        } catch (error) {
            console.error('❌ WASM evaluation error:', error);
            
            // Re-throw with more context
            throw new Error(`J expression evaluation failed: ${error.message}`);
        }
    }
    
    /**
     * Get performance metrics
     * @returns {Object} Performance data
     */
    getPerformanceMetrics() {
        return {
            ...this.performanceMetrics,
            isLoaded: this.isLoaded,
            memoryUsage: this.wasmModule?.memory ? 
                (this.wasmModule.memory.buffer.byteLength / 1024 / 1024).toFixed(2) + 'MB' : 
                'Unknown'
        };
    }
    
    /**
     * Check if module is ready for use
     * @returns {boolean} Ready status
     */
    isReady() {
        return this.isLoaded && this.wasmModule;
    }
    
    /**
     * Reset the loader (for testing purposes)
     */
    reset() {
        this.wasmModule = null;
        this.isLoaded = false;
        this.isLoading = false;
        this.loadPromise = null;
        this.performanceMetrics = {
            loadStart: 0,
            loadEnd: 0,
            initializationTime: 0
        };
    }
}

// Create and export singleton instance
const wasmLoader = new WasmLoader();

// Export for both ES6 modules and global access
export default wasmLoader;
window.wasmLoader = wasmLoader;

// Export class for testing
export { WasmLoader };