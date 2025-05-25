document.addEventListener('DOMContentLoaded', function() {
    const outputElement = document.getElementById('output');
    const inputElement = document.getElementById('input-field');
    let history = [];
    let historyIndex = -1;

    // Add welcome message
    appendToOutput('Welcome to J Web REPL! Type "help" for a list of commands.', 'system');

    // Handle input
    inputElement.addEventListener('keydown', function(event) {
        if (event.key === 'Enter') {
            event.preventDefault();
            const input = inputElement.value.trim();
            
            if (input) {
                // Add to history
                history.push(input);
                historyIndex = history.length;
                
                // Display input
                appendToOutput('> ' + input, 'expression');
                
                // Process input
                processInput(input);
                
                // Clear input field
                inputElement.value = '';
            }
        } else if (event.key === 'ArrowUp') {
            event.preventDefault();
            if (historyIndex > 0) {
                historyIndex--;
                inputElement.value = history[historyIndex];
            }
        } else if (event.key === 'ArrowDown') {
            event.preventDefault();
            if (historyIndex < history.length - 1) {
                historyIndex++;
                inputElement.value = history[historyIndex];
            } else if (historyIndex === history.length - 1) {
                historyIndex = history.length;
                inputElement.value = '';
            }
        }
    });

    // Process user input
    function processInput(input) {
        // Send to the server using fetch API
        fetch('/evaluate', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ expression: input }),
        })
        .then(response => response.json())
        .then(data => {
            if (data.error) {
                appendToOutput(data.error, 'error');
            } else {
                appendToOutput(data.result, 'result');
            }
        })
        .catch(error => {
            appendToOutput('Error communicating with server: ' + error, 'error');
        });
    }

    // Add text to the output area
    function appendToOutput(text, className = '') {
        const line = document.createElement('div');
        line.textContent = text;
        if (className) {
            line.classList.add(className);
        }
        outputElement.appendChild(line);
        
        // Scroll to bottom
        outputElement.scrollTop = outputElement.scrollHeight;
    }

    // Set focus to input field
    inputElement.focus();
});