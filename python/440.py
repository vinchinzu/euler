"""Project Euler Problem 440 — GCD and Tiling.

Let T(n) be the number of ways to tile a board of length n using 10 distinct
types of 1x1 blocks or 1x2 blocks. Find sum_{1<=a,b,c<=N} GCD(T(c^a), T(c^b)).

Uses the identity GCD(T(x), T(y)) = T(GCD(x+1,y+1) - 1) and matrix exponentiation.
N=2000, K=10, M=987898789.
Ported to embedded C for speed.
"""
import subprocess, tempfile, os

C_CODE = r'''
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

#define NN 2000
#define KK 10
#define M 987898789LL

/* 2x2 matrix: [a, b, c, d] = [[a,b],[c,d]] */
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
    Mat result = {1, 0, 0, 1}; /* identity */
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

    /* Compute mults[g] */
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
        /* mults[0] pairs: GCD is 2 if c odd, 1 if c even */
        /* T(2-1)=T(1), T(1-1)=T(0) */
        /* T(0) = 1, T(1) = K = 10 */
        ans = (ans + mults[0] * (c % 2 == 0 ? 1 : KK)) % M;

        /* For each g>=1, accumulate A = A^c (starting from [[K,1],[1,0]]) */
        /* After g iterations, A = [[K,1],[1,0]]^(c^g) */
        /* T(c^g - 1) = A[0][0] ... wait, T(n) = top-left of A^n */
        /* Actually T(c^g) is not directly what we need. */
        /* We need T(GCD(c^a+1, c^b+1) - 1). For the case where a/g, b/g both odd,
           GCD(c^a+1, c^b+1) = c^g + 1, so we need T(c^g + 1 - 1) = T(c^g). */
        /* T(n) = top-left entry of [[K,1],[1,0]]^n */
        Mat A = {KK, 1, 1, 0};
        for (g = 1; g <= NN; g++) {
            A = mat_pow(A, c);
            /* A is now [[K,1],[1,0]]^(c^g), T(c^g) = A.a */
            ans = (ans + mults[g] * A.a) % M;
        }
    }

    printf("%lld\n", ans);
    return 0;
}
'''

if __name__ == "__main__":
    with tempfile.NamedTemporaryFile(suffix='.c', delete=False) as f:
        f.write(C_CODE.encode())
        c_file = f.name
    exe = c_file[:-2]
    try:
        subprocess.run(['gcc', '-O2', '-o', exe, c_file, '-lm'], check=True, capture_output=True)
        result = subprocess.run([exe], capture_output=True, text=True, timeout=280)
        print(result.stdout.strip())
    finally:
        os.unlink(c_file)
        if os.path.exists(exe):
            os.unlink(exe)
