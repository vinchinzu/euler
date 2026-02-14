/* Project Euler Problem 754: Product of Gauss Factorials.
 * Translated from python/754.py
 *
 * Compute product of g(i) for i=1..N, where g(n) is the product of all
 * positive integers up to n that are relatively prime to n.
 * Uses Mobius function and factorials with grouping for large g.
 */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

typedef long long ll;
typedef __int128 lll;

#define MOD 1000000007LL
#define MAXN 100000001  /* 10^8 + 1 */

static signed char *mobius;
static ll *fact;
static ll *prod_fact;

ll pow_mod(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) result = (lll)result * base % mod;
        exp >>= 1;
        base = (lll)base * base % mod;
    }
    return result;
}

ll mod_inv(ll a, ll mod) {
    return pow_mod(a, mod - 2, mod);
}

ll tr(ll n) {
    /* n*(n+1)/2 -- careful with overflow, but n <= 10^8 so n*(n+1) <= ~10^16 */
    return n * (n + 1) / 2;
}

void compute_mobius(int limit) {
    mobius = (signed char *)malloc((limit + 1) * sizeof(signed char));
    char *is_prime = (char *)malloc((limit + 1) * sizeof(char));

    for (int i = 0; i <= limit; i++) {
        mobius[i] = 1;
        is_prime[i] = 1;
    }
    is_prime[0] = is_prime[1] = 0;

    for (int i = 2; i <= limit; i++) {
        if (is_prime[i]) {
            for (int j = i; j <= limit; j += i) {
                if (j > i) is_prime[j] = 0;
                if ((ll)j % ((ll)i * i) == 0)
                    mobius[j] = 0;
                else
                    mobius[j] = -mobius[j];
            }
        }
    }
    free(is_prime);
}

int main() {
    int N = 100000000; /* 10^8 */
    int L = (int)sqrt((double)N);

    compute_mobius(N);

    /* Precompute factorials mod M */
    fact = (ll *)malloc((N + 1) * sizeof(ll));
    fact[0] = 1;
    for (int i = 1; i <= N; i++)
        fact[i] = (lll)fact[i - 1] * i % MOD;

    /* Precompute prefix product of factorials */
    prod_fact = (ll *)malloc((N + 1) * sizeof(ll));
    prod_fact[0] = 1;
    for (int i = 1; i <= N; i++)
        prod_fact[i] = (lll)prod_fact[i - 1] * fact[i] % MOD;

    /* res[0] for mu=-1, res[2] for mu=1 */
    ll res_neg = 1, res_pos = 1;

    /* For g <= N/L */
    int g_limit = N / L;
    for (int g = 1; g <= g_limit; g++) {
        if (mobius[g] == 0) continue;
        ll pw = pow_mod(g, tr(N / g) % (MOD - 1), MOD);
        if (mobius[g] == 1)
            res_pos = (lll)res_pos * pw % MOD;
        else
            res_neg = (lll)res_neg * pw % MOD;
    }

    /* For g > N/L, group by exponent q = N/g */
    for (int q = 1; q < L; q++) {
        ll sub_neg = 1, sub_pos = 1;
        int g_lo = N / (q + 1) + 1;
        int g_hi = N / q;
        for (int g = g_lo; g <= g_hi; g++) {
            if (mobius[g] == 0) continue;
            if (mobius[g] == 1)
                sub_pos = (lll)sub_pos * g % MOD;
            else
                sub_neg = (lll)sub_neg * g % MOD;
        }
        ll exp_val = tr(q) % (MOD - 1);
        res_pos = (lll)res_pos * pow_mod(sub_pos, exp_val, MOD) % MOD;
        res_neg = (lll)res_neg * pow_mod(sub_neg, exp_val, MOD) % MOD;
    }

    /* Multiply by product of factorials */
    for (int g = 1; g <= N; g++) {
        if (mobius[g] == 0) continue;
        if (mobius[g] == 1)
            res_pos = (lll)res_pos * prod_fact[N / g] % MOD;
        else
            res_neg = (lll)res_neg * prod_fact[N / g] % MOD;
    }

    ll ans = (lll)res_pos * mod_inv(res_neg, MOD) % MOD;

    printf("%lld\n", ans);

    free(mobius);
    free(fact);
    free(prod_fact);
    return 0;
}
