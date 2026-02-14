/*
 * Project Euler Problem 472: Comfortable distance (lightbulb placement)
 *
 * Pattern-based approach: compute sumF(N) for N=10^12, mod 10^8.
 */
#include <stdio.h>

typedef long long ll;

#define MOD 100000000LL

static ll tr(ll n) {
    /* Triangular number n*(n+1)/2 mod MOD */
    return n % MOD * ((n + 1) % MOD) % MOD * 50000000LL % MOD;
    /* 50000000 = inverse of 2 mod 10^8 ... actually 10^8 is even, can't invert 2.
     * Use direct computation instead. */
}

int main(void) {
    ll N_val = 1000000000000LL; /* 10^12 */
    ll M = MOD;

    /* Triangular number helper: n*(n+1)/2 mod M */
    /* Since M=10^8 is even, we need to be careful. n*(n+1) is always even. */
    #define TR(n) ((((n) % M) * (((n) + 1) % M) / 2) % M)

    ll F[] = {1, 2, 2, 4, 3, 6, 2, 6, 3};
    ll sumf = 0;
    ll index = 0;

    for (int i = 0; i < 9; i++) {
        sumf = (sumf + F[i]) % M;
        index++;
        if (index >= N_val) {
            printf("%lld\n", sumf);
            return 0;
        }
    }

    while (1) {
        ll length = index / 4;
        sumf = (sumf + 8) % M;
        index++;
        if (index >= N_val) {
            printf("%lld\n", sumf);
            return 0;
        }

        ll l = 1;
        while (l <= length / 2) {
            /* Rising range */
            ll count = N_val - index;
            if (count > l) count = l;
            sumf = (sumf + 2 * TR(count) % M) % M;
            index += l;
            if (index >= N_val) {
                printf("%lld\n", sumf);
                return 0;
            }

            /* Peak */
            sumf = (sumf + 2 * ((2 * l + 1) % M)) % M;
            index++;
            if (index >= N_val) {
                printf("%lld\n", sumf);
                return 0;
            }

            /* Falling range */
            ll count2 = l - (N_val - index);
            if (count2 < 1) count2 = 1;
            sumf = (sumf + 2 * ((TR(l) - TR(count2) + M) % M)) % M;
            index += l - 1;
            if (index >= N_val) {
                printf("%lld\n", sumf);
                return 0;
            }

            l *= 2;
        }

        /* Center rising range */
        ll count = N_val - index;
        if (count > length) count = length;
        sumf = (sumf + 2 * TR(count) % M) % M;
        index += length;
        if (index >= N_val) {
            printf("%lld\n", sumf);
            return 0;
        }

        /* Center peak */
        sumf = (sumf + 3 * ((length + 1) % M)) % M;
        index++;
        if (index >= N_val) {
            printf("%lld\n", sumf);
            return 0;
        }

        /* Center falling range */
        ll count2 = length - (N_val - index) + 2;
        if (count2 < 2) count2 = 2;
        sumf = (sumf + (TR(length + 2) - TR(count2) + M) % M) % M;
        index += length;
        if (index >= N_val) {
            printf("%lld\n", sumf);
            return 0;
        }
    }
}
