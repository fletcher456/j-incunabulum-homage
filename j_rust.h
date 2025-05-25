#ifndef J_RUST_H
#define J_RUST_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Interprets a J language expression and returns the result as a string.
 * 
 * @param input The J code to interpret
 * @return A newly allocated string containing the result.
 *         The caller is responsible for freeing this string with free_string().
 */
char* interpret_j_code(const char* input);

/**
 * Frees a string allocated by interpret_j_code().
 * 
 * @param s The string to free
 */
void free_string(char* s);

#ifdef __cplusplus
}
#endif

#endif /* J_RUST_H */