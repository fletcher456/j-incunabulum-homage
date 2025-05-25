#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

/**
 * A simplified J-like interpreter with basic functionality
 */

/* Array type to hold data */
typedef struct {
    int length;
    double *values;
} JArray;

/* Function to create a new array */
JArray* create_array(int length) {
    JArray* arr = (JArray*)malloc(sizeof(JArray));
    arr->length = length;
    arr->values = (double*)calloc(length, sizeof(double));
    return arr;
}

/* Function to free an array */
void free_array(JArray* arr) {
    if (arr) {
        free(arr->values);
        free(arr);
    }
}

/* Function to create an iota array [0,1,2,...,n-1] */
JArray* iota(int n) {
    JArray* result = create_array(n);
    for (int i = 0; i < n; i++) {
        result->values[i] = i;
    }
    return result;
}

/* Function to parse a space-separated array like "1 2 3" */
JArray* parse_array(const char* input) {
    int count = 1;
    for (const char* p = input; *p; p++) {
        if (*p == ' ') count++;
    }
    
    JArray* arr = create_array(count);
    
    int index = 0;
    char* input_copy = strdup(input);
    char* token = strtok(input_copy, " ");
    
    while (token && index < count) {
        arr->values[index++] = atof(token);
        token = strtok(NULL, " ");
    }
    
    free(input_copy);
    arr->length = index; // Adjust in case we had fewer tokens than spaces
    return arr;
}

/* Add a scalar to each element in an array */
JArray* array_add(JArray* arr, double value) {
    JArray* result = create_array(arr->length);
    for (int i = 0; i < arr->length; i++) {
        result->values[i] = arr->values[i] + value;
    }
    return result;
}

/* Subtract a scalar from each element in an array */
JArray* array_subtract(JArray* arr, double value) {
    JArray* result = create_array(arr->length);
    for (int i = 0; i < arr->length; i++) {
        result->values[i] = arr->values[i] - value;
    }
    return result;
}

/* Multiply each element in an array by a scalar */
JArray* array_multiply(JArray* arr, double value) {
    JArray* result = create_array(arr->length);
    for (int i = 0; i < arr->length; i++) {
        result->values[i] = arr->values[i] * value;
    }
    return result;
}

/* Divide each element in an array by a scalar */
JArray* array_divide(JArray* arr, double value) {
    if (value == 0) return NULL; // Division by zero
    
    JArray* result = create_array(arr->length);
    for (int i = 0; i < arr->length; i++) {
        result->values[i] = arr->values[i] / value;
    }
    return result;
}

/* Convert array to string representation */
char* array_to_string(JArray* arr) {
    if (!arr) return strdup("Error: NULL array");
    
    // Allocate memory for the result
    int size = arr->length * 20 + 10; // Each number + spaces + brackets
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

/* Main interpretation function */
char* execute_j_code(const char* code) {
    // Check for iota function (i.n)
    if (strncmp(code, "i.", 2) == 0) {
        int n = atoi(code + 2);
        if (n > 0 && n < 1000) { // Reasonable limit
            JArray* result = iota(n);
            char* str = array_to_string(result);
            free_array(result);
            return str;
        } else {
            return strdup("Error: Invalid iota parameter");
        }
    }
    
    // Check for array operations
    const char* space = strchr(code, ' ');
    if (space) {
        // Look for operator after the space-separated numbers
        const char* op_ptr = strpbrk(code, "+-*/");
        if (op_ptr && op_ptr > space) {
            // This is like "1 2 3 + 5"
            char left_part[1024] = {0};
            strncpy(left_part, code, op_ptr - code);
            
            char op = *op_ptr;
            double right_value = atof(op_ptr + 1);
            
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
                    free_array(left_arr);
                    return strdup("Error: Unsupported operation");
            }
            
            if (!result) {
                free_array(left_arr);
                return strdup("Error: Operation failed (possibly division by zero)");
            }
            
            char* str = array_to_string(result);
            free_array(left_arr);
            free_array(result);
            return str;
        } else {
            // Just an array with no operation
            JArray* arr = parse_array(code);
            char* str = array_to_string(arr);
            free_array(arr);
            return str;
        }
    }
    
    // Check for simple arithmetic (e.g. "2+2")
    const char* op_ptr = strpbrk(code, "+-*/^");
    if (op_ptr) {
        char left_str[256] = {0};
        strncpy(left_str, code, op_ptr - code);
        
        double left = atof(left_str);
        char op = *op_ptr;
        double right = atof(op_ptr + 1);
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
                    return strdup("Error: Division by zero");
                }
                result = left / right;
                break;
            case '^':
                result = pow(left, right);
                break;
            default:
                return strdup("Error: Unsupported operation");
        }
        
        char* str = malloc(64);
        snprintf(str, 64, "%.2f", result);
        return str;
    }
    
    // Just a single number
    if (strspn(code, "0123456789.") == strlen(code)) {
        char* str = malloc(64);
        snprintf(str, 64, "%.2f", atof(code));
        return str;
    }
    
    return strdup("Error: Could not interpret expression");
}