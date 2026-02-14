/*
 * Project Euler 423 - Consecutive die throws
 *
 * C(n) = number of outcomes of throwing a 6-sided die n times such that
 * consecutive identical pairs does not exceed pi(n).
 * Extracted from embedded C in python/423.py.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define N 50000000
#define K 6
#define MOD 1000000007LL

static char is_prime_arr[N + 1];
static long long inv_arr[N + 1];

int main() {
    int i;

    /* Sieve of Eratosthenes */
    memset(is_prime_arr, 1, sizeof(is_prime_arr));
    is_prime_arr[0] = is_prime_arr[1] = 0;
    for (i = 2; (long long)i * i <= N; i++) {
        if (is_prime_arr[i]) {
            for (int j = i * i; j <= N; j += i)
                is_prime_arr[j] = 0;
        }
    }

    /* Modular inverses */
    inv_arr[0] = 0;
    inv_arr[1] = 1;
    for (i = 2; i <= N; i++) {
        inv_arr[i] = (MOD - MOD / i * inv_arr[MOD % i] % MOD) % MOD;
    }

    long long f = 1;
    long long R = 1;
    int pi_n = 0;

    long long ans = (K * f) % MOD;  /* C(1) = 6 */

    for (int n = 2; n <= N; n++) {
        if (is_prime_arr[n]) {
            long long R_new = R * ((n - 1) % MOD) % MOD * inv_arr[pi_n + 1] % MOD;
            f = ((long long)K * f % MOD - R % MOD + R_new + MOD) % MOD;
            R = R_new;
            pi_n++;
        } else {
            long long f_new = ((long long)K * f % MOD - R % MOD + MOD) % MOD;
            long long R_new;
            if (n - 1 > pi_n) {
                R_new = R * ((n - 1) % MOD) % MOD * (K - 1) % MOD * inv_arr[n - 1 - pi_n] % MOD;
            } else {
                R_new = 1;
            }
            f = f_new;
            R = R_new;
        }

        long long C_n = (long long)K * f % MOD;
        ans = (ans + C_n) % MOD;
    }

    printf("%lld\n", ans);
    return 0;
}
