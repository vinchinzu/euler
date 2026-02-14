/* Project Euler Problem 940 - Two-Dimensional Recurrence
 * A(0,0)=0, A(0,1)=1
 * A(m+1,n) = A(m,n+1) + A(m,n)
 * A(m+1,n+1) = 2*A(m+1,n) + A(m,n)
 * S(k) = sum_{i=2}^{k} sum_{j=2}^{k} A(f_i, f_j) mod 1123581313
 * Uses matrix exponentiation for the n-advancement.
 */
#include <stdio.h>
#include <string.h>

typedef long long ll;
typedef __int128 lll;

#define MOD 1123581313LL

/* 3x3 matrix operations */
typedef struct { ll m[3][3]; } Mat3;
typedef struct { ll m[2][2]; } Mat2;

Mat3 mat3_mult(Mat3 A, Mat3 B) {
    Mat3 C;
    for (int i = 0; i < 3; i++)
        for (int j = 0; j < 3; j++) {
            lll s = 0;
            for (int k = 0; k < 3; k++)
                s += (lll)A.m[i][k] * B.m[k][j];
            C.m[i][j] = (ll)(s % MOD);
        }
    return C;
}

Mat3 mat3_pow(Mat3 M, ll *exp_digits, int ndigits) {
    /* exp is a big number stored as array of bits (binary, LSB first) */
    /* For simplicity, use standard binary exponentiation with big exponent */
    /* Actually, Fibonacci numbers can be huge. We need big number exponentiation. */
    /* We'll process bit by bit from MSB to LSB */
    Mat3 res;
    memset(&res, 0, sizeof(res));
    for (int i = 0; i < 3; i++) res.m[i][i] = 1; /* identity */

    for (int i = ndigits - 1; i >= 0; i--) {
        res = mat3_mult(res, res);
        if (exp_digits[i]) {
            res = mat3_mult(res, M);
        }
    }
    return res;
}

Mat2 mat2_mult(Mat2 A, Mat2 B) {
    Mat2 C;
    for (int i = 0; i < 2; i++)
        for (int j = 0; j < 2; j++) {
            lll s = 0;
            for (int k = 0; k < 2; k++)
                s += (lll)A.m[i][k] * B.m[k][j];
            C.m[i][j] = (ll)(s % MOD);
        }
    return C;
}

Mat2 mat2_pow(Mat2 M, ll *exp_digits, int ndigits) {
    Mat2 res;
    memset(&res, 0, sizeof(res));
    for (int i = 0; i < 2; i++) res.m[i][i] = 1;

    for (int i = ndigits - 1; i >= 0; i--) {
        res = mat2_mult(res, res);
        if (exp_digits[i]) {
            res = mat2_mult(res, M);
        }
    }
    return res;
}

/* Big Fibonacci numbers (exact) for indices up to 50 */
/* fib[50] has about 10 digits. Actually fib(50) = 12586269025.
   But we need the binary representation for matrix exponentiation. */

/* Store Fibonacci as array of binary digits (LSB first) */
/* fib(50) ~ 1.26e10, fits in ll */
#define MAXK 51
static ll fib[MAXK];

void compute_fib(int k) {
    fib[0] = 0; fib[1] = 1;
    for (int i = 2; i <= k; i++)
        fib[i] = fib[i-1] + fib[i-2];
}

/* Convert ll to binary digits array (LSB first), return count */
int to_binary(ll n, ll *bits) {
    if (n == 0) { bits[0] = 0; return 1; }
    int cnt = 0;
    while (n > 0) {
        bits[cnt++] = n & 1;
        n >>= 1;
    }
    return cnt;
}

int main(void) {
    int k = 50;
    compute_fib(k);

    /* a_mat for A(m,0) recurrence: a_m = 3*a_{m-1} + a_{m-2} */
    Mat2 a_mat = {{{3, 1}, {1, 0}}};

    /* M for advancing n: state = [A(m,n), A(m,n+1), A(m+1,n)] */
    /* From the recurrences:
       A(m,n+2) = A(m,n) - A(m,n+1) + 2*A(m+1,n)  (derived)
       A(m+1,n+1) = 2*A(m+1,n) + A(m,n)
       New state [A(m,n+1), A(m,n+2), A(m+1,n+1)] = M * [A(m,n), A(m,n+1), A(m+1,n)]
       M = [[0, 1, 0],
            [1, -1, 2],
            [1, 0, 2]]
    */
    Mat3 M;
    M.m[0][0] = 0; M.m[0][1] = 1; M.m[0][2] = 0;
    M.m[1][0] = 1; M.m[1][1] = MOD - 1; M.m[1][2] = 2;
    M.m[2][0] = 1; M.m[2][1] = 0; M.m[2][2] = 2;

    /* Precompute initials for each i=2..k */
    /* V0_i = [A(f_i, 0), A(f_i, 1), A(f_i+1, 0)] */
    ll initials[MAXK][3];

    for (int ii = 2; ii <= k; ii++) {
        ll mm = fib[ii];
        ll bits[64];
        int nbits;

        if (mm <= 1) {
            if (mm == 0) {
                initials[ii][0] = 0; /* A(0,0)=0 */
                initials[ii][1] = 1; /* A(0,1)=1 */
                initials[ii][2] = 1; /* A(1,0)=1 */
            } else {
                initials[ii][0] = 1; /* A(1,0)=1 */
                initials[ii][1] = 2; /* A(1,1)=2 */
                initials[ii][2] = 3; /* A(2,0)=3 */
            }
        } else {
            /* Compute [a_mm, a_{mm-1}] = a_mat^(mm-1) * [1, 0] */
            nbits = to_binary(mm - 1, bits);
            Mat2 pow_a = mat2_pow(a_mat, bits, nbits);
            ll a_m = (pow_a.m[0][0] * 1 + pow_a.m[0][1] * 0) % MOD;
            ll a_mm1 = (pow_a.m[1][0] * 1 + pow_a.m[1][1] * 0) % MOD;
            ll a_mp1 = (3 * a_m + a_mm1) % MOD;
            ll b_m = (2 * a_m + a_mm1) % MOD;
            initials[ii][0] = a_m;
            initials[ii][1] = b_m;
            initials[ii][2] = a_mp1;
        }
    }

    ll total = 0;
    for (int i = 2; i <= k; i++) {
        for (int j = 2; j <= k; j++) {
            ll n_val = fib[j];
            ll res;
            if (n_val == 0) {
                res = initials[i][0];
            } else if (n_val == 1) {
                res = initials[i][1];
            } else {
                ll bits[64];
                int nbits = to_binary(n_val, bits);
                Mat3 pow_m = mat3_pow(M, bits, nbits);
                /* Vn = pow_m * V0 */
                lll r0 = 0;
                for (int c = 0; c < 3; c++)
                    r0 += (lll)pow_m.m[0][c] * initials[i][c];
                res = (ll)(r0 % MOD);
            }
            total = (total + res) % MOD;
        }
    }

    printf("%lld\n", total);
    return 0;
}
