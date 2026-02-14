"""Project Euler Problem 753: Fermat Equation — embedded C for speedup."""

from __future__ import annotations

import os
import subprocess
import sys
import tempfile

C_CODE = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

/*
 * Sieve primes up to N=6000000, then:
 * 1) For p%3!=1, add (p-1)*(p-2)
 * 2) Enumerate L,M with L^2+27*M^2=4p, p prime, add (L+p-8)*(p-1)
 * Answer fits in unsigned long long.
 */

#define N 6000000

static char is_prime_arr[N + 1];

void sieve(void) {
    memset(is_prime_arr, 1, sizeof(is_prime_arr));
    is_prime_arr[0] = is_prime_arr[1] = 0;
    for (int i = 2; (long long)i * i <= N; i++) {
        if (is_prime_arr[i]) {
            for (int j = i * i; j <= N; j += i)
                is_prime_arr[j] = 0;
        }
    }
}

int trial_is_prime(long long n) {
    if (n < 2) return 0;
    if (n <= N) return is_prime_arr[n];
    if (n % 2 == 0) return 0;
    for (long long i = 3; i * i <= n; i += 2) {
        if (n % i == 0) return 0;
    }
    return 1;
}

int main(void) {
    sieve();

    unsigned long long ans = 0;

    /* Case 1: p % 3 != 1 */
    for (int p = 2; p <= N; p++) {
        if (is_prime_arr[p] && p % 3 != 1) {
            ans += (unsigned long long)(p - 1) * (unsigned long long)(p - 2);
        }
    }

    /* Case 2: enumerate L, M with L^2 + 27*M^2 = 4p, L ≡ 1 (mod 3) */
    int max_abs_l = (int)sqrt(4.0 * N) + 1;
    for (int abs_l = 1; abs_l <= max_abs_l; abs_l++) {
        int signs[2] = {-abs_l, abs_l};
        for (int si = 0; si < 2; si++) {
            int L = signs[si];
            /* L % 3 must be 1. In C, (-2)%3 can be -2, so normalize. */
            int lmod3 = ((L % 3) + 3) % 3;
            if (lmod3 != 1) continue;

            int M_start = abs_l % 2;  /* same parity as L */
            for (int M = M_start; ; M += 2) {
                long long p_val = ((long long)L * L + 27LL * M * M) / 4;
                if (p_val > N) break;
                if (p_val >= 2 && is_prime_arr[p_val]) {
                    ans += (unsigned long long)(L + p_val - 8) * (unsigned long long)(p_val - 1);
                }
            }
        }
    }

    printf("%llu\n", ans);
    return 0;
}
"""


def main() -> int:
    """Main entry point."""
    with tempfile.NamedTemporaryFile(suffix=".c", mode="w", delete=False) as f:
        f.write(C_CODE)
        c_path = f.name
    bin_path = c_path.replace(".c", "")
    try:
        subprocess.run(
            ["gcc", "-O2", "-o", bin_path, c_path, "-lm"],
            check=True, capture_output=True,
        )
        result = subprocess.run(
            [bin_path],
            capture_output=True, text=True, timeout=280,
        )
        answer = int(result.stdout.strip())
        print(answer)
        return answer
    finally:
        for p in [c_path, bin_path]:
            if os.path.exists(p):
                os.unlink(p)


if __name__ == "__main__":
    main()
