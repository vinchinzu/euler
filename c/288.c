/*
 * Project Euler 288 - An Enormous Factorial
 *
 * N(P,Q) = sum_{n=0}^{Q} T_n * P^n, where T_n = S_n mod P,
 * S_0 = 290797, S_{n+1} = S_n^2 mod 50515093, P = 61, Q = 10^7.
 * Find NF(P,Q) mod P^10, where NF = number of factors of P in N(P,Q)!
 * By Legendre: NF = sum_{i=1}^{Q} floor(N/P^i).
 *
 * Uses __int128 for intermediate multiplications to avoid overflow.
 */
#include <stdio.h>

typedef long long ll;
typedef __int128 i128;

static ll mulmod(ll a, ll b, ll m) {
    return (ll)((i128)a * b % m);
}

int main(void) {
    const int P = 61;
    const int Q = 10000000;
    const int E = 10;
    const ll BBS_MOD = 50515093;

    /* M = P^E */
    ll M = 1;
    for (int i = 0; i < E; i++) M *= P;

    /* Precompute P powers */
    ll pows[E + 1];
    pows[0] = 1;
    for (int i = 1; i <= E; i++) pows[i] = pows[i - 1] * P;

    /* inv(P-1, M) using extended GCD */
    ll inv_pm1;
    {
        ll a = P - 1, b = M;
        ll x0 = 1, x1 = 0;
        ll a0 = a, b0 = b;
        while (b0 > 0) {
            ll q = a0 / b0;
            ll tmp = b0;
            b0 = a0 - q * b0;
            a0 = tmp;
            tmp = x1;
            x1 = x0 - q * x1;
            x0 = tmp;
        }
        inv_pm1 = x0 % M;
        if (inv_pm1 < 0) inv_pm1 += M;
    }

    ll coeff_full = mulmod(M - 1, inv_pm1, M);

    ll coeff_small[E];
    coeff_small[0] = 0;
    for (int k = 1; k < E; k++) {
        coeff_small[k] = mulmod(pows[k] - 1, inv_pm1, M);
    }

    /* Generate T values and accumulate */
    ll ans = 0;
    ll s = 290797;

    /* Skip T[0] - doesn't contribute (sum starts at i=1, so k>=1) */
    s = (s * s) % BBS_MOD;

    /* k=1 to E-1: small coefficients */
    for (int k = 1; k < E; k++) {
        ll tk = s % P;
        s = (s * s) % BBS_MOD;
        ans = (ans + mulmod(tk, coeff_small[k], M)) % M;
    }

    /* k=E to Q: full coefficient */
    ll chunk = 0;
    for (int k = E; k <= Q; k++) {
        ll tk = s % P;
        s = (s * s) % BBS_MOD;
        chunk += tk;
        if (k % 1000000 == 0 || k == Q) {
            ans = (ans + mulmod(coeff_full, chunk % M, M)) % M;
            chunk = 0;
        }
    }

    printf("%lld\n", ans);
    return 0;
}
