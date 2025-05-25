#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include "modern_j.h"

/* 
 * Test harness for modern_j.c
 * This file tests each function up to line 251 of modern_j.c
 */

/* Utility functions for testing */
void print_test_header(const char* test_name) {
    printf("\n====== TESTING %s ======\n", test_name);
}

void print_test_result(const char* test_name, int passed) {
    printf("%s: %s\n", test_name, passed ? "PASSED" : "FAILED");
}

void print_array(A arr) {
    if (!arr) {
        printf("NULL array\n");
        return;
    }
    
    printf("Array: type=%ld, rank=%ld, dimensions=[", arr->t, arr->r);
    for (int i = 0; i < arr->r; i++) {
        printf("%ld", arr->d[i]);
        if (i < arr->r - 1) printf(", ");
    }
    printf("], values=[");
    
    I n = tr(arr->r, arr->d);
    for (I i = 0; i < n; i++) {
        printf("%ld", arr->p[i]);
        if (i < n - 1) printf(", ");
    }
    printf("]\n");
}

/* Test cases */

/* Test 1: Memory allocation */
void test_ma() {
    print_test_header("ma (Memory Allocation)");
    
    I* mem = ma(5);
    int test_passed = (mem != NULL);
    print_test_result("Memory allocation", test_passed);
    
    /* Free the allocated memory */
    free(mem);
}

/* Test 2: Memory copy */
void test_mv() {
    print_test_header("mv (Memory Copy)");
    
    I source[5] = {1, 2, 3, 4, 5};
    I dest[5] = {0, 0, 0, 0, 0};
    
    mv(dest, source, 5);
    
    int test_passed = 1;
    for (int i = 0; i < 5; i++) {
        if (dest[i] != source[i]) {
            test_passed = 0;
            break;
        }
    }
    
    print_test_result("Memory copy", test_passed);
}

/* Test 3: Total size calculation */
void test_tr() {
    print_test_header("tr (Total Size Calculation)");
    
    I dims1[1] = {5};
    I dims2[2] = {2, 3};
    I dims3[3] = {2, 3, 4};
    
    I total1 = tr(1, dims1);
    I total2 = tr(2, dims2);
    I total3 = tr(3, dims3);
    
    printf("tr(1, [5]) = %ld (expected 5)\n", total1);
    printf("tr(2, [2,3]) = %ld (expected 6)\n", total2);
    printf("tr(3, [2,3,4]) = %ld (expected 24)\n", total3);
    
    int test_passed = (total1 == 5 && total2 == 6 && total3 == 24);
    print_test_result("Total size calculation", test_passed);
}

/* Test 4: Array creation */
void test_ga() {
    print_test_header("ga (Array Creation)");
    
    I dims[2] = {2, 3};
    A arr = ga(0, 2, dims);
    
    printf("Created array:\n");
    print_array(arr);
    
    int test_passed = (arr != NULL && arr->t == 0 && arr->r == 2 &&
                       arr->d[0] == 2 && arr->d[1] == 3);
    
    print_test_result("Array creation", test_passed);
    
    /* Free the allocated array */
    free(arr);
}

/* Test 5: Iota function */
void test_iota() {
    print_test_header("iota (Create sequence [0,1,...,n-1])");
    
    /* Create an array with a single value 5 */
    I dims[1] = {1};
    A n_arr = ga(0, 1, dims);
    n_arr->p[0] = 5;
    
    /* Apply iota to get [0,1,2,3,4] */
    A result = iota(n_arr);
    
    printf("Iota(5) result:\n");
    print_array(result);
    
    int test_passed = (result != NULL && result->r == 1 && result->d[0] == 5);
    if (test_passed) {
        for (I i = 0; i < 5; i++) {
            if (result->p[i] != i) {
                test_passed = 0;
                break;
            }
        }
    }
    
    print_test_result("Iota function", test_passed);
    
    /* Free allocated arrays */
    free(n_arr);
    free(result);
}

/* Test 6: Plus function */
void test_plus() {
    print_test_header("plus (Element-wise addition)");
    
    /* Create two arrays: [1,2,3] and [4,5,6] */
    I dims[1] = {3};
    A arr1 = ga(0, 1, dims);
    A arr2 = ga(0, 1, dims);
    
    arr1->p[0] = 1; arr1->p[1] = 2; arr1->p[2] = 3;
    arr2->p[0] = 4; arr2->p[1] = 5; arr2->p[2] = 6;
    
    /* Add them to get [5,7,9] */
    A result = plus(arr1, arr2);
    
    printf("Plus result: [1,2,3] + [4,5,6]\n");
    print_array(result);
    
    int test_passed = (result != NULL && result->r == 1 && result->d[0] == 3);
    if (test_passed) {
        test_passed = (result->p[0] == 5 && result->p[1] == 7 && result->p[2] == 9);
    }
    
    print_test_result("Plus function", test_passed);
    
    /* Free allocated arrays */
    free(arr1);
    free(arr2);
    free(result);
}

/* Test 7: From function */
void test_from() {
    print_test_header("from (Extract elements)");
    
    /* Create an index array with value 1 */
    I idx_dims[1] = {1};
    A idx = ga(0, 1, idx_dims);
    idx->p[0] = 1;
    
    /* Create a matrix [[1,2,3], [4,5,6]] */
    I mat_dims[2] = {2, 3};
    A mat = ga(0, 2, mat_dims);
    mat->p[0] = 1; mat->p[1] = 2; mat->p[2] = 3;
    mat->p[3] = 4; mat->p[4] = 5; mat->p[5] = 6;
    
    /* Extract the second row [4,5,6] */
    A result = from(idx, mat);
    
    printf("From result: extract row 1 from [[1,2,3], [4,5,6]]\n");
    print_array(result);
    
    int test_passed = (result != NULL && result->r == 1 && result->d[0] == 3);
    if (test_passed) {
        test_passed = (result->p[0] == 4 && result->p[1] == 5 && result->p[2] == 6);
    }
    
    print_test_result("From function", test_passed);
    
    /* Free allocated arrays */
    free(idx);
    free(mat);
    free(result);
}

/* Test 8: Box function */
void test_box() {
    print_test_header("box (Create scalar box)");
    
    /* Create an array [1,2,3] */
    I dims[1] = {3};
    A arr = ga(0, 1, dims);
    arr->p[0] = 1; arr->p[1] = 2; arr->p[2] = 3;
    
    /* Box it */
    A result = box(arr);
    
    printf("Box result: box([1,2,3])\n");
    printf("Boxed array: type=%ld, rank=%ld\n", result->t, result->r);
    printf("Box content address: %p\n", (void*)(*result->p));
    
    int test_passed = (result != NULL && result->t == 1 && result->r == 0);
    if (test_passed) {
        A inner = (A)(*result->p);
        test_passed = (inner == arr);
    }
    
    print_test_result("Box function", test_passed);
    
    /* Free allocated arrays */
    free(arr);
    free(result);
}

/* Test 9: Concatenate function */
void test_cat() {
    print_test_header("cat (Concatenate arrays)");
    
    /* Create two arrays: [1,2] and [3,4,5] */
    I dims1[1] = {2};
    I dims2[1] = {3};
    A arr1 = ga(0, 1, dims1);
    A arr2 = ga(0, 1, dims2);
    
    arr1->p[0] = 1; arr1->p[1] = 2;
    arr2->p[0] = 3; arr2->p[1] = 4; arr2->p[2] = 5;
    
    /* Concatenate them to get [1,2,3,4,5] */
    A result = cat(arr1, arr2);
    
    printf("Cat result: [1,2] concatenated with [3,4,5]\n");
    print_array(result);
    
    int test_passed = (result != NULL && result->r == 1 && result->d[0] == 5);
    if (test_passed) {
        test_passed = (result->p[0] == 1 && result->p[1] == 2 && 
                       result->p[2] == 3 && result->p[3] == 4 && result->p[4] == 5);
    }
    
    print_test_result("Cat function", test_passed);
    
    /* Free allocated arrays */
    free(arr1);
    free(arr2);
    free(result);
}

/* Test 10: Reshape function */
void test_rsh() {
    print_test_header("rsh (Reshape array)");
    
    /* Create shape array [2,2] */
    I shape_dims[1] = {2};
    A shape = ga(0, 1, shape_dims);
    shape->p[0] = 2; shape->p[1] = 2;
    
    /* Create data array [1,2,3] */
    I data_dims[1] = {3};
    A data = ga(0, 1, data_dims);
    data->p[0] = 1; data->p[1] = 2; data->p[2] = 3;
    
    /* Reshape to 2x2 matrix [[1,2], [3,1]] */
    A result = rsh(shape, data);
    
    printf("Reshape result: reshape [1,2,3] to [2,2]\n");
    print_array(result);
    
    int test_passed = (result != NULL && result->r == 2 && 
                      result->d[0] == 2 && result->d[1] == 2);
    if (test_passed) {
        test_passed = (result->p[0] == 1 && result->p[1] == 2 && 
                       result->p[2] == 3 && result->p[3] == 1);
    }
    
    print_test_result("Reshape function", test_passed);
    
    /* Free allocated arrays */
    free(shape);
    free(data);
    free(result);
}

/* Test 11: Shape function */
void test_sha() {
    print_test_header("sha (Get array shape)");
    
    /* Create a 2x3 matrix */
    I dims[2] = {2, 3};
    A mat = ga(0, 2, dims);
    
    /* Get its shape [2,3] */
    A result = sha(mat);
    
    printf("Shape result: shape of 2x3 matrix\n");
    print_array(result);
    
    int test_passed = (result != NULL && result->r == 1 && result->d[0] == 2);
    if (test_passed) {
        test_passed = (result->p[0] == 2 && result->p[1] == 3);
    }
    
    print_test_result("Shape function", test_passed);
    
    /* Free allocated arrays */
    free(mat);
    free(result);
}

/* Test 12: Identity function */
void test_id() {
    print_test_header("id (Identity function)");
    
    /* Create an array [1,2,3] */
    I dims[1] = {3};
    A arr = ga(0, 1, dims);
    arr->p[0] = 1; arr->p[1] = 2; arr->p[2] = 3;
    
    /* Apply identity */
    A result = id(arr);
    
    printf("Identity result: should be same as input\n");
    printf("Original address: %p, Result address: %p\n", (void*)arr, (void*)result);
    
    int test_passed = (result == arr);
    
    print_test_result("Identity function", test_passed);
    
    /* Free allocated array */
    free(arr);
}

/* Test 13: Size function */
void test_size() {
    print_test_header("size (Get first dimension)");
    
    /* Create a 2x3 matrix */
    I dims[2] = {2, 3};
    A mat = ga(0, 2, dims);
    
    /* Get its size (first dimension = 2) */
    A result = size(mat);
    
    printf("Size result: first dimension of 2x3 matrix\n");
    print_array(result);
    
    int test_passed = (result != NULL && result->r == 0 && result->p[0] == 2);
    
    print_test_result("Size function", test_passed);
    
    /* Free allocated arrays */
    free(mat);
    free(result);
}

/* Test 14: Variable check function */
void test_qp() {
    print_test_header("qp (Check if character is a variable)");
    
    int test_a = qp('a');
    int test_z = qp('z');
    int test_A = qp('A');
    int test_1 = qp('1');
    
    printf("qp('a') = %d (expected 1)\n", test_a);
    printf("qp('z') = %d (expected 1)\n", test_z);
    printf("qp('A') = %d (expected 0)\n", test_A);
    printf("qp('1') = %d (expected 0)\n", test_1);
    
    int test_passed = (test_a && test_z && !test_A && !test_1);
    
    print_test_result("Variable check function", test_passed);
}

/* Test 15: Verb check function */
void test_qv() {
    print_test_header("qv (Check if character is a verb)");
    
    int test_plus = qv('+');
    int test_a = qv('a');
    
    printf("qv('+') = %d (expected 1)\n", test_plus);
    printf("qv('a') = %d (expected 0)\n", test_a);
    
    int test_passed = (test_plus && !test_a);
    
    print_test_result("Verb check function", test_passed);
}

/* Test 16: Noun parsing */
void test_noun() {
    print_test_header("noun (Parse numeric literal)");
    
    A result1 = noun('5');
    A result2 = noun('a');
    
    printf("noun('5') result:\n");
    if (result1) {
        print_array(result1);
    } else {
        printf("NULL (unexpected)\n");
    }
    
    printf("noun('a') result: %p (expected NULL)\n", (void*)result2);
    
    int test_passed = (result1 != NULL && result1->r == 0 && result1->p[0] == 5 && result2 == NULL);
    
    print_test_result("Noun parsing", test_passed);
    
    /* Free allocated array */
    if (result1) free(result1);
}

/* Test 17: Verb parsing */
void test_verb() {
    print_test_header("verb (Parse verb character)");
    
    I result_plus = verb('+');
    I result_a = verb('a');
    
    printf("verb('+') = %ld (expected > 0)\n", result_plus);
    printf("verb('a') = %ld (expected 0)\n", result_a);
    
    int test_passed = (result_plus > 0 && result_a == 0);
    
    print_test_result("Verb parsing", test_passed);
}

/* Test 18: Word parsing */
void test_wd() {
    print_test_header("wd (Tokenize J expression)");
    
    C expr[] = "1+2";
    I* result = wd(expr);
    
    printf("wd(\"1+2\") results:\n");
    printf("result[0] (type): %ld\n", (I)((A)result[0])->t);
    printf("result[0] (value): %ld\n", (I)((A)result[0])->p[0]);
    printf("result[1] (verb): %ld\n", result[1]);
    printf("result[2] (type): %ld\n", (I)((A)result[2])->t);
    printf("result[2] (value): %ld\n", (I)((A)result[2])->p[0]);
    
    int test_passed = (result != NULL && 
                       ((A)result[0])->p[0] == 1 && 
                       result[1] == verb('+') && 
                       ((A)result[2])->p[0] == 2);
    
    print_test_result("Word parsing", test_passed);
    
    /* Free allocated memory */
    free(result);
}

/* Main test runner */
int main() {
    printf("==================================\n");
    printf("RUNNING TESTS FOR MODERN J INTERPRETER\n");
    printf("==================================\n");
    
    /* Run all tests */
    test_ma();
    test_mv();
    test_tr();
    test_ga();
    test_iota();
    test_plus();
    test_from();
    test_box();
    test_cat();
    test_rsh();
    test_sha();
    test_id();
    test_size();
    test_qp();
    test_qv();
    test_noun();
    test_verb();
    test_wd();
    
    printf("\n==================================\n");
    printf("ALL TESTS COMPLETED\n");
    printf("==================================\n");
    
    return 0;
}