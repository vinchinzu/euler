/*
 * Project Euler Problem 365
 *
 * Compute sum(C(10^18, 10^9) mod p*q*r) for all primes 1000 < p < q < r < 5000,
 * using Lucas' theorem and CRT.
 */

#include <stdio.h>
#include <string.h>
#include <stdlib.h>

typedef long long ll;
typedef unsigned long long ull;
typedef __int128 lll;

#define MAX_PRIME 5000
#define MAX_PRIMES 700  /* there are about 560 primes between 1001 and 4999 */

static int primes[MAX_PRIMES];
static int nprimes;
static int fact_table[MAX_PRIMES][MAX_PRIME];
static int inv_fact_table[MAX_PRIMES][MAX_PRIME];
static ll lucas_vals[MAX_PRIMES];

static void sieve_primes(void) {
    char sieve[MAX_PRIME + 1];
    memset(sieve, 1, sizeof(sieve));
    sieve[0] = sieve[1] = 0;
    for (int i = 2; i * i <= MAX_PRIME; i++)
        if (sieve[i])
            for (int j = i * i; j <= MAX_PRIME; j += i)
                sieve[j] = 0;
    nprimes = 0;
    for (int i = 1001; i < MAX_PRIME; i++)
        if (sieve[i])
            primes[nprimes++] = i;
}

static ll extended_gcd(ll a, ll b, ll *x, ll *y) {
    if (a == 0) { *x = 0; *y = 1; return b; }
    ll x1, y1;
    ll g = extended_gcd(b % a, a, &x1, &y1);
    *x = y1 - (b / a) * x1;
    *y = x1;
    return g;
}

static ll mod_inverse(ll a, ll m) {
    ll x, y;
    extended_gcd(a, m, &x, &y);
    return ((x % m) + m) % m;
}

static void precompute_factorials(int idx) {
    int p = primes[idx];
    fact_table[idx][0] = 1;
    for (int i = 1; i < p; i++)
        fact_table[idx][i] = (int)((ll)fact_table[idx][i-1] * i % p);
    inv_fact_table[idx][p-1] = (int)mod_inverse(fact_table[idx][p-1], p);
    for (int i = p - 2; i >= 0; i--)
        inv_fact_table[idx][i] = (int)((ll)inv_fact_table[idx][i+1] * (i+1) % p);
}

static int binomial_small(int idx, int n, int k) {
    int p = primes[idx];
    if (k < 0 || k > n) return 0;
    if (n >= p) return 0;
    return (int)((ll)fact_table[idx][n] * inv_fact_table[idx][k] % p * inv_fact_table[idx][n-k] % p);
}

static ll lucas_mod(ll n, ll k, int idx) {
    int p = primes[idx];
    if (k < 0 || k > n) return 0;
    if (k == 0) return 1;
    ll result = 1;
    ll nd = n, kd = k;
    while (nd > 0 || kd > 0) {
        int nd_digit = (int)(nd % p);
        int kd_digit = (int)(kd % p);
        if (kd_digit > nd_digit) return 0;
        int c = binomial_small(idx, nd_digit, kd_digit);
        result = result * c % p;
        nd /= p;
        kd /= p;
    }
    return result;
}

static ll crt_three(ll a1, ll m1, ll a2, ll m2, ll a3, ll m3) {
    lll m12 = (lll)m1 * m2;
    lll m = m12 * m3;

    ll inv1 = mod_inverse(m1, m2);
    lll x12 = ((lll)a1 + (lll)m1 * (((lll)(a2 - a1) * inv1 % m2 + m2) % m2)) % m12;

    ll inv12 = mod_inverse((ll)(m12 % m3), m3);
    lll x = (x12 + m12 * (((lll)(a3 - (ll)(x12 % m3)) * inv12 % m3 + m3) % m3)) % m;

    return (ll)x;
}

int main(void) {
    sieve_primes();

    ll N = 1000000000000000000LL; /* 10^18 */
    ll K = 1000000000LL;          /* 10^9 */

    for (int i = 0; i < nprimes; i++) {
        precompute_factorials(i);
        lucas_vals[i] = lucas_mod(N, K, i);
    }

    ll total_sum = 0;
    for (int i = 0; i < nprimes - 2; i++) {
        ll ap = lucas_vals[i];
        int p = primes[i];
        for (int j = i + 1; j < nprimes - 1; j++) {
            ll aq = lucas_vals[j];
            int q = primes[j];
            for (int ki = j + 1; ki < nprimes; ki++) {
                ll ar = lucas_vals[ki];
                int r = primes[ki];
                total_sum += crt_three(ap, p, aq, q, ar, r);
            }
        }
    }

    printf("%lld\n", total_sum);
    return 0;
}
