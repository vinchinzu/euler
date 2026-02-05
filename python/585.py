"""Project Euler Problem 585: Nested square roots."""

import subprocess
import tempfile
import os

def solve():
    c_code = r'''
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <math.h>

typedef int64_t i64;

#define N 5000000

int *ff;  // smallest prime factor
int *phi;
char *is_square_free;

void init() {
    ff = (int*)calloc(N + 1, sizeof(int));
    phi = (int*)calloc(N + 1, sizeof(int));
    is_square_free = (char*)calloc(N + 1, sizeof(char));

    // Sieve for smallest prime factor
    for (int i = 2; i <= N; i++) {
        if (ff[i] == 0) {
            ff[i] = i;
            for (i64 j = (i64)i * i; j <= N; j += i)
                if (ff[j] == 0) ff[j] = i;
        }
    }

    // Compute phi
    phi[1] = 1;
    for (int i = 2; i <= N; i++) {
        int p = ff[i];
        int temp = i;
        int pe = 1;
        while (temp % p == 0) {
            temp /= p;
            pe *= p;
        }
        if (temp == 1) {
            // i = p^e
            phi[i] = pe - pe / p;
        } else {
            phi[i] = phi[pe] * phi[temp];
        }
    }

    // Compute square-free flags
    for (int i = 1; i <= N; i++) is_square_free[i] = 1;
    for (int i = 2; (i64)i * i <= N; i++) {
        for (i64 j = (i64)i * i; j <= N; j += (i64)i * i)
            is_square_free[j] = 0;
    }
}

int gcd(int a, int b) {
    while (b) { int t = b; b = a % b; a = t; }
    return a;
}

int is_sq(i64 n) {
    if (n < 0) return 0;
    i64 r = (i64)sqrt((double)n);
    while (r * r > n) r--;
    while ((r+1) * (r+1) <= n) r++;
    return r * r == n;
}

int main() {
    init();

    int *f = (int*)calloc(N + 1, sizeof(int));
    int *fp = (int*)calloc(N + 1, sizeof(int));

    for (int n = 1; n <= N; n++) {
        f[n] = (n - 1) / 2;
        fp[n] = phi[n] / 2;
    }

    // Subtract cases where a*b is a perfect square
    for (int k = 1; k <= N; k++) {
        if (!is_square_free[k]) continue;
        for (int s = 1; (i64)s * s * k <= N; s++) {
            for (int t = s + 1; ((i64)s * s + (i64)t * t) * k <= N; t++) {
                int sum = (s * s + t * t) * k;
                f[sum]--;
                if (k == 1 && gcd(s, t) == 1)
                    fp[sum]--;
            }
        }
    }

    // First case: sum of f[n]
    i64 ans = 0;
    for (int n = 1; n <= N; n++)
        ans += f[n];

    // Second case
    i64 res = 0;
    for (int g_plus_h = 1; g_plus_h <= N; g_plus_h++) {
        for (int ap_plus_bp = 1; (i64)g_plus_h * ap_plus_bp <= N; ap_plus_bp++) {
            res += (i64)f[g_plus_h] * fp[ap_plus_bp];
        }
    }

    // Compute sizes for each index
    i64 *sizes = (i64*)calloc(N + 1, sizeof(i64));
    // sizes[n] = product of (2*e + 1) for each prime power in factorization
    sizes[1] = 1;
    for (int i = 2; i <= N; i++) {
        int p = ff[i];
        int temp = i;
        int e = 0;
        while (temp % p == 0) {
            temp /= p;
            e++;
        }
        sizes[i] = sizes[temp] * (2 * e + 1);
    }

    int *startIndices = (int*)calloc(N + 2, sizeof(int));
    for (int i = 1; i <= N; i++)
        startIndices[i + 1] = startIndices[i] + (int)((sizes[i] + 1) / 2);

    int totalSize = startIndices[N + 1];
    int *currIndices = (int*)malloc((N + 2) * sizeof(int));
    memcpy(currIndices, startIndices, (N + 2) * sizeof(int));

    i64 *smalls = (i64*)calloc(totalSize, sizeof(i64));
    i64 *bigs = (i64*)calloc(totalSize, sizeof(i64));

    for (int k = 1; k <= N; k++) {
        if (!is_square_free[k]) continue;
        for (int s = 1; (i64)s * s * k <= N; s++) {
            for (int t = s; ((i64)s * s + (i64)t * t) * k <= N; t++) {
                int idx = s * t * k;
                if (idx <= N && currIndices[idx] < startIndices[idx + 1]) {
                    smalls[currIndices[idx]] = (i64)s * s * k;
                    bigs[currIndices[idx]] = (i64)t * t * k;
                    currIndices[idx]++;
                }
            }
        }
    }

    for (int i = 0; i <= N; i++) {
        for (int ad = startIndices[i]; ad < currIndices[i]; ad++) {
            for (int bc = startIndices[i]; bc < currIndices[i]; bc++) {
                i64 a = smalls[ad], b = smalls[bc], c = bigs[bc], d = bigs[ad];
                if (a < b && a + b + c + d <= N && !is_sq(a * b))
                    res -= (b == c) ? 1 : 2;
            }
        }
    }

    ans += res / 2;

    printf("%lld\n", ans);

    free(f); free(fp); free(ff); free(phi); free(is_square_free);
    free(sizes); free(startIndices); free(currIndices);
    free(smalls); free(bigs);

    return 0;
}
'''

    with tempfile.NamedTemporaryFile(suffix='.c', delete=False) as f:
        f.write(c_code.encode())
        c_file = f.name

    exe = c_file[:-2]
    subprocess.run(['gcc', '-O3', '-o', exe, c_file, '-lm'], check=True, capture_output=True)
    result = subprocess.check_output([exe]).decode().strip()

    os.unlink(c_file)
    os.unlink(exe)

    return int(result)

if __name__ == "__main__":
    print(solve())
