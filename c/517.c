/*
 * Project Euler Problem 517: A real recursion.
 * g_a(x) = 1 for x<a, g_a(x) = g_a(x-1) + g_a(x-a) for x>=a.
 * Sum g_{sqrt(p)}(p) over all primes A <= p < B.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef long long ll;

#define A_VAL 10000000
#define B_VAL 10010000
#define MOD 1000000007LL

ll fact[B_VAL + 1];
ll inv_fact[B_VAL + 1];

ll power_mod(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) result = result * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return result;
}

void precompute_factorials(int limit) {
    fact[0] = 1;
    for (int i = 1; i <= limit; i++)
        fact[i] = fact[i - 1] * i % MOD;
    inv_fact[limit] = power_mod(fact[limit], MOD - 2, MOD);
    for (int i = limit; i > 0; i--)
        inv_fact[i - 1] = inv_fact[i] * i % MOD;
}

ll ncr_mod(int n, int k) {
    if (k < 0 || k > n) return 0;
    return fact[n] % MOD * inv_fact[k] % MOD * inv_fact[n - k] % MOD;
}

int main() {
    precompute_factorials(B_VAL);

    /* Sieve primes in [0, B_VAL] */
    char *is_prime = (char*)calloc(B_VAL + 1, 1);
    memset(is_prime, 1, B_VAL + 1);
    is_prime[0] = is_prime[1] = 0;
    int sq = (int)sqrt((double)B_VAL);
    for (int i = 2; i <= sq; i++)
        if (is_prime[i])
            for (int j = i * i; j <= B_VAL; j += i)
                is_prime[j] = 0;

    ll ans = 0;
    for (int p = A_VAL; p < B_VAL; p++) {
        if (!is_prime[p]) continue;

        double a = sqrt((double)p);
        int num_a = 0;
        while ((double)num_a * a < (double)p) {
            int floor_val = (int)floor((double)p - (double)num_a * a);
            int n = num_a + floor_val;
            ans = (ans + ncr_mod(n, num_a)) % MOD;
            num_a++;
        }
    }

    printf("%lld\n", ans);

    free(is_prime);
    return 0;
}
