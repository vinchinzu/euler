#include <stdio.h>
#include <string.h>

/*
 * Project Euler 857 - Beautiful Graphs
 *
 * Computes G(n) mod p where:
 *   G(0) = 1
 *   G(i) = sum_{k=1}^{min(i,5)} binom(i, k) * A[k] * G(i-k)  (mod p)
 *   A = {0, 1, 2, 6, 18, 12}
 *   p = 10^9 + 7
 *   n = 10^7
 *
 * Uses a sliding window of 5 G values and precomputed inv_fact[k] for k=1..5.
 */

static const long long MOD = 1000000007LL;
static const long long A_VALS[6] = {0, 1, 2, 6, 18, 12};

/* Modular exponentiation: base^exp mod m */
static long long mod_pow(long long base, long long exp, long long m) {
    long long result = 1;
    base %= m;
    while (exp > 0) {
        if (exp & 1)
            result = result * base % m;
        base = base * base % m;
        exp >>= 1;
    }
    return result;
}

int main(void) {
    long long n = 10000000LL;

    /* Precompute inverse factorials for k = 1..5 */
    long long inv_fact[6];
    inv_fact[0] = 1;
    long long fact = 1;
    for (int i = 1; i <= 5; i++) {
        fact = fact * i % MOD;
        inv_fact[i] = mod_pow(fact, MOD - 2, MOD);
    }

    /* Precompute A[k] * inv_fact[k] mod MOD for k=1..5 to save one multiply per inner step */
    long long a_invf[6];
    for (int k = 1; k <= 5; k++) {
        a_invf[k] = A_VALS[k] % MOD * inv_fact[k] % MOD;
    }

    /*
     * Sliding window of 5 G values stored in a circular buffer.
     * g_hist[j] for j in 0..4 stores G values.
     * 'head' points to the oldest entry.
     *
     * Initially represents G(-4)..G(0) = {0, 0, 0, 0, 1}.
     * For iteration i:
     *   G(i-k) is at index (head + 5 - k) % 5  (since newest = (head+4)%5)
     *
     * After computing G(i), we write it at g_hist[head] and advance head.
     */
    long long g_hist[5] = {0, 0, 0, 0, 1};
    int head = 0; /* points to the slot holding G(i-5), which is the oldest */

    for (long long i = 1; i <= n; i++) {
        long long val = 0;
        long long n_prod = 1; /* accumulates i * (i-1) * ... * (i-k+1) mod MOD */

        for (int k = 1; k <= 5; k++) {
            /* G(i-k) is at index (head + 5 - k) % 5 */
            long long prev_g = g_hist[(head + 5 - k) % 5];

            /* n_prod = i * (i-1) * ... * (i-k+1) mod MOD */
            n_prod = n_prod % MOD * ((i - k + 1) % MOD + MOD) % MOD;

            /* binom(i,k) * A[k] * prev_g = n_prod * a_invf[k] * prev_g */
            long long term = n_prod % MOD * a_invf[k] % MOD * (prev_g % MOD) % MOD;
            val = (val + term) % MOD;
        }

        /* Write G(i) into the oldest slot, advance head */
        g_hist[head] = val;
        head = (head + 1) % 5;
    }

    /* The answer G(n) was the last value written, at index (head - 1 + 5) % 5 */
    long long ans = g_hist[(head - 1 + 5) % 5];
    printf("%lld\n", ans);
    return 0;
}
