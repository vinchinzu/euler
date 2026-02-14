/*
 * Project Euler 652 - Distinct values of logarithms
 * Count distinct log_m(n) for 2 <= m,n <= N = 10^18, mod 10^9.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef long long ll;
typedef __int128 lll;

#define M 1000000000LL

static ll nth_root(ll n, int k) {
    if (k == 1) return n;
    if (n <= 1) return n;
    double dr = 1.0 / k;
    ll x = (ll)pow((double)n, dr);
    /* Refine */
    if (x < 1) x = 1;
    /* Check x-1, x, x+1, x+2 */
    for (ll t = x + 2; t >= 1 && t >= x - 2; t--) {
        /* Check t^k <= n */
        lll pw = 1;
        int ok = 1;
        for (int i = 0; i < k; i++) {
            pw *= t;
            if (pw > n) { ok = 0; break; }
        }
        if (ok) {
            /* Check (t+1)^k > n */
            lll pw2 = 1;
            int over = 0;
            for (int i = 0; i < k; i++) {
                pw2 *= (t + 1);
                if (pw2 > n) { over = 1; break; }
            }
            if (over) return t;
        }
    }
    return x;
}

/* Mobius function precomputed */
static int mobius_vals[65];

static void precompute_mobius(int max_k) {
    for (int i = 0; i <= max_k; i++) mobius_vals[i] = 1;
    mobius_vals[0] = 0;

    /* Sieve of primes up to max_k */
    int is_prime[65];
    for (int i = 0; i <= max_k; i++) is_prime[i] = 1;
    is_prime[0] = is_prime[1] = 0;
    for (int i = 2; i * i <= max_k; i++)
        if (is_prime[i])
            for (int j = i*i; j <= max_k; j += i)
                is_prime[j] = 0;

    for (int p = 2; p <= max_k; p++) {
        if (!is_prime[p]) continue;
        for (int j = p; j <= max_k; j += p)
            mobius_vals[j] = -mobius_vals[j];
        for (int j = p*p; j <= max_k; j += p*p)
            mobius_vals[j] = 0;
    }
}

static ll nCr2(ll n) {
    /* C(n, 2) = n*(n-1)/2 */
    return (n % M) * ((n - 1) % M) % M * 500000000LL % M; /* inv2 mod 10^9 doesn't work since 10^9 is not prime */
}

/* Since M = 10^9 is not prime, compute C(n,2) mod M directly */
static ll nCr2_mod(ll n) {
    /* n*(n-1)/2 mod M */
    ll a = n % (2 * M);
    ll b = (n - 1) % (2 * M);
    ll prod = (a * b) % (2 * M);
    return (prod / 2) % M;
}

/* Check if b^e fits */
static int pow_fits(ll b, int e, ll limit) {
    lll pw = 1;
    for (int i = 0; i < e; i++) {
        pw *= b;
        if (pw > limit) return 0;
    }
    return pw <= limit;
}

int main(void) {
    ll N = 1000000000000000000LL; /* 10^18 */

    precompute_mobius(60);

    /* ans = (N-1)^2 - (N-2) mod M */
    ll ans = ((N - 1) % M * ((N - 1) % M) % M - (N - 2) % M + M) % M;

    /* Subtract k-th powers */
    for (int k = 2; k <= 60; k++) {
        if (!pow_fits(3, k, N)) break;
        ll num_powers = nth_root(N, k) - 1;
        ll contrib = (2 * mobius_vals[k] % M + M) % M * nCr2_mod(num_powers) % M;
        ans = (ans + contrib) % M;
    }

    /* Subtract cases where n is a k-th power of m >= 3 */
    for (int k = 2; k <= 60; k++) {
        if (!pow_fits(3, k, N)) break;
        ans = (ans - 2 * ((nth_root(N, k) - 2) % M) % M + 2 * M) % M;
        int e = 2;
        while (pow_fits(2, k * e, N)) {
            ll val = (2 * (ll)mobius_vals[e] % M + M) % M * ((nth_root(N, k * e) - 1) % M) % M;
            ans = (ans - val % M + M) % M;
            e++;
        }
    }

    /* Subtract pairs of powers with relatively prime exponents */
    /* For each base b >= 3 that is not a perfect power */
    ll max_b_val = nth_root(N, 3);

    /* Mark perfect powers up to max_b_val */
    /* A number is a perfect power if it can be written as c^e for e >= 2 */
    /* We only need to check for small max_b_val */
    /* Since max_b_val ~ 10^6, we can use an array */
    int max_b = (int)max_b_val;
    if (max_b > 10000000) max_b = 10000000;
    char *is_perfect_power = (char *)calloc(max_b + 1, 1);

    for (int b = 2; b <= max_b; b++) {
        if (!pow_fits(b, 6, N)) break;
        int e = 2;
        while (1) {
            lll pw = 1;
            int ok = 1;
            for (int i = 0; i < 3 * e; i++) {
                pw *= b;
                if (pw > N) { ok = 0; break; }
            }
            if (!ok) break;
            /* b^e is a perfect power */
            lll bpow = 1;
            for (int i = 0; i < e; i++) bpow *= b;
            if (bpow <= max_b) is_perfect_power[(int)bpow] = 1;
            e++;
        }
    }

    for (int b = 3; b <= max_b; b++) {
        if (!pow_fits(b, 3, N)) break;
        if (is_perfect_power[b]) continue;
        int largest_e = 1;
        while (pow_fits(b, largest_e + 1, N)) largest_e++;
        for (int e1 = 2; e1 <= largest_e; e1++) {
            for (int e2 = 2; e2 <= largest_e; e2++) {
                /* Check gcd(e1, e2) == 1 */
                int a = e1, bb = e2;
                while (bb) { int t = bb; bb = a % bb; a = t; }
                if (a == 1) {
                    ans = (ans - 1 + M) % M;
                }
            }
        }
    }

    free(is_perfect_power);
    printf("%lld\n", ans % M);
    return 0;
}
