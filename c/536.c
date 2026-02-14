/*
 * Project Euler 536 - Modulo Power Identity
 *
 * Find sum of all m <= N such that a^{m+4} = a (mod m) for all a.
 * m must be squarefree, and lambda(m) | m+3.
 * Recursively build squarefree m by multiplying primes.
 */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <math.h>

typedef long long ll;

#define SIEVE_LIMIT 100000000
#define PRIME_LIMIT 56000  /* sqrt(10^12) ~ 10^6, but we sieve primes up to that */

static int *spf;  /* smallest prime factor */
static int *primes;
static int nprimes;

static ll N_val;
static ll ans;

static ll gcd(ll a, ll b) {
    while (b) { ll t = b; b = a % b; a = t; }
    return a;
}

static ll lcm(ll a, ll b) {
    return a / gcd(a, b) * b;
}

static void sieve_spf(int limit) {
    spf = (int*)malloc((limit + 1) * sizeof(int));
    for (int i = 0; i <= limit; i++) spf[i] = i;
    for (int i = 2; (ll)i * i <= limit; i++) {
        if (spf[i] == i) {
            for (int j = i * i; j <= limit; j += i) {
                if (spf[j] == j) spf[j] = i;
            }
        }
    }
}

static void sieve_primes(int limit) {
    char *is_p = (char*)calloc(limit + 1, 1);
    for (int i = 2; i <= limit; i++) is_p[i] = 1;
    for (int i = 2; (ll)i * i <= limit; i++) {
        if (is_p[i])
            for (int j = i * i; j <= limit; j += i)
                is_p[j] = 0;
    }
    nprimes = 0;
    for (int i = 2; i <= limit; i++) if (is_p[i]) nprimes++;
    primes = (int*)malloc(nprimes * sizeof(int));
    int idx = 0;
    for (int i = 2; i <= limit; i++) if (is_p[i]) primes[idx++] = i;
    free(is_p);
}

static ll mod_inv(ll a, ll m) {
    if (m == 1) return 0;
    ll t = 0, new_t = 1, r = m, new_r = a;
    while (new_r != 0) {
        ll q = r / new_r;
        ll tmp = new_t; new_t = t - q * new_t; t = tmp;
        tmp = new_r; new_r = r - q * new_r; r = tmp;
    }
    if (r != 1) return -1;
    if (t < 0) t += m;
    return t;
}

static ll imod(ll a, ll m) {
    return ((a % m) + m) % m;
}

/* Check if m*r satisfies conditions where r > 1 is the remaining factor */
static int good(ll m, ll r, int max_p) {
    while (r > 1) {
        int p;
        if (r < SIEVE_LIMIT)
            p = spf[(int)r];
        else
            p = (int)r; /* r itself must be prime if > sieve limit */
        if ((m + 3) % (p - 1) != 0) return 0;
        if (p >= max_p) return 0;
        r /= p;
        if (r % p == 0) return 0;  /* not squarefree */
    }
    return 1;
}

static void helper(int max_index, ll m, ll carmichael) {
    ll g = gcd(m, carmichael);
    if (3 % g != 0) return;
    if ((m + 3) % carmichael == 0) {
        ans += m;
    }

    /* Optimization: enumerate multiples via CRT */
    if (N_val / m < SIEVE_LIMIT && N_val / m / carmichael < (1LL << max_index)) {
        ll mod_val = carmichael / g;
        if (mod_val > 0) {
            ll inv = mod_inv(m / g, mod_val);
            if (inv >= 0) {
                ll r_start = imod((-3 / g) * inv, mod_val);
                for (ll r = r_start; m * r <= N_val; r += mod_val) {
                    if (r > 1) {
                        int mp = max_index < nprimes ? primes[max_index] : (int)(N_val + 1);
                        if (good(m * r, r, mp)) {
                            ans += m * r;
                        }
                    }
                }
            }
        }
        return;
    }

    /* Recursive case: try adding primes */
    for (int index = max_index - 1; index >= 0; index--) {
        ll p = primes[index];
        if (m * p > N_val) continue;
        helper(index, m * p, lcm(carmichael, p - 1));
    }
}

int main(void) {
    N_val = 1000000000000LL; /* 10^12 */
    int sqrt_n = (int)sqrt((double)N_val) + 1;

    sieve_spf(SIEVE_LIMIT);
    sieve_primes(sqrt_n);

    ans = 0;
    helper(nprimes, 1, 1);

    printf("%lld\n", ans);

    free(spf);
    free(primes);
    return 0;
}
