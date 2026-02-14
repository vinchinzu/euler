/*
 * Project Euler Problem 324 - Building a tower
 *
 * Find f(10^10000) mod 100000007, where f(n) is the number of ways
 * to fill a 3x3xn tower with 2x1x1 blocks.
 *
 * Uses transition matrix with symmetry reduction, then matrix exponentiation.
 * The 3x3 face has 512 bitmasks. With D4 symmetry, reachable states from
 * empty face reduce to 46, squaring trick to 23.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define Q 100000007LL
#define K 3
#define KK 9
#define FULL 511  /* (1<<9)-1 */

typedef long long ll;
typedef __int128 i128;

/* Symmetry operations on a 3x3 grid bitmask */
int rotate(int face) {
    int new = 0;
    for (int i = 0; i < K; i++)
        for (int j = 0; j < K; j++)
            if (face & (1 << (K * i + j)))
                new |= 1 << (K * j + (K - 1 - i));
    return new;
}

int flip(int face) {
    int new = 0;
    for (int i = 0; i < K; i++)
        for (int j = 0; j < K; j++)
            if (face & (1 << (K * i + j)))
                new |= 1 << (K * i + (K - 1 - j));
    return new;
}

/* Compute the canonical representative (min) of a face's symmetry class */
int face_min(int face) {
    int mn = face;
    int f = face;
    for (int r = 0; r < 4; r++) {
        if (f < mn) mn = f;
        int fl = flip(f);
        if (fl < mn) mn = fl;
        f = rotate(f);
    }
    return mn;
}

/* Enumerate how a layer can be filled given a fixed input mask.
 * mask_cur = current filled positions, mask_out = bits pushed to next layer.
 * When mask_cur = FULL, done. */
#define MAX_RESULTS 256
int results[MAX_RESULTS];
int nresults;

void fill_layer(int mask_cur, int mask_out) {
    int inv = (~mask_cur) & FULL;
    if (inv == 0) {
        results[nresults++] = mask_out;
        return;
    }
    int pos = __builtin_ctz(inv);
    /* Place block vertically (into next layer) */
    fill_layer(mask_cur | (1 << pos), mask_out | (1 << pos));

    int row = pos / K, col = pos % K;
    /* Place block horizontally within row */
    int nei = pos + 1;
    if (col + 1 < K && !(mask_cur & (1 << nei)))
        fill_layer(mask_cur | (1 << pos) | (1 << nei), mask_out);
    /* Place block vertically within layer (along row direction) */
    nei = pos + K;
    if (row + 1 < K && !(mask_cur & (1 << nei)))
        fill_layer(mask_cur | (1 << pos) | (1 << nei), mask_out);
}

/* Raw transitions: for each input mask, list of output masks */
int raw_results[512][MAX_RESULTS];
int raw_count[512];

/* Faces and transitions */
#define MAX_FACES 100
int face_rep[MAX_FACES];   /* canonical representative for each face class */
int nfaces;
int face_index[512];       /* maps canonical rep -> face index, -1 if not reachable */

/* Matrix operations mod Q */
typedef struct {
    int n;
    ll *data;
} Matrix;

Matrix mat_alloc(int n) {
    Matrix m;
    m.n = n;
    m.data = (ll *)calloc((size_t)n * n, sizeof(ll));
    return m;
}

void mat_free(Matrix *m) {
    free(m->data);
}

#define MAT(m, i, j) ((m).data[(i) * (m).n + (j)])

Matrix mat_mul(Matrix A, Matrix B) {
    int n = A.n;
    Matrix C = mat_alloc(n);
    for (int i = 0; i < n; i++)
        for (int k = 0; k < n; k++) {
            if (MAT(A, i, k) == 0) continue;
            for (int j = 0; j < n; j++)
                MAT(C, i, j) = (MAT(C, i, j) + (i128)MAT(A, i, k) * MAT(B, k, j)) % Q;
        }
    return C;
}

Matrix mat_eye(int n) {
    Matrix m = mat_alloc(n);
    for (int i = 0; i < n; i++) MAT(m, i, i) = 1;
    return m;
}

/* Big integer exponent: 10^10000 / 2 */
/* We need to do matrix exponentiation with a very large exponent.
 * Store the exponent as a string of decimal digits and process bit-by-bit
 * by converting to binary on the fly. */

/* Instead, we compute 10^10000 mod (order of GL(n, F_Q)).
 * But Q is prime, so we can work directly with the exponent.
 * For matrix exponentiation with huge exponent, we convert the exponent
 * to binary representation. */

/* Actually, we can do repeated squaring with the decimal exponent.
 * mat_pow_decimal(M, digits, len) = M^(digits as decimal number) */
Matrix mat_pow_decimal(Matrix M, const char *digits, int len) {
    int n = M.n;
    Matrix result = mat_eye(n);

    for (int d = 0; d < len; d++) {
        /* result = result^10 * M^digit */
        /* First: result^10 = ((result^2)^2 * result)^2 */
        Matrix r2 = mat_mul(result, result);
        Matrix r4 = mat_mul(r2, r2);
        mat_free(&r2);
        Matrix r5 = mat_mul(r4, result);
        mat_free(&r4);
        Matrix r10 = mat_mul(r5, r5);
        mat_free(&r5);
        mat_free(&result);
        result = r10;

        /* Now multiply by M^digit */
        int digit = digits[d] - '0';
        /* Compute M^digit by repeated squaring */
        if (digit > 0) {
            Matrix base = mat_alloc(n);
            memcpy(base.data, M.data, (size_t)n * n * sizeof(ll));
            Matrix md = mat_eye(n);
            int e = digit;
            while (e > 0) {
                if (e & 1) {
                    Matrix tmp = mat_mul(md, base);
                    mat_free(&md);
                    md = tmp;
                }
                e >>= 1;
                if (e > 0) {
                    Matrix tmp = mat_mul(base, base);
                    mat_free(&base);
                    base = tmp;
                }
            }
            mat_free(&base);
            Matrix tmp = mat_mul(result, md);
            mat_free(&result);
            mat_free(&md);
            result = tmp;
        }
    }
    return result;
}

int main(void) {
    /* Precompute raw transitions */
    for (int mask = 0; mask < (1 << KK); mask++) {
        nresults = 0;
        fill_layer(mask, 0);
        raw_count[mask] = nresults;
        memcpy(raw_results[mask], results, nresults * sizeof(int));
    }

    /* Build symmetry classes using BFS from face 0 */
    memset(face_index, -1, sizeof(face_index));
    nfaces = 0;

    int queue[MAX_FACES];
    int qhead = 0, qtail = 0;

    int rep0 = face_min(0);
    face_rep[nfaces] = rep0;
    face_index[rep0] = nfaces;
    nfaces++;
    queue[qtail++] = rep0;

    /* For transitions, count how many raw transitions go from one face class to another */
    /* trans[i][j] = count */
    int trans[MAX_FACES][MAX_FACES];
    memset(trans, 0, sizeof(trans));

    while (qhead < qtail) {
        int rep = queue[qhead++];
        int fi = face_index[rep];
        int cnt = raw_count[rep];
        for (int r = 0; r < cnt; r++) {
            int ns = raw_results[rep][r];
            int nrep = face_min(ns);
            if (face_index[nrep] == -1) {
                face_rep[nfaces] = nrep;
                face_index[nrep] = nfaces;
                nfaces++;
                queue[qtail++] = nrep;
            }
            trans[face_index[nrep]][fi]++;
        }
    }

    /* Build full transition matrix */
    Matrix A = mat_alloc(nfaces);
    for (int i = 0; i < nfaces; i++)
        for (int j = 0; j < nfaces; j++)
            MAT(A, i, j) = trans[i][j] % Q;

    /* Squaring trick: keep only even-popcount faces */
    int even_faces[MAX_FACES];
    int ne = 0;
    for (int i = 0; i < nfaces; i++)
        if (__builtin_popcount(face_rep[i]) % 2 == 0)
            even_faces[ne++] = i;

    /* A^2 restricted to even-popcount faces */
    Matrix A2 = mat_alloc(ne);
    for (int i = 0; i < ne; i++)
        for (int j = 0; j < ne; j++) {
            ll sum = 0;
            for (int k = 0; k < nfaces; k++)
                sum = (sum + (i128)MAT(A, even_faces[i], k) * MAT(A, k, even_faces[j])) % Q;
            MAT(A2, i, j) = sum;
        }
    mat_free(&A);

    /* Find index of face 0 (empty face) in even_faces */
    int fi0 = face_index[face_min(0)];
    int ei = -1;
    for (int i = 0; i < ne; i++)
        if (even_faces[i] == fi0) { ei = i; break; }

    /* Exponent = 10^10000 / 2 = 5 * 10^9999 */
    /* Build the decimal string for 5 * 10^9999 */
    /* That's "5" followed by 9999 zeros */
    int exp_len = 10000;
    char *exp_str = (char *)malloc(exp_len + 1);
    exp_str[0] = '5';
    for (int i = 1; i < exp_len; i++) exp_str[i] = '0';
    exp_str[exp_len] = '\0';

    Matrix powered = mat_pow_decimal(A2, exp_str, exp_len);

    printf("%lld\n", MAT(powered, ei, ei));

    mat_free(&A2);
    mat_free(&powered);
    free(exp_str);
    return 0;
}
