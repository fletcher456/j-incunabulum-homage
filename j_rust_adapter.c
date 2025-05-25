#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Import the Rust functions through the FFI interface
extern char* interpret_j_code(const char* input);
extern void free_string(char* s);

// Wrapper function for the webserver to use
char* execute_j_rust(const char* code) {
    char* result = interpret_j_code(code);
    
    // Make a copy of the result string that can be handled by the C code
    char* result_copy = strdup(result);
    
    // Free the original string allocated by Rust
    free_string(result);
    
    return result_copy;
}