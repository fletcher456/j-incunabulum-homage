#ifndef MODERN_J_H
#define MODERN_J_H

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

/**
 * Modern J Interpreter Header
 * Contains type definitions and function declarations
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

/* Function declarations */
I* ma(I n);
void mv(I *d, I *s, I n);
I tr(I r, I *d);
A ga(I t, I r, I *d);
A iota(A w);
A plus(A a, A w);
A from(A a, A w);
A box(A w);
A cat(A a, A w);
A find(A a, A w);
A rsh(A a, A w);
A sha(A w);
A id(A w);
A size(A w);
void pi(I i);
void nl(void);
void pr(A w);
int qp(I a);
int qv(I a);
A ex(I *e);
A noun(C c);
I verb(C c);
I* wd(C *s);

/* Main interface function */
char* execute_j_code(const char *code);

#endif /* MODERN_J_H */