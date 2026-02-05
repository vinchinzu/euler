"""Project Euler Problem 454: Solutions to 1/x + 1/y = 1/n."""

import subprocess
import tempfile
import os

def solve():
    # Direct port from Java
    c_code = r'''
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <math.h>

typedef int64_t i64;

#define N 1000000000000LL  // 10^12

int L;
signed char *mobius;

void pre_mobius(int limit) {
    mobius = calloc(limit + 1, 1);
    for (int i = 0; i <= limit; i++) mobius[i] = 1;
    char *is_prime = calloc(limit + 2, 1);
    for (int i = 0; i <= limit + 1; i++) is_prime[i] = 1;
    is_prime[0] = is_prime[1] = 0;

    for (int i = 2; i <= limit; i++) {
        if (is_prime[i]) {
            for (int j = i; j <= limit; j += i) is_prime[j] = 0;
            for (i64 j = (i64)i * i; j <= limit; j += (i64)i * i) mobius[j] = 0;
            for (int j = i; j <= limit; j += i) mobius[j] = -mobius[j];
        }
    }
    free(is_prime);
}

i64 sq(i64 n) { return n * n; }
i64 cb(i64 n) { return n * n * n; }

int main() {
    L = (int)sqrtl((long double)N) + 1;
    pre_mobius(L + 10);

    i64 ans = 0;

    // From Java:
    // for (int g = 1; g <= L; g++) {
    //     long n = N / sq(g);
    //     for (int y = 2; sq(y) <= n; y++)
    //         if (cb(y) <= n)
    //             for (int x = 1; x < y; x++)
    //                 ans += mobius[g] * n / y / (x + y);
    //         else
    //             for (int q = (int) Math.max(n / y / (2 * y - 1), 1); true; q++) {
    //                 long upper = Math.min(n / y / q, 2 * y - 1);
    //                 long lower = Math.max(n / y / (q + 1), y);
    //                 ans += mobius[g] * (upper - lower) * q;
    //                 if (lower == y)
    //                     break;
    //             }
    // }

    for (int g = 1; g <= L; g++) {
        if (mobius[g] == 0) continue;
        i64 n = N / sq(g);

        for (i64 y = 2; sq(y) <= n; y++) {
            if (cb(y) <= n) {
                // Brute force for small y
                for (i64 x = 1; x < y; x++) {
                    ans += mobius[g] * (n / y / (x + y));
                }
            } else {
                // For large y, group by quotient
                i64 start_q = n / y / (2 * y - 1);
                if (start_q < 1) start_q = 1;
                for (i64 q = start_q; ; q++) {
                    i64 upper = n / y / q;
                    if (upper > 2 * y - 1) upper = 2 * y - 1;
                    i64 lower = n / y / (q + 1);
                    if (lower < y) lower = y;
                    ans += mobius[g] * (upper - lower) * q;
                    if (lower == y) break;
                }
            }
        }
    }

    printf("%lld\n", (long long)ans);
    return 0;
}
'''

    with tempfile.NamedTemporaryFile(suffix='.c', delete=False) as f:
        f.write(c_code.encode())
        c_file = f.name

    exe = c_file[:-2]
    subprocess.run(['gcc', '-O3', '-march=native', '-o', exe, c_file, '-lm'], check=True, capture_output=True)
    result = subprocess.check_output([exe]).decode().strip()
    os.unlink(c_file)
    os.unlink(exe)
    return int(result)

if __name__ == "__main__":
    print(solve())
