#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

// Simplified J interpreter for basic operations
// Focuses on numerical operations and arrays

// Array structure for J-like operations
typedef struct {
    int length;
    double *values;
} JArray;

// Create a new array
JArray* create_array(int length) {
    JArray* arr = (JArray*)malloc(sizeof(JArray));
    arr->length = length;
    arr->values = (double*)malloc(length * sizeof(double));
    return arr;
}

// Free an array
void free_array(JArray* arr) {
    if (arr) {
        free(arr->values);
        free(arr);
    }
}

// Parse string to create array (e.g., "1 2 3" -> [1, 2, 3])
JArray* parse_array(const char* input) {
    // Count spaces to estimate array size
    int count = 1;
    for (const char* p = input; *p; p++) {
        if (*p == ' ') count++;
    }
    
    JArray* arr = create_array(count);
    
    // Parse numbers
    int index = 0;
    char* input_copy = strdup(input);
    char* token = strtok(input_copy, " ");
    
    while (token && index < count) {
        arr->values[index++] = atof(token);
        token = strtok(NULL, " ");
    }
    
    arr->length = index; // Adjust length in case count was overestimated
    free(input_copy);
    return arr;
}

// Iota function: i.n creates array [0,1,...,n-1]
JArray* iota(int n) {
    if (n <= 0 || n > 1000) return NULL; // Safety check
    
    JArray* result = create_array(n);
    for (int i = 0; i < n; i++) {
        result->values[i] = i;
    }
    return result;
}

// Add a scalar to each element of an array
JArray* array_add(const JArray* arr, double value) {
    JArray* result = create_array(arr->length);
    for (int i = 0; i < arr->length; i++) {
        result->values[i] = arr->values[i] + value;
    }
    return result;
}

// Subtract a scalar from each element of an array
JArray* array_subtract(const JArray* arr, double value) {
    JArray* result = create_array(arr->length);
    for (int i = 0; i < arr->length; i++) {
        result->values[i] = arr->values[i] - value;
    }
    return result;
}

// Multiply each element of an array by a scalar
JArray* array_multiply(const JArray* arr, double value) {
    JArray* result = create_array(arr->length);
    for (int i = 0; i < arr->length; i++) {
        result->values[i] = arr->values[i] * value;
    }
    return result;
}

// Divide each element of an array by a scalar
JArray* array_divide(const JArray* arr, double value) {
    if (value == 0) return NULL; // Division by zero
    
    JArray* result = create_array(arr->length);
    for (int i = 0; i < arr->length; i++) {
        result->values[i] = arr->values[i] / value;
    }
    return result;
}

// Format array to string representation
char* array_to_string(const JArray* arr) {
    if (!arr) return strdup("Error: Invalid array");
    
    // Allocate plenty of space for the result
    char* result = (char*)malloc(arr->length * 30 + 10);
    int offset = 0;
    
    offset += sprintf(result + offset, "[");
    for (int i = 0; i < arr->length; i++) {
        if (i > 0) offset += sprintf(result + offset, " ");
        // Use %.8g for a reasonable representation of the number
        offset += sprintf(result + offset, "%.8g", arr->values[i]);
    }
    offset += sprintf(result + offset, "]");
    
    return result;
}

// Main interpretation function
char* interpret_j_code(const char* code) {
    static char buffer[4096];
    memset(buffer, 0, sizeof(buffer));
    
    // Check for empty input
    if (!code || strlen(code) == 0) {
        strcpy(buffer, "Error: Empty expression");
        return buffer;
    }
    
    // Check for iota (i.n) expression
    if (strncmp(code, "i.", 2) == 0) {
        int n = atoi(code + 2);
        JArray* result = iota(n);
        
        if (result) {
            char* str = array_to_string(result);
            strncpy(buffer, str, sizeof(buffer) - 1);
            free(str);
            free_array(result);
        } else {
            strcpy(buffer, "Error: Invalid iota parameter");
        }
        return buffer;
    }
    
    // Look for basic arithmetic operators between two values
    const char* op_pos = strpbrk(code, "+-*/%^");
    if (op_pos) {
        // Check if there are spaces (array operation)
        if (strchr(code, ' ')) {
            // This is an array operation like "1 2 3 + 5"
            char left_part[1024] = {0};
            strncpy(left_part, code, op_pos - code);
            
            char op = *op_pos;
            double right_value = atof(op_pos + 1);
            
            JArray* left_arr = parse_array(left_part);
            JArray* result = NULL;
            
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
                    strcpy(buffer, "Error: Unsupported array operation");
                    free_array(left_arr);
                    return buffer;
            }
            
            if (result) {
                char* str = array_to_string(result);
                strncpy(buffer, str, sizeof(buffer) - 1);
                free(str);
                free_array(result);
            } else {
                strcpy(buffer, "Error: Operation failed (possibly division by zero)");
            }
            
            free_array(left_arr);
            return buffer;
        } else {
            // Simple operation between two numbers like "2+3"
            double left = atof(code);
            char op = *op_pos;
            double right = atof(op_pos + 1);
            double result;
            
            switch (op) {
                case '+':
                    result = left + right;
                    break;
                case '-':
                    result = left - right;
                    break;
                case '*':
                    result = left * right;
                    break;
                case '/':
                    if (right == 0) {
                        strcpy(buffer, "Error: Division by zero");
                        return buffer;
                    }
                    result = left / right;
                    break;
                case '%':
                    if (right == 0) {
                        strcpy(buffer, "Error: Modulo by zero");
                        return buffer;
                    }
                    result = fmod(left, right);
                    break;
                case '^':
                    result = pow(left, right);
                    break;
                default:
                    strcpy(buffer, "Error: Unknown operator");
                    return buffer;
            }
            
            sprintf(buffer, "%.8g", result);
            return buffer;
        }
    }
    
    // If it's just an array like "1 2 3"
    if (strchr(code, ' ')) {
        JArray* arr = parse_array(code);
        char* str = array_to_string(arr);
        strncpy(buffer, str, sizeof(buffer) - 1);
        free(str);
        free_array(arr);
        return buffer;
    }
    
    // If it's just a single number
    if (strspn(code, "0123456789.eE+-") == strlen(code)) {
        sprintf(buffer, "%.8g", atof(code));
        return buffer;
    }
    
    // If we couldn't interpret it
    strcpy(buffer, "Error: Could not interpret J expression");
    return buffer;
}

// Function to be called from the webserver
char* execute_j_code(const char* code) {
    return interpret_j_code(code);
}