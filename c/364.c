/*
 * Project Euler Problem 364: Comfortable Distance.
 *
 * T(N) counts distinct seating sequences for N seats following priority rules.
 * Formula (Max Alekseyev, OEIS A192008):
 *   T(n) = sum over v=0,1,2 and valid m of:
 *     (m+k+1)! * C(m+k,m) * 2^k * (k+v)! * (m+k)! * (1+(v==1))
 *   where k = (n-1-v-2m)/3.
 *
 * Compute T(1000000) mod 100000007.
 */

#include <stdio.h>
#include <stdlib.h>

#define MOD 100000007LL

typedef long long ll;

static ll *fact;
static ll *inv_fact;
static int N_MAX;

static ll mod_pow(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) result = result * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return result;
}

static void precompute(int n) {
    N_MAX = n + 2;
    fact = (ll *)malloc((N_MAX + 1) * sizeof(ll));
    inv_fact = (ll *)malloc((N_MAX + 1) * sizeof(ll));
    fact[0] = 1;
    for (int i = 1; i <= N_MAX; i++)
        fact[i] = fact[i-1] * i % MOD;
    inv_fact[N_MAX] = mod_pow(fact[N_MAX], MOD - 2, MOD);
    for (int i = N_MAX - 1; i >= 0; i--)
        inv_fact[i] = inv_fact[i+1] * (i+1) % MOD;
}

static ll C_mod(int n, int k) {
    if (k < 0 || k > n) return 0;
    return fact[n] * inv_fact[k] % MOD * inv_fact[n-k] % MOD;
}

int main(void) {
    int N = 1000000;
    precompute(N);

    ll r = 0;
    for (int v = 0; v < 3; v++) {
        int rem = N - 1 - v;
        if (rem < 0) continue;

        int m_start;
        if (rem % 3 == 0) m_start = 0;
        else if (rem % 3 == 1) m_start = 2;
        else m_start = 1;

        for (int m = m_start; 2 * m <= rem; m += 3) {
            int k = (rem - 2 * m) / 3;
            int mk = m + k;

            ll term = fact[mk + 1];
            term = term * C_mod(mk, m) % MOD;
            term = term * mod_pow(2, k, MOD) % MOD;
            term = term * fact[k + v] % MOD;
            term = term * fact[mk] % MOD;
            if (v == 1) term = term * 2 % MOD;

            r = (r + term) % MOD;
        }
    }

    printf("%lld\n", r);
    free(fact);
    free(inv_fact);
    return 0;
}
