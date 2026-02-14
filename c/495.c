/* Project Euler 495 - Writing n! as product of k distinct integers
 * Extracted from embedded C in python/495.py
 * Inclusion-exclusion over partitions of K=30.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define N 10000
#define K 30
#define MOD 1000000007LL

static long long dp[K + 1][N + 1];
static int primes[1300];
static int exponents[1300];
static int nprimes;
static long long inv_val[K + 1];
static long long inv_fact[K + 1];
static long long ans;

static void sieve(void) {
    char is_prime[N + 1];
    memset(is_prime, 1, sizeof(is_prime));
    is_prime[0] = is_prime[1] = 0;
    int i, j;
    for (i = 2; (long long)i * i <= N; i++) {
        if (is_prime[i]) {
            for (j = i * i; j <= N; j += i)
                is_prime[j] = 0;
        }
    }
    nprimes = 0;
    for (i = 2; i <= N; i++) {
        if (is_prime[i])
            primes[nprimes++] = i;
    }
}

static int vp_factorial(int n, int p) {
    int count = 0;
    long long pw = p;
    while (pw <= n) {
        count += n / (int)pw;
        pw *= p;
    }
    return count;
}

static void precompute(void) {
    inv_val[0] = 0;
    inv_val[1] = 1;
    int i;
    for (i = 2; i <= K; i++) {
        inv_val[i] = (MOD - MOD / i) * inv_val[MOD % i] % MOD;
    }
    long long fact = 1;
    for (i = 1; i <= K; i++) fact = fact * i % MOD;
    {
        long long base = fact, exp = MOD - 2, result = 1;
        while (exp > 0) {
            if (exp & 1) result = result * base % MOD;
            base = base * base % MOD;
            exp >>= 1;
        }
        inv_fact[K] = result;
    }
    for (i = K - 1; i >= 0; i--) {
        inv_fact[i] = inv_fact[i + 1] * (i + 1) % MOD;
    }
}

static int coins[K + 1];

static void helper(int min_val, int remaining, int depth) {
    if (remaining == 0) {
        long long res = 1;
        int i;
        for (i = 0; i < nprimes; i++) {
            res = res * dp[depth][exponents[i]] % MOD;
        }
        for (i = 0; i < depth; i++) {
            int c = coins[i];
            if (c % 2 == 0) {
                res = res * (MOD - 1) % MOD;
            }
            res = res * inv_val[c] % MOD;
        }
        i = 0;
        while (i < depth) {
            int j = i;
            while (j < depth && coins[j] == coins[i]) j++;
            int cnt = j - i;
            res = res * inv_fact[cnt] % MOD;
            i = j;
        }
        ans = (ans + res) % MOD;
        return;
    }

    int coeff;
    for (coeff = min_val; coeff <= remaining; coeff++) {
        int max_exp = N;
        int e;
        memcpy(dp[depth + 1], dp[depth], (max_exp + 1) * sizeof(long long));
        for (e = coeff; e <= max_exp; e++) {
            dp[depth + 1][e] = (dp[depth + 1][e] + dp[depth + 1][e - coeff]) % MOD;
        }
        coins[depth] = coeff;
        helper(coeff, remaining - coeff, depth + 1);
    }
}

int main(void) {
    sieve();
    precompute();

    int i;
    for (i = 0; i < nprimes; i++) {
        exponents[i] = vp_factorial(N, primes[i]);
    }

    memset(dp, 0, sizeof(dp));
    dp[0][0] = 1;

    ans = 0;
    helper(1, K, 0);

    printf("%lld\n", ans);
    return 0;
}
