/*
 * Project Euler 773 - Numbers Relatively Prime to Primes Ending in 7
 *
 * Let L_N = 10 * product of first N primes ending in 7.
 * Find sum of all numbers < L_N ending in 7 and coprime to those primes.
 *
 * Uses inclusion-exclusion with the factored formula:
 *   ans = B/2 * prod(p_i - 1) + sum_i parity(i)*k_i*C(N,i)
 * multiplied by prod(primes).
 */
#include <stdio.h>
#include <stdint.h>

typedef __int128 i128;

#define N_PRIMES 97
#define MOD 1000000007LL
#define B 10
#define DIGIT 7

static int64_t pow_mod(int64_t base, int64_t exp, int64_t mod) {
    int64_t result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1)
            result = (i128)result * base % mod;
        base = (i128)base * base % mod;
        exp >>= 1;
    }
    return result;
}

static int64_t mod_inv_prime(int64_t a, int64_t mod) {
    return pow_mod(a, mod - 2, mod);
}

/* Extended GCD for mod inverse with non-prime modulus */
static int64_t mod_inv_general(int64_t a, int64_t m) {
    int64_t g = m, x = 0, y = 1;
    int64_t a0 = a;
    while (a0 != 0) {
        int64_t q = g / a0;
        int64_t t = g - q * a0;
        g = a0; a0 = t;
        t = x - q * y;
        x = y; y = t;
    }
    return (x % m + m) % m;
}

/* Binomial coefficient C(n, k) mod MOD (MOD prime) */
static int64_t nCr(int n, int k) {
    if (k < 0 || k > n) return 0;
    if (k == 0 || k == n) return 1;
    int64_t result = 1;
    int kk = k < n - k ? k : n - k;
    for (int i = 0; i < kk; i++) {
        result = (i128)result * ((n - i) % MOD) % MOD;
        result = (i128)result * mod_inv_prime(i + 1, MOD) % MOD;
    }
    return result;
}

/* Simple primality test */
static int is_prime(int n) {
    if (n < 2) return 0;
    if (n < 4) return 1;
    if (n % 2 == 0 || n % 3 == 0) return 0;
    for (int i = 5; (long long)i * i <= n; i += 6) {
        if (n % i == 0 || n % (i + 2) == 0) return 0;
    }
    return 1;
}

int main(void) {
    /* Find first 97 primes ending in 7 */
    int primes[N_PRIMES];
    int count = 0;
    for (int n = 2; count < N_PRIMES; n++) {
        if (is_prime(n) && n % B == DIGIT)
            primes[count++] = n;
    }

    /* ans = B/2 * prod(p_i - 1) mod MOD */
    int64_t ans = B / 2;
    for (int i = 0; i < N_PRIMES; i++)
        ans = (i128)ans * ((primes[i] - 1) % MOD) % MOD;

    /* Add inclusion-exclusion correction for k values */
    for (int i = 0; i <= N_PRIMES; i++) {
        /* k_val = 7 * (7^i)^{-1} mod 10 */
        int64_t ki = pow_mod(DIGIT, i, B);
        int64_t ki_inv = mod_inv_general(ki, B);
        int64_t k_val = ((int64_t)DIGIT * ki_inv) % B;

        int64_t sign = (i % 2 == 0) ? 1 : MOD - 1;
        int64_t term = (i128)sign * k_val % MOD * nCr(N_PRIMES, i) % MOD;
        ans = (ans + term) % MOD;
    }

    /* Multiply by prod(primes) */
    for (int i = 0; i < N_PRIMES; i++)
        ans = (i128)ans * (primes[i] % MOD) % MOD;

    printf("%lld\n", ans);
    return 0;
}
