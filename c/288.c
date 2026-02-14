/*
 * Project Euler 288 - An Enormous Factorial
 *
 * N(P,Q) = sum_{n=0}^{Q} T_n * P^n, where T_n = S_n mod P,
 * S_0 = 290797, S_{n+1} = S_n^2 mod 50515093, P = 61, Q = 10^7.
 * Find NF(P,Q) mod P^10, where NF = number of factors of P in N(P,Q)!
 * By Legendre: NF = sum_{i=1}^{Q} floor(N/P^i).
 */
#include <stdio.h>

int main(void) {
    const int P = 61;
    const int Q = 10000000;
    const int E = 10;
    const long long BBS_MOD = 50515093;

    /* M = P^E */
    long long M = 1;
    for (int i = 0; i < E; i++) M *= P;

    /* Precompute P powers */
    long long pows[E + 1];
    pows[0] = 1;
    for (int i = 1; i <= E; i++) pows[i] = pows[i - 1] * P;

    /* inv(P-1, M) */
    /* Extended GCD or just use pow(P-1, M/gcd - 1, M) -- but P-1=60 and M=61^10 are coprime */
    /* Fermat won't work since M isn't prime. Use extended GCD. */
    long long inv_pm1;
    {
        long long a = P - 1, b = M;
        long long x0 = 1, x1 = 0;
        long long a0 = a, b0 = b;
        while (b0 > 0) {
            long long q = a0 / b0;
            long long tmp = b0;
            b0 = a0 - q * b0;
            a0 = tmp;
            tmp = x1;
            x1 = x0 - q * x1;
            x0 = tmp;
        }
        inv_pm1 = x0 % M;
        if (inv_pm1 < 0) inv_pm1 += M;
    }

    long long coeff_full = ((M - 1) % M * inv_pm1) % M;

    long long coeff_small[E];
    coeff_small[0] = 0;
    for (int k = 1; k < E; k++) {
        coeff_small[k] = ((pows[k] - 1) % M * inv_pm1) % M;
    }

    /* Generate T values and accumulate */
    long long ans = 0;
    long long s = 290797;

    /* We need T[0]..T[Q]. T[0] doesn't contribute (k starts at 1). */
    /* Generate and accumulate on the fly */

    /* First: skip T[0] */
    /* T[0] = s % P but not used */
    long long t0 = s % P;
    (void)t0;
    s = (s * s) % BBS_MOD;

    /* k=1 to E-1: small coefficients */
    for (int k = 1; k < E; k++) {
        long long tk = s % P;
        s = (s * s) % BBS_MOD;
        ans = (ans + tk * coeff_small[k]) % M;
    }

    /* k=E to Q: full coefficient */
    long long chunk = 0;
    for (int k = E; k <= Q; k++) {
        long long tk = s % P;
        s = (s * s) % BBS_MOD;
        chunk += tk;
        if (k % 1000000 == 0 || k == Q) {
            ans = (ans + coeff_full % M * (chunk % M)) % M;
            chunk = 0;
        }
    }

    printf("%lld\n", ans);
    return 0;
}
