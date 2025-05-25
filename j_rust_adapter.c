#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/**
 * This is an adapter that provides compatibility with our webserver's expected interface.
 * It calls the Rust-based J interpreter and handles memory management.
 */

// Import the Rust functions
extern char* interpret_j_code(const char* code);
extern void free_string(char* s);

// Function that matches the interface expected by the webserver
char* execute_j_code(const char* code) {
    char* result;
    char* copy;

    // Call the Rust implementation
    result = interpret_j_code(code);
    if (!result) {
        return strdup("Error: Failed to interpret J code");
    }

    // Make a copy of the result that will be managed by the C code
    copy = strdup(result);
    
    // Free the original result from Rust
    free_string(result);
    
    return copy;
}