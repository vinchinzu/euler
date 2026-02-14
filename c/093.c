#include <stdio.h>
#include <math.h>
#include <stdbool.h>
#include <string.h>

/* Results tracking using a bit array for positive integers up to ~10000 */
#define MAX_RESULT 10000
static unsigned char results[MAX_RESULT / 8 + 1];

void results_clear(void) { memset(results, 0, sizeof(results)); }
void results_add(int n) {
    if (n > 0 && n < MAX_RESULT)
        results[n >> 3] |= (1 << (n & 7));
}
bool results_has(int n) {
    if (n > 0 && n < MAX_RESULT)
        return (results[n >> 3] >> (n & 7)) & 1;
    return false;
}

double apply_op(double a, int op, double b) {
    switch (op) {
        case 0: return a + b;
        case 1: return a - b;
        case 2: return a * b;
        case 3: return (b != 0.0) ? a / b : 1e18;
    }
    return 1e18;
}

bool is_pos_int(double val) {
    return isfinite(val) && val > 0.0 && fabs(val - floor(val + 0.5)) < 1e-7;
}

void try_add(double val) {
    if (is_pos_int(val))
        results_add((int)(val + 0.5));
}

void evaluate_all(int *p, int op0, int op1, int op2) {
    double a = p[0], b = p[1], c = p[2], d = p[3];
    double v, v1, v2;

    /* ((a op0 b) op1 c) op2 d */
    v = apply_op(a, op0, b);
    v = apply_op(v, op1, c);
    v = apply_op(v, op2, d);
    try_add(v);

    /* (a op0 (b op1 c)) op2 d */
    v = apply_op(b, op1, c);
    v = apply_op(a, op0, v);
    v = apply_op(v, op2, d);
    try_add(v);

    /* a op0 ((b op1 c) op2 d) */
    v = apply_op(b, op1, c);
    v = apply_op(v, op2, d);
    v = apply_op(a, op0, v);
    try_add(v);

    /* a op0 (b op1 (c op2 d)) */
    v = apply_op(c, op2, d);
    v = apply_op(b, op1, v);
    v = apply_op(a, op0, v);
    try_add(v);

    /* (a op0 b) op1 (c op2 d) */
    v1 = apply_op(a, op0, b);
    v2 = apply_op(c, op2, d);
    v = apply_op(v1, op1, v2);
    try_add(v);
}

/* Generate all permutations */
void swap(int *a, int *b) { int t = *a; *a = *b; *b = t; }

void permute(int *arr, int l, int r) {
    if (l == r) {
        for (int i = 0; i < 4; i++)
            for (int j = 0; j < 4; j++)
                for (int k = 0; k < 4; k++)
                    evaluate_all(arr, i, j, k);
        return;
    }
    for (int i = l; i <= r; i++) {
        swap(&arr[l], &arr[i]);
        permute(arr, l + 1, r);
        swap(&arr[l], &arr[i]);
    }
}

int find_max_consecutive(void) {
    int n = 0;
    while (results_has(n + 1)) n++;
    return n;
}

int main(void) {
    int max_n = 0;
    int best[4] = {0, 0, 0, 0};

    for (int a = 0; a <= 9; a++)
        for (int b = a + 1; b <= 9; b++)
            for (int c = b + 1; c <= 9; c++)
                for (int d = c + 1; d <= 9; d++) {
                    results_clear();
                    int digits[4] = {a, b, c, d};
                    permute(digits, 0, 3);
                    int n = find_max_consecutive();
                    if (n > max_n) {
                        max_n = n;
                        best[0] = a; best[1] = b; best[2] = c; best[3] = d;
                    }
                }

    printf("%d%d%d%d\n", best[0], best[1], best[2], best[3]);
    return 0;
}
