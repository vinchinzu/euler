/*
 * Project Euler 440 - GCD and Tiling
 *
 * T(n) uses 10 distinct tile types. Find sum_{1<=a,b,c<=N} GCD(T(c^a), T(c^b)).
 * Uses identity GCD(T(x), T(y)) = T(GCD(x+1,y+1) - 1) and matrix exponentiation.
 * Extracted from embedded C in python/440.py.
 */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

#define NN 2000
#define KK 10
#define M 987898789LL

typedef struct { int64_t a, b, c, d; } Mat;

static Mat mat_mul(Mat x, Mat y) {
    Mat r;
    r.a = (x.a * y.a + x.b * y.c) % M;
    r.b = (x.a * y.b + x.b * y.d) % M;
    r.c = (x.c * y.a + x.d * y.c) % M;
    r.d = (x.c * y.b + x.d * y.d) % M;
    return r;
}

static Mat mat_pow(Mat base, int exp) {
    Mat result = {1, 0, 0, 1};
    while (exp > 0) {
        if (exp & 1) result = mat_mul(result, base);
        base = mat_mul(base, base);
        exp >>= 1;
    }
    return result;
}

static int gcd(int a, int b) {
    while (b) { int t = b; b = a % b; a = t; }
    return a;
}

int main(void) {
    int a, b, g;

    int64_t mults[NN + 1];
    for (g = 0; g <= NN; g++) mults[g] = 0;

    for (a = 1; a <= NN; a++) {
        for (b = 1; b <= NN; b++) {
            g = gcd(a, b);
            if ((a / g) % 2 == 1 && (b / g) % 2 == 1)
                mults[g]++;
            else
                mults[0]++;
        }
    }

    int64_t ans = 0;
    int c;
    for (c = 1; c <= NN; c++) {
        ans = (ans + mults[0] * (c % 2 == 0 ? 1 : KK)) % M;

        Mat A = {KK, 1, 1, 0};
        for (g = 1; g <= NN; g++) {
            A = mat_pow(A, c);
            ans = (ans + mults[g] * A.a) % M;
        }
    }

    printf("%lld\n", (long long)ans);
    return 0;
}
