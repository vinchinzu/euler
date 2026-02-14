/*
 * Project Euler 830: Binomial Coefficients mod p^3
 *
 * Uses CRT with p = 83, 89, 97.
 * Computes S(p, n) = sum_{m=0}^{3p-1} [sum_{j=0}^m (-1)^(m-j) C(m,j) j^e] * C(n,m) * 2^(n-m) mod p^3
 * where e = 10^18 mod (p^2 * (p-1)).
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef __int128 i128;

static ll mod_pow(ll base, ll exp, ll mod) {
    i128 result = 1;
    i128 b = base % mod;
    if (b < 0) b += mod;
    while (exp > 0) {
        if (exp & 1) result = result * b % mod;
        b = b * b % mod;
        exp >>= 1;
    }
    return (ll)result;
}

/* Compute binomial C(n, m) mod mod where n can be huge but m is small */
static ll binom_mod(ll n, int m, ll mod) {
    if (m == 0) return 1 % mod;
    ll res = 1;
    for (int i = 1; i <= m; i++) {
        res = (i128)res * ((n - m + i) % mod) % mod;
        /* divide by i: use modular inverse */
        res = (i128)res * mod_pow(i, mod - 2, mod) % mod;
        /* Actually mod might not be prime. Use extended GCD or just compute exact fraction. */
    }
    return (res % mod + mod) % mod;
}

/* Build binomial table up to max_m using Pascal's triangle mod mod */
static void build_bin_table(int max_m, ll mod, ll table[][300]) {
    for (int i = 0; i <= max_m; i++) {
        table[i][0] = 1;
        for (int j = 1; j <= i && j <= max_m; j++) {
            table[i][j] = (table[i-1][j-1] + table[i-1][j]) % mod;
        }
        for (int j = i + 1; j <= max_m; j++) {
            table[i][j] = 0;
        }
    }
}

/* Compute C(n, m) mod p^3 for large n, small m */
static ll compute_binom_mod(ll n, int m, ll mod) {
    if (m == 0) return 1 % mod;
    /* Since m is small (<300), compute numerator directly */
    ll res = 1;
    for (int i = 1; i <= m; i++) {
        res = (i128)res * ((n - m + i) % mod + mod) % mod;
    }
    /* Compute m! mod mod and its inverse */
    ll fact_m = 1;
    for (int i = 2; i <= m; i++) {
        fact_m = (i128)fact_m * i % mod;
    }
    /* Need modular inverse of fact_m mod p^3 */
    /* Use extended GCD since mod may not be prime */
    /* But for p^3 where p is prime and m < p, gcd(m!, p^3) = 1 */
    /* Use Euler's theorem: a^phi(p^3) = 1 mod p^3 */
    ll phi = 0;
    /* Compute phi(mod) = mod * (1 - 1/p) for each prime factor */
    /* mod = p^3, so phi = p^3 - p^2 = p^2 * (p-1) */
    /* We'll receive this from caller */

    /* Simpler: just compute inverse via extended GCD */
    ll a = fact_m, b = mod;
    ll old_r = a, r = b;
    ll old_s = 1, s = 0;
    while (r != 0) {
        ll q = old_r / r;
        ll tmp = r; r = old_r - q * r; old_r = tmp;
        tmp = s; s = old_s - q * s; old_s = tmp;
    }
    ll inv = (old_s % mod + mod) % mod;
    res = (i128)res * inv % mod;
    return res;
}

static ll compute_s_mod_p3(int p, ll n) {
    ll mod = (ll)p * p * p;
    ll phi = (ll)p * p * (p - 1);
    ll e = mod_pow(10, 18, phi);

    int max_m = 3 * p - 1;

    /* Build binomial table */
    ll (*bin_table)[300] = (ll (*)[300])calloc(max_m + 1, 300 * sizeof(ll));
    build_bin_table(max_m, mod, bin_table);

    ll total = 0;
    for (int m = 0; m <= max_m; m++) {
        ll sum_se = 0;
        for (int j = 0; j <= m; j++) {
            int sign = ((m - j) % 2 == 0) ? 1 : -1;
            ll je = (j == 0) ? 0 : mod_pow(j, e, mod);
            ll term = (i128)bin_table[m][j] * je % mod * sign;
            sum_se = (sum_se + term % mod + mod) % mod;
        }
        ll bnm = compute_binom_mod(n, m, mod);
        ll tw = mod_pow(2, n - m, mod);
        ll term = (i128)sum_se * bnm % mod;
        term = (i128)term * tw % mod;
        total = (total + term) % mod;
    }

    free(bin_table);
    return total;
}

/* Extended GCD for CRT */
static ll ext_gcd(ll a, ll b, ll *x, ll *y) {
    if (b == 0) { *x = 1; *y = 0; return a; }
    ll x1, y1;
    ll g = ext_gcd(b, a % b, &x1, &y1);
    *x = y1;
    *y = x1 - (a / b) * y1;
    return g;
}

static ll crt3(ll *as, ll *ms) {
    i128 M = (i128)ms[0] * ms[1] * ms[2];
    i128 x = 0;
    for (int i = 0; i < 3; i++) {
        i128 Mi = M / ms[i];
        ll xi, yi;
        ext_gcd((ll)(Mi % ms[i]), ms[i], &xi, &yi);
        xi = (xi % ms[i] + ms[i]) % ms[i];
        x = (x + (i128)as[i] * Mi % M * xi) % M;
    }
    return (ll)((x % M + M) % M);
}

int main(void) {
    ll n = 1000000000000000000LL; /* 10^18 */
    int ps[] = {83, 89, 97};
    ll ss[3], ms[3];

    for (int i = 0; i < 3; i++) {
        ss[i] = compute_s_mod_p3(ps[i], n);
        ms[i] = (ll)ps[i] * ps[i] * ps[i];
    }

    ll answer = crt3(ss, ms);
    printf("%lld\n", answer);
    return 0;
}
