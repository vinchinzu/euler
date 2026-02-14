/* Project Euler Problem 753: Fermat Equation.
 * Extracted from embedded C in python/753.py
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define N 6000000

static char is_prime_arr[N + 1];

void sieve(void) {
    memset(is_prime_arr, 1, sizeof(is_prime_arr));
    is_prime_arr[0] = is_prime_arr[1] = 0;
    for (int i = 2; (long long)i * i <= N; i++) {
        if (is_prime_arr[i]) {
            for (int j = i * i; j <= N; j += i)
                is_prime_arr[j] = 0;
        }
    }
}

int main(void) {
    sieve();

    unsigned long long ans = 0;

    /* Case 1: p % 3 != 1 */
    for (int p = 2; p <= N; p++) {
        if (is_prime_arr[p] && p % 3 != 1) {
            ans += (unsigned long long)(p - 1) * (unsigned long long)(p - 2);
        }
    }

    /* Case 2: enumerate L, M with L^2 + 27*M^2 = 4p, L = 1 (mod 3) */
    int max_abs_l = (int)sqrt(4.0 * N) + 1;
    for (int abs_l = 1; abs_l <= max_abs_l; abs_l++) {
        int signs[2] = {-abs_l, abs_l};
        for (int si = 0; si < 2; si++) {
            int L = signs[si];
            int lmod3 = ((L % 3) + 3) % 3;
            if (lmod3 != 1) continue;

            int M_start = abs_l % 2;
            for (int M = M_start; ; M += 2) {
                long long p_val = ((long long)L * L + 27LL * M * M) / 4;
                if (p_val > N) break;
                if (p_val >= 2 && is_prime_arr[p_val]) {
                    ans += (unsigned long long)(L + p_val - 8) * (unsigned long long)(p_val - 1);
                }
            }
        }
    }

    printf("%llu\n", ans);
    return 0;
}
