#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

// Simple J-like interpreter with basic array support
typedef struct {
    int length;
    double *values;
} Array;

// Function to create a new array
Array* create_array(int length) {
    Array* arr = (Array*)malloc(sizeof(Array));
    arr->length = length;
    arr->values = (double*)malloc(length * sizeof(double));
    return arr;
}

// Function to free an array
void free_array(Array* arr) {
    free(arr->values);
    free(arr);
}

// Parse a J-like array expression like "1 2 3 4"
Array* parse_array(const char* input) {
    int capacity = 10;
    Array* arr = create_array(capacity);
    int count = 0;
    
    char* str = strdup(input);
    char* token = strtok(str, " ");
    
    while (token != NULL) {
        if (count >= capacity) {
            capacity *= 2;
            arr->values = (double*)realloc(arr->values, capacity * sizeof(double));
        }
        
        arr->values[count++] = atof(token);
        token = strtok(NULL, " ");
    }
    
    arr->length = count;
    free(str);
    return arr;
}

// Function to handle iota (i.) operation
Array* iota(int n) {
    Array* result = create_array(n);
    for (int i = 0; i < n; i++) {
        result->values[i] = i;
    }
    return result;
}

// Function to handle array addition
Array* array_add(Array* arr, double value) {
    Array* result = create_array(arr->length);
    for (int i = 0; i < arr->length; i++) {
        result->values[i] = arr->values[i] + value;
    }
    return result;
}

// Function to handle array subtraction
Array* array_subtract(Array* arr, double value) {
    Array* result = create_array(arr->length);
    for (int i = 0; i < arr->length; i++) {
        result->values[i] = arr->values[i] - value;
    }
    return result;
}

// Function to handle array multiplication
Array* array_multiply(Array* arr, double value) {
    Array* result = create_array(arr->length);
    for (int i = 0; i < arr->length; i++) {
        result->values[i] = arr->values[i] * value;
    }
    return result;
}

// Function to handle array division
Array* array_divide(Array* arr, double value) {
    if (value == 0) {
        return NULL; // Division by zero
    }
    
    Array* result = create_array(arr->length);
    for (int i = 0; i < arr->length; i++) {
        result->values[i] = arr->values[i] / value;
    }
    return result;
}

// Function to convert array to string
char* array_to_string(Array* arr) {
    if (!arr) return strdup("Error: Invalid array");
    
    // Calculate the size needed for the string
    // Each number could take up to 20 chars, plus spaces and brackets
    int size = arr->length * 20 + 10;
    char* result = (char*)malloc(size);
    
    int offset = 0;
    offset += snprintf(result + offset, size - offset, "[");
    
    for (int i = 0; i < arr->length; i++) {
        if (i > 0) {
            offset += snprintf(result + offset, size - offset, " ");
        }
        offset += snprintf(result + offset, size - offset, "%.2f", arr->values[i]);
    }
    
    offset += snprintf(result + offset, size - offset, "]");
    return result;
}

// Main interpretation function
char* interpret_j_code(const char* code) {
    static char result_buffer[4096];
    memset(result_buffer, 0, sizeof(result_buffer));
    
    // Check for iota (i.n) operation
    if (strncmp(code, "i.", 2) == 0) {
        int n = atoi(code + 2);
        if (n > 0 && n < 100) { // Limit to reasonable size
            Array* arr = iota(n);
            char* str = array_to_string(arr);
            strncpy(result_buffer, str, sizeof(result_buffer) - 1);
            free(str);
            free_array(arr);
            return result_buffer;
        } else {
            snprintf(result_buffer, sizeof(result_buffer), "Error: Invalid iota parameter");
            return result_buffer;
        }
    }
    
    // Check for basic arithmetic on single values
    if (strchr("+-*/%^", code[1]) && !strchr(code, ' ')) {
        double left = atof(code);
        char op = code[strspn(code, "0123456789.")];
        double right = atof(code + strspn(code, "0123456789.") + 1);
        
        switch (op) {
            case '+':
                snprintf(result_buffer, sizeof(result_buffer), "%.2f", left + right);
                break;
            case '-':
                snprintf(result_buffer, sizeof(result_buffer), "%.2f", left - right);
                break;
            case '*':
                snprintf(result_buffer, sizeof(result_buffer), "%.2f", left * right);
                break;
            case '/':
                if (right == 0) {
                    snprintf(result_buffer, sizeof(result_buffer), "Error: Division by zero");
                } else {
                    snprintf(result_buffer, sizeof(result_buffer), "%.2f", left / right);
                }
                break;
            case '%':
                if (right == 0) {
                    snprintf(result_buffer, sizeof(result_buffer), "Error: Modulo by zero");
                } else {
                    snprintf(result_buffer, sizeof(result_buffer), "%.2f", fmod(left, right));
                }
                break;
            case '^':
                snprintf(result_buffer, sizeof(result_buffer), "%.2f", pow(left, right));
                break;
            default:
                snprintf(result_buffer, sizeof(result_buffer), "Error: Unknown operator");
        }
        return result_buffer;
    }
    
    // Handle array operations (e.g. "1 2 3 + 5")
    char* op_loc = strpbrk(code, "+-*/%^");
    if (op_loc && strchr(code, ' ')) {
        // Split into left array and right operand
        int op_pos = op_loc - code;
        char left_part[1024] = {0};
        strncpy(left_part, code, op_pos);
        
        char op = code[op_pos];
        double right_value = atof(code + op_pos + 1);
        
        Array* left_arr = parse_array(left_part);
        Array* result = NULL;
        
        switch (op) {
            case '+':
                result = array_add(left_arr, right_value);
                break;
            case '-':
                result = array_subtract(left_arr, right_value);
                break;
            case '*':
                result = array_multiply(left_arr, right_value);
                break;
            case '/':
                result = array_divide(left_arr, right_value);
                break;
            default:
                snprintf(result_buffer, sizeof(result_buffer), "Error: Unsupported array operation");
                free_array(left_arr);
                return result_buffer;
        }
        
        if (result) {
            char* str = array_to_string(result);
            strncpy(result_buffer, str, sizeof(result_buffer) - 1);
            free(str);
            free_array(result);
        } else {
            snprintf(result_buffer, sizeof(result_buffer), "Error: Operation failed");
        }
        
        free_array(left_arr);
        return result_buffer;
    }
    
    // If it's just a simple array, parse and return it
    if (strchr(code, ' ')) {
        Array* arr = parse_array(code);
        char* str = array_to_string(arr);
        strncpy(result_buffer, str, sizeof(result_buffer) - 1);
        free(str);
        free_array(arr);
        return result_buffer;
    }
    
    // If it's just a number, return it
    if (strspn(code, "0123456789.") == strlen(code)) {
        snprintf(result_buffer, sizeof(result_buffer), "%.2f", atof(code));
        return result_buffer;
    }
    
    // Fallback
    snprintf(result_buffer, sizeof(result_buffer), "Error: Could not interpret J expression");
    return result_buffer;
}

// Function to be called from the webserver
char* execute_j_code(const char* code) {
    return interpret_j_code(code);
}