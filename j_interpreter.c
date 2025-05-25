#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

/**
 * A minimal J language interpreter implementation
 * Based on an original fragment with improved formatting and structure
 */

/* Type definitions */
typedef char C;
typedef long I;
typedef struct a {
    I t;       /* Type */
    I r;       /* Rank */
    I d[3];    /* Dimensions */
    I p[2];    /* Data pointer/storage */
} *A;          /* Array pointer type */

/* Macros for readability */
#define P printf
#define R return
#define DO(n, x) {I i = 0, _n = (n); for (; i < _n; ++i) { x; }}

/* Forward declarations */
A ex(I *e);
void pr(A w);

/**
 * Memory allocation for arrays
 */
I* ma(I n) {
    return (I*)malloc(n * 4);
}

/**
 * Memory copy function
 */
void mv(I *d, I *s, I n) {
    DO(n, d[i] = s[i]);
}

/**
 * Calculate total size from rank and dimensions
 */
I tr(I r, I *d) {
    I z = 1;
    DO(r, z = z * d[i]);
    return z;
}

/**
 * Create a new array with specified type, rank, and dimensions
 */
A ga(I t, I r, I *d) {
    A z = (A)ma(5 + tr(r, d));
    z->t = t;
    z->r = r;
    mv(z->d, d, r);
    return z;
}

/**
 * Iota function: creates an array containing [0,1,2,...,n-1]
 */
A iota(A w) {
    I n = *w->p;
    A z = ga(0, 1, &n);
    DO(n, z->p[i] = i);
    return z;
}

/**
 * Plus function: element-wise addition of two arrays
 */
A plus(A a, A w) {
    I r = w->r;
    I *d = w->d;
    I n = tr(r, d);
    A z = ga(0, r, d);
    DO(n, z->p[i] = a->p[i] + w->p[i]);
    return z;
}

/**
 * From function: extract elements from an array
 */
A from(A a, A w) {
    I r = w->r - 1;
    I *d = w->d + 1;
    I n = tr(r, d);
    A z = ga(w->t, r, d);
    mv(z->p, w->p + (n * *a->p), n);
    return z;
}

/**
 * Box function: create a scalar box containing an array
 */
A box(A w) {
    A z = ga(1, 0, 0);
    *z->p = (I)w;
    return z;
}

/**
 * Concatenate function: join two arrays
 */
A cat(A a, A w) {
    I an = tr(a->r, a->d);
    I wn = tr(w->r, w->d);
    I n = an + wn;
    A z = ga(w->t, 1, &n);
    mv(z->p, a->p, an);
    mv(z->p + an, w->p, wn);
    return z;
}

/**
 * Find function: locate elements in an array (placeholder)
 */
A find(A a, A w) {
    return 0;  /* Placeholder implementation */
}

/**
 * Reshape function: create a new array with different dimensions
 */
A rsh(A a, A w) {
    I r = a->r ? *a->d : 1;
    I n = tr(r, a->p);
    I wn = tr(w->r, w->d);
    A z = ga(w->t, r, a->p);
    mv(z->p, w->p, wn = n > wn ? wn : n);
    if (n -= wn) {
        mv(z->p + wn, z->p, n);
    }
    return z;
}

/**
 * Shape function: return array dimensions as an array
 */
A sha(A w) {
    A z = ga(0, 1, &w->r);
    mv(z->p, w->d, w->r);
    return z;
}

/**
 * Identity function: return input unchanged
 */
A id(A w) {
    return w;
}

/**
 * Size function: return first dimension or 1
 */
A size(A w) {
    A z = ga(0, 0, 0);
    *z->p = w->r ? *w->d : 1;
    return z;
}

/**
 * Print an integer
 */
void pi(I i) {
    P("%d ", i);
}

/**
 * Print a newline
 */
void nl() {
    P("\n");
}

/**
 * Print an array (recursive)
 */
void pr(A w) {
    I r = w->r;
    I *d = w->d;
    I n = tr(r, d);
    
    DO(r, pi(d[i]));
    nl();
    
    if (w->t) {
        DO(n, P("< "); pr((A)w->p[i]));
    } else {
        DO(n, pi(w->p[i]));
    }
    nl();
}

/* Verb table for parser */
C vt[] = "+{~<#,";

/* Function pointers for dyadic (two-argument) verbs */
A (*vd[])() = {
    0,    /* Placeholder at index 0 */
    plus, /* Addition */
    from, /* From */
    find, /* Find */
    0,    /* Placeholder */
    rsh,  /* Reshape */
    cat   /* Concatenate */
};

/* Function pointers for monadic (one-argument) verbs */
A (*vm[])() = {
    0,     /* Placeholder at index 0 */
    id,    /* Identity */
    size,  /* Size */
    iota,  /* Iota */
    box,   /* Box */
    sha,   /* Shape */
    0      /* Placeholder */
};

/* Symbol table for variables (a-z) */
I st[26];

/**
 * Check if character is a variable name (a-z)
 */
int qp(I a) {
    return a >= 'a' && a <= 'z';
}

/**
 * Check if character is a verb
 */
int qv(I a) {
    return a < 'a';
}

/**
 * Execute J expression
 */
A ex(I *e) {
    I a = *e;
    
    if (qp(a)) {
        if (e[1] == '=') {
            st[a - 'a'] = (I)ex(e + 2);
            return (A)st[a - 'a'];
        }
        a = st[a - 'a'];
    }
    
    return qv(a) ? 
           (*vm[a])(ex(e + 1)) : 
           e[1] ? (*vd[e[1]])((A)a, ex(e + 2)) : (A)a;
}

/**
 * Parse a numeric literal (0-9)
 */
A noun(C c) {
    A z;
    
    if (c < '0' || c > '9') {
        return 0;
    }
    
    z = ga(0, 0, 0);
    *z->p = c - '0';
    return z;
}

/**
 * Parse a verb from the verb table
 */
I verb(C c) {
    I i = 0;
    
    for (; vt[i]; ) {
        if (vt[i++] == c) {
            return i;
        }
    }
    
    return 0;
}

/**
 * Tokenize a string into words (parsed J tokens)
 */
I* wd(C *s) {
    I n = strlen(s);
    I *e = ma(n + 1);
    C c;
    I a_val;
    A a_noun;
    
    I i;
    for (i = 0; i < n; i++) {
        c = s[i];
        a_noun = noun(c);
        
        if (a_noun) {
            // It's a noun (numeric literal)
            e[i] = (I)a_noun;
        } else {
            // Check if it's a verb
            a_val = verb(c);
            if (a_val) {
                e[i] = a_val;
            } else {
                // It's a regular character
                e[i] = (I)c;
            }
        }
    }
    
    e[n] = 0;
    return e;
}

/**
 * Function to execute J code and return result as string
 * This is the main interface for the webserver
 */
char* execute_j_code(const char *code) {
    static char result_buffer[4096];
    memset(result_buffer, 0, sizeof(result_buffer));
    
    // Redirect stdout to capture the output
    FILE* original_stdout = stdout;
    FILE* temp_file = tmpfile();
    if (!temp_file) {
        snprintf(result_buffer, sizeof(result_buffer), "Error: Failed to create temporary file");
        return result_buffer;
    }
    
    stdout = temp_file;
    
    // Execute the J code
    A result = ex(wd((char*)code));
    if (result) {
        pr(result);
    } else {
        P("Error evaluating J expression\n");
    }
    
    // Restore stdout
    fflush(temp_file);
    stdout = original_stdout;
    
    // Read the captured output
    fseek(temp_file, 0, SEEK_SET);
    size_t read_size = fread(result_buffer, 1, sizeof(result_buffer) - 1, temp_file);
    result_buffer[read_size] = '\0';
    
    // Close the temporary file
    fclose(temp_file);
    
    return result_buffer;
}