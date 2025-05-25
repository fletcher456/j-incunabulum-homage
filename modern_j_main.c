#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/**
 * Main function for the modernized J interpreter
 * This file provides a standalone way to test the J interpreter
 */

/* Forward declaration for the execute_j_code function from modern_j.c */
extern char* execute_j_code(const char *code);

int main(int argc, char *argv[]) {
    printf("Modern J Interpreter\n");
    printf("====================\n\n");
    
    if (argc > 1) {
        /* Execute the J code provided as command line argument */
        char* result = execute_j_code(argv[1]);
        printf("Result:\n%s\n", result);
    } else {
        /* Interactive mode */
        char input[1024];
        printf("Enter J expressions (Ctrl+D to exit):\n");
        
        while (1) {
            printf("> ");
            if (fgets(input, sizeof(input), stdin) == NULL) {
                break;
            }
            
            /* Remove newline character */
            size_t len = strlen(input);
            if (len > 0 && input[len-1] == '\n') {
                input[len-1] = '\0';
            }
            
            /* Exit on empty input */
            if (strlen(input) == 0) {
                continue;
            }
            
            /* Execute the J code */
            char* result = execute_j_code(input);
            printf("%s\n", result);
        }
    }
    
    return 0;
}