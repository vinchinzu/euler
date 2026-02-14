/*
 * Project Euler 700 - Eulercoin
 *
 * Find the sum of all Eulercoins.
 * e(n) = (1504170715041707 * n) mod 4503599627370517.
 * A Eulercoin is e(n) when it's smaller than all previous e(k) for k < n.
 *
 * Two-phase approach:
 * Phase 1: scan forward for small n.
 * Phase 2: scan backward using modular inverse for small e values.
 *
 * Extracted from embedded C in Python solution.
 */
#include <stdio.h>
#include <math.h>

typedef unsigned long long ull;

ull mod_inverse(ull a, ull m) {
    long long t = 0, new_t = 1;
    long long r = (long long)m, new_r = (long long)(a % m);
    while (new_r != 0) {
        long long q = r / new_r;
        long long tmp;
        tmp = new_t; new_t = t - q * new_t; t = tmp;
        tmp = new_r; new_r = r - q * new_r; r = tmp;
    }
    if (t < 0) t += (long long)m;
    return (ull)t;
}

int main(void) {
    ull N_val = 1504170715041707ULL;
    ull M = 4503599627370517ULL;
    ull L = (ull)sqrt((double)M);

    ull min_el = M;
    ull ans = 0;
    ull el = N_val;
    for (ull n = 1; n <= L; n++) {
        if (el < min_el) {
            min_el = el;
            ans += el;
        }
        el = (el + N_val) % M;
    }

    ull mod_inv = mod_inverse(N_val, M);
    ull min_n = M;
    ull n_val = mod_inv;
    for (ull e = 1; e < min_el; e++) {
        if (n_val < min_n) {
            min_n = n_val;
            ans += e;
        }
        n_val = (n_val + mod_inv) % M;
    }

    printf("%llu\n", ans);
    return 0;
}
