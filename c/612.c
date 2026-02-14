/*
 * Project Euler Problem 612: Friend numbers
 *
 * Count pairs (p,q) with 1<=p<q<10^N sharing at least one common digit.
 * Uses inclusion-exclusion over digit bitmasks.
 */
#include <stdio.h>
#include <stdint.h>

typedef long long ll;
typedef __int128 i128;

#define N_VAL 18
#define M_VAL 1000267129LL
#define B_VAL 10

ll mod_inv(ll a, ll m) {
    ll t = 0, new_t = 1, r = m, new_r = a % m;
    while (new_r != 0) {
        ll q = r / new_r;
        ll tmp = new_t; new_t = t - q * new_t; t = tmp;
        tmp = new_r; new_r = r - q * new_r; r = tmp;
    }
    if (t < 0) t += m;
    return t;
}

ll pow_mod(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) result = (i128)result * base % mod;
        base = (i128)base * base % mod;
        exp >>= 1;
    }
    return result;
}

int bit_count(int n) {
    int c = 0;
    while (n) { c += n & 1; n >>= 1; }
    return c;
}

int main(void) {
    ll f[1 << B_VAL];
    for (int i = 0; i < (1 << B_VAL); i++) f[i] = 0;

    for (int subset = 1; subset < (1 << B_VAL); subset++) {
        int n = bit_count(subset);
        for (int d = 1; d <= N_VAL; d++) {
            f[subset] = (f[subset] + pow_mod(n, d, M_VAL)) % M_VAL;
            if (subset & 1) { /* includes digit 0 */
                f[subset] = (f[subset] - pow_mod(n, d - 1, M_VAL) + M_VAL) % M_VAL;
            }
        }
        /* Subtract strict subsets */
        for (int ss = (subset - 1) & subset; ss > 0; ss = (ss - 1) & subset) {
            f[subset] = (f[subset] - f[ss] + M_VAL) % M_VAL;
        }
    }

    ll ans = 0;
    for (int s1 = 1; s1 < (1 << B_VAL); s1++) {
        for (int s2 = 1; s2 < (1 << B_VAL); s2++) {
            if ((s1 & s2) > 0) {
                ans = (ans + (i128)f[s1] * f[s2]) % M_VAL;
            }
        }
    }

    ans = (ans - (pow_mod(B_VAL, N_VAL, M_VAL) - 1 + M_VAL) % M_VAL + M_VAL) % M_VAL;
    ans = (i128)ans * mod_inv(2, M_VAL) % M_VAL;

    printf("%lld\n", ans);
    return 0;
}
