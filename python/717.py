"""Project Euler Problem 717: Summation of a Modular Formula.

Embedded C solution for speed. Sieve primes up to 10^7, compute g(p) for each
odd prime using modular exponentiation mod p and p^2, sum the results.
"""
import subprocess, tempfile, os, sys

C_CODE = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef unsigned long long ull;

ull pow_mod(ull base, ull exp, ull mod) {
    ull result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) result = (unsigned __int128)result * base % mod;
        base = (unsigned __int128)base * base % mod;
        exp >>= 1;
    }
    return result;
}

int main(void) {
    int N = 10000000;
    /* Sieve of Eratosthenes */
    char *is_prime = calloc(N + 1, 1);
    if (!is_prime) { fprintf(stderr, "alloc failed\n"); return 1; }
    memset(is_prime, 1, N + 1);
    is_prime[0] = is_prime[1] = 0;
    for (int i = 2; (long long)i * i <= N; i++) {
        if (is_prime[i]) {
            for (int j = i * i; j <= N; j += i)
                is_prime[j] = 0;
        }
    }

    long long ans = 0;
    for (int p = 3; p <= N; p += 2) {
        if (!is_prime[p]) continue;
        /* g(p): compute 2^(2^p + p - 2) mod p^2, then floor/p mod p */
        ull pm1 = (ull)p - 1;
        ull p2 = (ull)p * (ull)p;

        /* exp_mod_pm1 = 2^p mod (p-1) */
        ull exp_pm1 = pow_mod(2, p, pm1);
        /* k = 2^(exp_pm1 + p - 2) mod p */
        ull k = pow_mod(2, exp_pm1 + (ull)p - 2, (ull)p);
        /* two_to_p_mod_p2 = 2^p mod p^2 */
        ull t2p = pow_mod(2, p, p2);
        /* numerator = k * 2^p mod p^2 */
        ull num = (unsigned __int128)k * t2p % p2;
        /* g(p) = floor(num / p) mod p */
        ull gp = (num / (ull)p) % (ull)p;
        ans += (long long)gp;
    }

    free(is_prime);
    printf("%lld\n", ans);
    return 0;
}
"""

def main():
    with tempfile.NamedTemporaryFile(suffix='.c', mode='w', delete=False) as f:
        f.write(C_CODE)
        c_path = f.name
    bin_path = c_path.replace('.c', '')
    try:
        subprocess.run(['gcc', '-O2', '-o', bin_path, c_path, '-lm'],
                       check=True, capture_output=True)
        result = subprocess.run([bin_path], capture_output=True, text=True,
                                timeout=280)
        print(result.stdout.strip())
    finally:
        for p in [c_path, bin_path]:
            if os.path.exists(p):
                os.unlink(p)

if __name__ == "__main__":
    main()
