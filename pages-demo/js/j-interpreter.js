/**
 * J Language Interpreter Application
 * Main application logic for the interactive J language demo
 */

import wasmLoader from './wasm-loader.js';

class JInterpreterApp {
    constructor() {
        this.wasmReady = false;
        this.expressionHistory = [];
        this.historyIndex = -1;
        this.performanceStats = {
            totalEvaluations: 0,
            totalTime: 0,
            averageTime: 0
        };
        
        // Initialize application
        this.initializeElements();
        this.attachEventListeners();
        this.initializeWasm();
        this.setupKeyboardShortcuts();
        this.loadPersistedHistory();
    }
    
    /**
     * Initialize DOM elements
     */
    initializeElements() {
        // Core elements
        this.inputField = document.getElementById('j-input');
        this.evaluateBtn = document.getElementById('evaluate-btn');
        this.resultDisplay = document.getElementById('result-display');
        this.statusDisplay = document.getElementById('wasm-status');
        this.evalTimeDisplay = document.getElementById('eval-time');
        
        // History elements
        this.historyList = document.getElementById('history-list');
        this.clearHistoryBtn = document.getElementById('clear-history');
        
        // Example buttons
        this.exampleButtons = document.querySelectorAll('.example-btn');
        
        // Validate all elements exist
        this.validateElements();
    }
    
    /**
     * Validate that all required elements exist
     */
    validateElements() {
        const requiredElements = [
            'inputField', 'evaluateBtn', 'resultDisplay', 
            'statusDisplay', 'historyList'
        ];
        
        for (const elementName of requiredElements) {
            if (!this[elementName]) {
                console.error(`Required element not found: ${elementName}`);
                throw new Error(`Missing required DOM element: ${elementName}`);
            }
        }
    }
    
    /**
     * Attach event listeners
     */
    attachEventListeners() {
        // Evaluate button
        this.evaluateBtn.addEventListener('click', () => {
            this.evaluateExpression();
        });
        
        // Input field events
        this.inputField.addEventListener('keypress', (e) => {
            if (e.key === 'Enter') {
                e.preventDefault();
                this.evaluateExpression();
            }
        });
        
        this.inputField.addEventListener('input', (e) => {
            this.handleInputChange(e.target.value);
        });
        
        // History navigation
        this.inputField.addEventListener('keydown', (e) => {
            this.handleHistoryNavigation(e);
        });
        
        // Example buttons
        this.exampleButtons.forEach(btn => {
            btn.addEventListener('click', (e) => {
                const expression = e.currentTarget.dataset.expr;
                const description = e.currentTarget.dataset.desc;
                
                this.loadExample(expression, description);
            });
        });
        
        // Clear history
        if (this.clearHistoryBtn) {
            this.clearHistoryBtn.addEventListener('click', () => {
                this.clearHistory();
            });
        }
        
        // Window events
        window.addEventListener('beforeunload', () => {
            this.persistHistory();
        });
    }
    
    /**
     * Set up keyboard shortcuts
     */
    setupKeyboardShortcuts() {
        document.addEventListener('keydown', (e) => {
            // Ctrl/Cmd + Enter: Evaluate
            if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
                e.preventDefault();
                this.evaluateExpression();
            }
            
            // Escape: Clear input
            if (e.key === 'Escape') {
                this.inputField.value = '';
                this.inputField.focus();
            }
            
            // Ctrl/Cmd + L: Clear history
            if ((e.ctrlKey || e.metaKey) && e.key === 'l') {
                e.preventDefault();
                this.clearHistory();
            }
        });
    }
    
    /**
     * Handle input field changes
     */
    handleInputChange(value) {
        // Enable/disable evaluate button based on input
        this.evaluateBtn.disabled = !this.wasmReady || value.trim().length === 0;
        
        // Clear previous results when typing
        if (value.trim().length === 0) {
            this.displayResult('Ready for J expressions...', 'placeholder');
        }
    }
    
    /**
     * Handle history navigation with arrow keys
     */
    handleHistoryNavigation(e) {
        if (e.key === 'ArrowUp') {
            e.preventDefault();
            this.navigateHistory('previous');
        } else if (e.key === 'ArrowDown') {
            e.preventDefault();
            this.navigateHistory('next');
        }
    }
    
    /**
     * Navigate through expression history
     */
    navigateHistory(direction) {
        if (this.expressionHistory.length === 0) return;
        
        if (direction === 'previous') {
            if (this.historyIndex > 0) {
                this.historyIndex--;
            } else {
                this.historyIndex = this.expressionHistory.length - 1;
            }
        } else if (direction === 'next') {
            if (this.historyIndex < this.expressionHistory.length - 1) {
                this.historyIndex++;
            } else {
                this.historyIndex = 0;
            }
        }
        
        const expression = this.expressionHistory[this.historyIndex];
        this.inputField.value = expression.text;
        this.inputField.select();
    }
    
    /**
     * Initialize WebAssembly module
     */
    async initializeWasm() {
        this.updateStatus('Initializing WebAssembly...', 'loading');
        
        try {
            const success = await wasmLoader.initialize();
            
            if (success) {
                this.wasmReady = true;
                this.updateStatus('WebAssembly Ready', 'ready');
                this.evaluateBtn.disabled = false;
                
                // Show performance metrics
                const metrics = wasmLoader.getPerformanceMetrics();
                console.log('🎯 WASM Performance:', metrics);
                
                // Auto-evaluate first example if no input
                if (!this.inputField.value.trim()) {
                    this.loadExample('~3+~3', 'Demonstration of negation and addition');
                }
                
            } else {
                throw new Error('WASM initialization failed');
            }
            
        } catch (error) {
            console.error('WASM initialization error:', error);
            this.updateStatus('WebAssembly Failed', 'error');
            this.evaluateBtn.disabled = true;
            
            // Show fallback message
            this.displayResult(
                'WebAssembly initialization failed. Please check console for details.',
                'error'
            );
        }
    }
    
    /**
     * Load an example expression
     */
    loadExample(expression, description) {
        this.inputField.value = expression;
        this.inputField.focus();
        
        // Auto-evaluate after a brief delay
        setTimeout(() => {
            this.evaluateExpression();
        }, 100);
        
        // Log example usage
        console.log(`📝 Loaded example: ${expression} (${description})`);
    }
    
    /**
     * Evaluate the current expression
     */
    evaluateExpression() {
        if (!this.wasmReady) {
            this.displayResult('WebAssembly not ready', 'error');
            return;
        }
        
        const expression = this.inputField.value.trim();
        if (!expression) {
            this.displayResult('Please enter an expression', 'warning');
            return;
        }
        
        try {
            console.log(`🧮 Evaluating: ${expression}`);
            const startTime = performance.now();
            
            // Evaluate expression
            const result = wasmLoader.evaluateExpression(expression);
            
            const endTime = performance.now();
            const duration = endTime - startTime;
            
            // Update performance stats
            this.updatePerformanceStats(duration);
            
            // Display result
            this.displayResult(result, 'success');
            this.displayEvaluationTime(duration);
            
            // Add to history
            this.addToHistory(expression, result, duration);
            
            console.log(`✅ Evaluation completed in ${duration.toFixed(3)}ms`);
            
        } catch (error) {
            console.error('❌ Evaluation error:', error);
            this.displayResult(`Error: ${error.message}`, 'error');
            this.displayEvaluationTime(0);
            
            // Add error to history
            this.addToHistory(expression, `Error: ${error.message}`, 0);
        }
    }
    
    /**
     * Update performance statistics
     */
    updatePerformanceStats(duration) {
        this.performanceStats.totalEvaluations++;
        this.performanceStats.totalTime += duration;
        this.performanceStats.averageTime = 
            this.performanceStats.totalTime / this.performanceStats.totalEvaluations;
    }
    
    /**
     * Display evaluation result
     */
    displayResult(result, type = 'success') {
        // Clear previous result
        this.resultDisplay.innerHTML = '';
        
        if (type === 'placeholder') {
            const placeholderDiv = document.createElement('div');
            placeholderDiv.className = 'result-placeholder';
            placeholderDiv.textContent = result;
            this.resultDisplay.appendChild(placeholderDiv);
        } else {
            const resultDiv = document.createElement('div');
            resultDiv.className = `result-content result-${type}`;
            
            // Format result for display
            const formattedResult = this.formatResult(result, type);
            resultDiv.innerHTML = formattedResult;
            
            this.resultDisplay.appendChild(resultDiv);
        }
        
        // Add animation
        this.resultDisplay.style.opacity = '0';
        requestAnimationFrame(() => {
            this.resultDisplay.style.opacity = '1';
        });
        
        // Update accessibility
        this.resultDisplay.setAttribute('aria-label', `Result: ${result}`);
    }
    
    /**
     * Format result for better display
     */
    formatResult(result, type) {
        if (type === 'error') {
            return `<span class="error-icon">⚠️</span> ${this.escapeHtml(result)}`;
        } else if (type === 'warning') {
            return `<span class="warning-icon">⚡</span> ${this.escapeHtml(result)}`;
        } else {
            // Format J language results
            const escaped = this.escapeHtml(result);
            
            // Check if it's a matrix (contains newlines)
            if (escaped.includes('\n')) {
                return `<pre class="matrix-result">${escaped}</pre>`;
            } else {
                return `<code class="scalar-result">${escaped}</code>`;
            }
        }
    }
    
    /**
     * Escape HTML characters
     */
    escapeHtml(text) {
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }
    
    /**
     * Display evaluation time
     */
    displayEvaluationTime(duration) {
        if (this.evalTimeDisplay) {
            if (duration > 0) {
                this.evalTimeDisplay.textContent = `${duration.toFixed(2)}ms`;
                this.evalTimeDisplay.style.opacity = '1';
            } else {
                this.evalTimeDisplay.style.opacity = '0';
            }
        }
    }
    
    /**
     * Update status display
     */
    updateStatus(message, type) {
        const statusIcon = this.statusDisplay.querySelector('.status-icon');
        const statusText = this.statusDisplay.querySelector('.status-text');
        
        if (statusText) {
            statusText.textContent = message;
        }
        
        // Update status classes
        this.statusDisplay.className = `wasm-status ${type}`;
        
        // Update icon based on status
        if (statusIcon) {
            statusIcon.className = `status-icon ${type}`;
        }
    }
    
    /**
     * Add expression to history
     */
    addToHistory(expression, result, duration) {
        const historyItem = {
            id: Date.now(),
            text: expression,
            result: result,
            duration: duration,
            timestamp: new Date().toISOString()
        };
        
        // Avoid duplicates
        const isDuplicate = this.expressionHistory.some(item => 
            item.text === expression && item.result === result
        );
        
        if (!isDuplicate) {
            this.expressionHistory.unshift(historyItem);
            
            // Limit history size
            if (this.expressionHistory.length > 50) {
                this.expressionHistory = this.expressionHistory.slice(0, 50);
            }
            
            this.updateHistoryDisplay();
            this.historyIndex = -1; // Reset history navigation
        }
    }
    
    /**
     * Update history display
     */
    updateHistoryDisplay() {
        if (!this.historyList) return;
        
        if (this.expressionHistory.length === 0) {
            this.historyList.innerHTML = '<div class="history-empty">No expressions yet</div>';
            return;
        }
        
        const historyHTML = this.expressionHistory.map(item => `
            <div class="history-item" data-expression="${this.escapeHtml(item.text)}">
                <div class="history-expression">
                    <button class="history-btn" title="Click to reuse">
                        <code>${this.escapeHtml(item.text)}</code>
                    </button>
                </div>
                <div class="history-result">
                    ${this.formatResult(item.result, item.result.startsWith('Error:') ? 'error' : 'success')}
                </div>
                <div class="history-meta">
                    <span class="history-time">${item.duration.toFixed(2)}ms</span>
                    <span class="history-timestamp">${this.formatTimestamp(item.timestamp)}</span>
                </div>
            </div>
        `).join('');
        
        this.historyList.innerHTML = historyHTML;
        
        // Add click listeners to history items
        this.historyList.querySelectorAll('.history-btn').forEach(btn => {
            btn.addEventListener('click', (e) => {
                const expression = e.currentTarget.closest('.history-item').dataset.expression;
                this.inputField.value = expression;
                this.inputField.focus();
            });
        });
    }
    
    /**
     * Format timestamp for display
     */
    formatTimestamp(timestamp) {
        const date = new Date(timestamp);
        const now = new Date();
        const diffMs = now - date;
        const diffMins = Math.floor(diffMs / 60000);
        
        if (diffMins < 1) {
            return 'just now';
        } else if (diffMins < 60) {
            return `${diffMins}m ago`;
        } else if (diffMins < 1440) {
            return `${Math.floor(diffMins / 60)}h ago`;
        } else {
            return date.toLocaleDateString();
        }
    }
    
    /**
     * Clear expression history
     */
    clearHistory() {
        this.expressionHistory = [];
        this.historyIndex = -1;
        this.updateHistoryDisplay();
        this.persistHistory();
        
        console.log('🗑️ Expression history cleared');
    }
    
    /**
     * Load persisted history from localStorage
     */
    loadPersistedHistory() {
        try {
            const stored = localStorage.getItem('j-interpreter-history');
            if (stored) {
                this.expressionHistory = JSON.parse(stored);
                this.updateHistoryDisplay();
                console.log(`📚 Loaded ${this.expressionHistory.length} expressions from history`);
            }
        } catch (error) {
            console.warn('Failed to load history from localStorage:', error);
        }
    }
    
    /**
     * Persist history to localStorage
     */
    persistHistory() {
        try {
            localStorage.setItem('j-interpreter-history', JSON.stringify(this.expressionHistory));
        } catch (error) {
            console.warn('Failed to persist history to localStorage:', error);
        }
    }
    
    /**
     * Get application statistics
     */
    getStatistics() {
        return {
            wasmReady: this.wasmReady,
            historyCount: this.expressionHistory.length,
            performanceStats: this.performanceStats,
            wasmMetrics: wasmLoader.getPerformanceMetrics()
        };
    }
}

// Initialize application when DOM is ready
document.addEventListener('DOMContentLoaded', () => {
    try {
        const app = new JInterpreterApp();
        
        // Make app available globally for debugging
        window.jInterpreterApp = app;
        
        console.log('🎉 J Language Interpreter initialized successfully');
        
    } catch (error) {
        console.error('❌ Failed to initialize J Language Interpreter:', error);
        
        // Show error message to user
        const errorDiv = document.createElement('div');
        errorDiv.className = 'initialization-error';
        errorDiv.innerHTML = `
            <h2>Initialization Error</h2>
            <p>Failed to initialize the J Language Interpreter.</p>
            <p>Error: ${error.message}</p>
            <p>Please check the browser console for more details.</p>
        `;
        document.body.appendChild(errorDiv);
    }
});

// Export for testing
export { JInterpreterApp };