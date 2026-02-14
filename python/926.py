"""Project Euler Problem 926 - Total Roundness. Embedded C port for speed."""

import subprocess
import tempfile
import os

C_CODE = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
#define MOD 1000000007LL

int main(void) {
    int n = 10000000;

    /* Sieve of Eratosthenes */
    char *is_prime = (char *)malloc((size_t)(n + 1));
    if (!is_prime) { fprintf(stderr, "malloc failed\n"); return 1; }
    memset(is_prime, 1, (size_t)(n + 1));
    is_prime[0] = is_prime[1] = 0;
    for (int i = 2; (ll)i * i <= n; i++) {
        if (is_prime[i]) {
            for (int j = i * i; j <= n; j += i) {
                is_prime[j] = 0;
            }
        }
    }

    /* Count primes */
    int num_primes = 0;
    for (int i = 2; i <= n; i++) {
        if (is_prime[i]) num_primes++;
    }

    /* Collect primes and compute exponents in n! */
    int *primes = (int *)malloc((size_t)num_primes * sizeof(int));
    ll *exponents = (ll *)malloc((size_t)num_primes * sizeof(ll));
    if (!primes || !exponents) { fprintf(stderr, "malloc failed\n"); return 1; }

    int idx = 0;
    for (int i = 2; i <= n; i++) {
        if (is_prime[i]) {
            primes[idx] = i;
            /* Legendre's formula: exponent of p in n! */
            ll count = 0;
            ll power = (ll)i;
            while (power <= (ll)n) {
                count += (ll)n / power;
                power *= (ll)i;
            }
            exponents[idx] = count;
            idx++;
        }
    }
    free(is_prime);

    /* Find max exponent */
    ll max_v = 0;
    for (int i = 0; i < num_primes; i++) {
        if (exponents[i] > max_v) max_v = exponents[i];
    }

    ll total = 0;

    /* For each roundness value j from 1 to max_v */
    for (ll j = 1; j <= max_v; j++) {
        ll product = 1;
        int all_one = 1;

        for (int i = 0; i < num_primes; i++) {
            ll vp = exponents[i];
            if (vp < j) break; /* exponents are sorted descending for n! */

            ll factor = 1 + vp / j;
            product = product * factor % MOD;
            if (factor > 1) all_one = 0;
        }

        if (!all_one) {
            ll contribution = (product - 1 + MOD) % MOD;
            total = (total + contribution) % MOD;
        }
    }

    printf("%lld\n", total);

    free(primes);
    free(exponents);
    return 0;
}
"""

def main():
    with tempfile.TemporaryDirectory() as tmpdir:
        src = os.path.join(tmpdir, "p926.c")
        exe = os.path.join(tmpdir, "p926")
        with open(src, "w") as f:
            f.write(C_CODE)
        subprocess.run(["gcc", "-O2", "-o", exe, src], check=True)
        result = subprocess.run([exe], capture_output=True, text=True, timeout=280)
        print(result.stdout.strip())

if __name__ == "__main__":
    main()
