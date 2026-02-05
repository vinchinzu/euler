"""Project Euler Problem 668: Square Root Smooth Numbers."""

import subprocess
import tempfile
import os

def solve():
    c_code = r'''
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <math.h>

/*
 * Lucy DP to count primes up to each N/d value.
 * We need pi(N/d) for d = 1, 2, ..., L where L = sqrt(N).
 */

typedef long long ll;
typedef __int128 lll;

ll N;
int L;
ll *small_vals;  // small_vals[i] = pi(i) for i <= L
ll *large_vals;  // large_vals[i] = pi(N/i) for i <= L

ll *quotients;
int num_quotients;

void lucy_dp() {
    // Initialize: S(v) = v - 1 (counting all integers >= 2)
    for (int i = 1; i <= L; i++) {
        small_vals[i] = i - 1;
        large_vals[i] = N / i - 1;
    }

    // Sieve by primes up to sqrt(N)
    for (ll p = 2; p <= L; p++) {
        if (small_vals[p] == small_vals[p - 1]) continue; // p not prime

        ll pi_p_minus_1 = small_vals[p - 1];
        ll p2 = p * p;

        // Update large values
        for (int i = 1; i <= L && N / i >= p2; i++) {
            ll v = N / i;
            ll v_div_p = v / p;
            ll sub;
            if (v_div_p <= L)
                sub = small_vals[v_div_p];
            else
                sub = large_vals[N / v_div_p];
            large_vals[i] -= (sub - pi_p_minus_1);
        }

        // Update small values
        for (int i = L; i >= p2; i--) {
            small_vals[i] -= (small_vals[i / p] - pi_p_minus_1);
        }
    }
}

ll pi(ll v) {
    if (v <= 0) return 0;
    if (v <= L) return small_vals[v];
    return large_vals[N / v];
}

int *sieve_primes;
int num_primes;

void gen_primes(int limit) {
    char *is_composite = calloc(limit + 1, 1);
    sieve_primes = malloc((limit + 1) * sizeof(int));
    num_primes = 0;
    for (int i = 2; i <= limit; i++) {
        if (!is_composite[i]) {
            sieve_primes[num_primes++] = i;
            if ((ll)i * i <= limit) {
                for (int j = i * i; j <= limit; j += i)
                    is_composite[j] = 1;
            }
        }
    }
    free(is_composite);
}

int main() {
    N = 10000000000LL;
    L = (int)sqrt((double)N) + 1;
    while ((ll)L * L > N) L--;

    small_vals = calloc(L + 2, sizeof(ll));
    large_vals = calloc(L + 2, sizeof(ll));

    lucy_dp();

    ll ans = N;

    // Remove primes p <= N/L
    // For these, we remove p values each
    int prime_limit = (int)(N / L);
    gen_primes(prime_limit);

    for (int i = 0; i < num_primes && sieve_primes[i] <= prime_limit; i++) {
        ans -= sieve_primes[i];
    }

    // For primes p > sqrt(N), remove floor(N/p) values
    // Group by d = floor(N/p), count primes with that d using Lucy DP
    for (int d = 1; d < L; d++) {
        ll count = pi(N / d) - pi(N / (d + 1));
        ans -= (ll)d * count;
    }

    printf("%lld\n", ans);

    free(small_vals);
    free(large_vals);
    free(sieve_primes);
    return 0;
}
'''
    with tempfile.NamedTemporaryFile(suffix='.c', delete=False) as f:
        f.write(c_code.encode())
        c_file = f.name
    exe = c_file[:-2]
    subprocess.run(['gcc', '-O3', '-march=native', '-lm', '-o', exe, c_file], check=True)
    result = subprocess.check_output([exe]).decode().strip()
    os.unlink(c_file)
    os.unlink(exe)
    print(result)

if __name__ == "__main__":
    solve()
