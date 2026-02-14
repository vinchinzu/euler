"""Project Euler Problem 618: Numbers with a given prime factor sum.

Find sum_{k=2}^N S(F_k), where F_k is the kth Fibonacci number, and S(k) is
the sum of all numbers whose prime factors sum to k.

DP over primes: for each prime p <= F(24)=46368, update dp[k] += p * dp[k-p].
Then sum dp[F(i)] for i=2..24.

Embedded C implementation for speed.
"""

from __future__ import annotations

import os
import subprocess
import tempfile

C_CODE = r"""
#include <stdio.h>
#include <string.h>

#define LIMIT 46368
#define MOD 1000000000LL

static int is_prime[LIMIT + 1];
static long long dp[LIMIT + 1];

int main(void) {
    /* Sieve of Eratosthenes */
    memset(is_prime, 1, sizeof(is_prime));
    is_prime[0] = is_prime[1] = 0;
    for (int i = 2; (long long)i * i <= LIMIT; i++) {
        if (is_prime[i]) {
            for (int j = i * i; j <= LIMIT; j += i) {
                is_prime[j] = 0;
            }
        }
    }

    /* DP: dp[0] = 1, for each prime p, dp[k] += p * dp[k-p] */
    dp[0] = 1;
    for (int p = 2; p <= LIMIT; p++) {
        if (!is_prime[p]) continue;
        for (int k = p; k <= LIMIT; k++) {
            dp[k] = (dp[k] + (long long)p * dp[k - p]) % MOD;
        }
    }

    /* Compute Fibonacci numbers F(2)..F(24) and sum dp[F(i)] */
    int fib_prev = 0, fib_curr = 1;
    long long ans = 0;
    for (int i = 1; i <= 24; i++) {
        int tmp = fib_prev + fib_curr;
        fib_prev = fib_curr;
        fib_curr = tmp;
        /* After this iteration, fib_curr = F(i+1), fib_prev = F(i) */
        /* We want F(i) for i=2..24 */
        if (i >= 2) {
            ans = (ans + dp[fib_prev]) % MOD;
        }
    }

    printf("%lld\n", ans);
    return 0;
}
"""


def solve() -> int:
    """Solve Problem 618 using embedded C."""
    with tempfile.NamedTemporaryFile(suffix='.c', mode='w', delete=False) as f:
        f.write(C_CODE)
        c_path = f.name
    bin_path = c_path.replace('.c', '')
    try:
        subprocess.run(['gcc', '-O2', '-o', bin_path, c_path, '-lm'], check=True)
        result = subprocess.run([bin_path], capture_output=True, text=True, timeout=280)
        return int(result.stdout.strip())
    finally:
        for path in (c_path, bin_path):
            if os.path.exists(path):
                os.unlink(path)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
