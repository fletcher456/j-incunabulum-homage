#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "modern_j.h"

/**
 * Standalone test program for the modern J interpreter
 * This tests basic functionality using the public interface
 */

int main() {
    printf("=== Modern J Interpreter Test ===\n\n");
    
    // Array of J expressions to test
    const char* test_expressions[] = {
        "2+2",
        "3*4",
        "i.5",
        "1 2 3 + 5"
    };
    
    int num_tests = sizeof(test_expressions) / sizeof(test_expressions[0]);
    
    // Run each test expression
    for (int i = 0; i < num_tests; i++) {
        const char* expr = test_expressions[i];
        printf("Expression: %s\n", expr);
        
        // Execute the J code
        char* result = execute_j_code(expr);
        
        // Print the result
        printf("Result:\n%s\n", result);
        printf("----------------------------\n");
    }
    
    return 0;
}