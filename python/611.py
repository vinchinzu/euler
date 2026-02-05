#!/usr/bin/env python3
"""Project Euler Problem 611: Hallway of square steps.

Count integers up to N that can be expressed as a^2 + b^2 for positive a < b
in an odd number of ways.
"""

import subprocess
import tempfile
import os


def solve():
    c_code = r'''
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef long long ll;
typedef __int128 lll;

#define N 1000000000000LL

ll isqrt_ll(ll n) {
    ll x = (ll)sqrtl((long double)n);
    while (x > 0 && (lll)x * x > n) x--;
    while ((lll)(x+1)*(x+1) <= n) x++;
    return x;
}

int main() {
    ll L = isqrt_ll(N);

    // Sieve primes up to L starting from 3
    char *is_prime = (char*)calloc(L + 1, sizeof(char));
    for (ll i = 2; i <= L; i++) is_prime[i] = 1;
    for (ll i = 2; i * i <= L; i++) {
        if (is_prime[i]) {
            for (ll j = i * i; j <= L; j += i) is_prime[j] = 0;
        }
    }

    // Collect primes >= 3
    int num_primes = 0;
    for (ll i = 3; i <= L; i++) if (is_prime[i]) num_primes++;
    int *primes = (int*)malloc(num_primes * sizeof(int));
    int idx = 0;
    for (ll i = 3; i <= L; i++) if (is_prime[i]) primes[idx++] = (int)i;

    // Initialize big and small arrays for counting primes ≡ 1 (mod 4)
    // big[r][i] counts numbers ≡ (1 + 2r) (mod 4) up to N/i
    // small[r][i] counts numbers ≡ (1 + 2r) (mod 4) up to i
    ll big_size = N / L + 1;
    ll *big0 = (ll*)calloc(big_size, sizeof(ll));
    ll *big1 = (ll*)calloc(big_size, sizeof(ll));
    ll *small0 = (ll*)calloc(L + 1, sizeof(ll));
    ll *small1 = (ll*)calloc(L + 1, sizeof(ll));

    // Initialize: count of n ≡ 1 (mod 4) in [1, x] is (x + 3) / 4
    //             count of n ≡ 3 (mod 4) in [1, x] is (x + 1) / 4
    for (ll i = 1; i < big_size; i++) {
        big0[i] = (N / i + 3) / 4;  // ≡ 1 (mod 4)
        big1[i] = (N / i + 1) / 4;  // ≡ 3 (mod 4)
    }
    for (ll i = 1; i <= L; i++) {
        small0[i] = (i + 3) / 4;
        small1[i] = (i + 1) / 4;
    }

    // Sieve: for each odd prime p >= 3, update the counts
    for (int pi = 0; pi < num_primes; pi++) {
        ll p = primes[pi];
        ll p2 = p * p;

        // Update big arrays
        for (ll i = 1; i < big_size && N / i >= p2; i++) {
            ll ip = i * p;
            ll val0, val1;

            if (ip < big_size) {
                val0 = big0[ip];
                val1 = big1[ip];
            } else {
                val0 = small0[N / ip];
                val1 = small1[N / ip];
            }

            // Subtract primes up to p-1 from val
            val0 -= small0[p - 1];
            val1 -= small1[p - 1];

            if (p % 4 == 1) {
                big0[i] -= val0;
                big1[i] -= val1;
            } else {  // p % 4 == 3
                big0[i] -= val1;
                big1[i] -= val0;
            }
        }

        // Update small arrays (in reverse order)
        for (ll i = L; i >= p2; i--) {
            ll val0 = small0[i / p] - small0[p - 1];
            ll val1 = small1[i / p] - small1[p - 1];
            if (p % 4 == 1) {
                small0[i] -= val0;
                small1[i] -= val1;
            } else {
                small0[i] -= val1;
                small1[i] -= val0;
            }
        }
    }

    // Subtract 1 from small0 and big0 to remove the count of "1" (which is ≡ 1 mod 4 but not prime)
    for (ll i = 1; i < big_size; i++) big0[i]--;
    for (ll i = 1; i <= L; i++) small0[i]--;

    // Now big0[i] = π_1(N/i) = count of primes ≡ 1 (mod 4) up to N/i
    // small0[i] = π_1(i) = count of primes ≡ 1 (mod 4) up to i

    ll ans = 0;

    // Helper function via stack-based iteration
    // State: (minIndex, n, P, skip)
    typedef struct {
        int minIndex;
        ll n;
        int P;
        int skip;
    } State;

    int stack_size = 0;
    int stack_cap = 1000000;
    State *stack = (State*)malloc(stack_cap * sizeof(State));

    // Push initial state
    stack[stack_size++] = (State){0, 1, 1, 1};

    while (stack_size > 0) {
        State s = stack[--stack_size];
        int minIndex = s.minIndex;
        ll n = s.n;
        int P = s.P;
        int skip = s.skip;

        if (minIndex >= num_primes) continue;

        int p_first = primes[minIndex];

        // Count this value if not skipping
        if (!skip && ((P + 1) / 2 - P) % 2 != 0) {
            ans++;
        }

        // Add contribution from largest prime factor (primes ≡ 1 mod 4)
        if (N / n >= p_first && ((2 * P + 1) / 2) % 2 != 0) {
            // Count primes q ≡ 1 (mod 4) with p_first <= q <= N/n
            ll cnt;
            ll limit = N / n;
            if (limit <= L) {
                cnt = small0[limit];
            } else {
                // Find i such that N / i = limit, i.e., i = N / limit
                ll i = N / limit;
                cnt = big0[i];
            }
            cnt -= small0[p_first - 1];
            if (p_first % 4 == 1) cnt++;  // include p_first itself
            ans += cnt;
        }

        // Recurse with smaller factors
        for (int index = minIndex; index + 1 < num_primes; index++) {
            int p = primes[index];
            if ((lll)n * p * p > N) break;

            // For p ≡ 3 (mod 4), only even exponents matter (start at 2, step 2)
            // For p ≡ 1 (mod 4), all exponents matter (start at 1, step 1)
            int e_start = (p % 4 == 3) ? 2 : 1;
            int e_step = (p % 4 == 3) ? 2 : 1;

            ll power = 1;
            for (int i = 0; i < e_start; i++) power *= p;

            for (int e = e_start; (double)n * power <= (double)N; e += e_step) {
                int new_P = P * ((p % 4 == 1) ? (e + 1) : 1);
                int new_skip = (p % 4 == 1 && e == 1) ? 1 : 0;

                if (stack_size >= stack_cap) {
                    stack_cap *= 2;
                    stack = (State*)realloc(stack, stack_cap * sizeof(State));
                }
                stack[stack_size++] = (State){index + 1, n * power, new_P, new_skip};

                for (int i = 0; i < e_step; i++) {
                    if ((double)power * p > (double)N / n) break;
                    power *= p;
                }
            }
        }
    }

    printf("%lld\n", ans);

    free(is_prime);
    free(primes);
    free(big0);
    free(big1);
    free(small0);
    free(small1);
    free(stack);
    return 0;
}
'''

    with tempfile.NamedTemporaryFile(suffix='.c', delete=False) as f:
        f.write(c_code.encode())
        c_file = f.name
    exe = c_file[:-2]
    try:
        subprocess.run(['gcc', '-O3', '-march=native', '-o', exe, c_file, '-lm'],
                       check=True, capture_output=True)
        result = subprocess.check_output([exe], timeout=300).decode().strip()
        print(result)
    finally:
        os.unlink(c_file)
        if os.path.exists(exe):
            os.unlink(exe)


if __name__ == "__main__":
    solve()
