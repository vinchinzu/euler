/*
 * Project Euler 668 - Square Root Smooth Numbers
 *
 * Lucy DP to count primes, then count numbers n <= N whose largest
 * prime factor is <= sqrt(n).
 */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <math.h>

typedef long long ll;
typedef __int128 lll;

ll N;
int L;
ll *small_vals;  /* small_vals[i] = pi(i) for i <= L */
ll *large_vals;  /* large_vals[i] = pi(N/i) for i <= L */

void lucy_dp() {
    for (int i = 1; i <= L; i++) {
        small_vals[i] = i - 1;
        large_vals[i] = N / i - 1;
    }

    for (ll p = 2; p <= L; p++) {
        if (small_vals[p] == small_vals[p - 1]) continue;

        ll pi_p_minus_1 = small_vals[p - 1];
        ll p2 = p * p;

        for (int i = 1; i <= L && N / i >= p2; i++) {
            ll v = N / i;
            ll v_div_p = v / p;
            ll sub;
            if (v_div_p <= L)
                sub = small_vals[v_div_p];
            else
                sub = large_vals[N / v_div_p];
            large_vals[i] -= (sub - pi_p_minus_1);
        }

        for (int i = L; i >= p2; i--) {
            small_vals[i] -= (small_vals[i / p] - pi_p_minus_1);
        }
    }
}

ll pi(ll v) {
    if (v <= 0) return 0;
    if (v <= L) return small_vals[v];
    return large_vals[N / v];
}

int *sieve_primes;
int num_primes;

void gen_primes(int limit) {
    char *is_composite = calloc(limit + 1, 1);
    sieve_primes = malloc((limit + 1) * sizeof(int));
    num_primes = 0;
    for (int i = 2; i <= limit; i++) {
        if (!is_composite[i]) {
            sieve_primes[num_primes++] = i;
            if ((ll)i * i <= limit) {
                for (int j = i * i; j <= limit; j += i)
                    is_composite[j] = 1;
            }
        }
    }
    free(is_composite);
}

int main() {
    N = 10000000000LL;
    L = (int)sqrt((double)N) + 1;
    while ((ll)L * L > N) L--;

    small_vals = calloc(L + 2, sizeof(ll));
    large_vals = calloc(L + 2, sizeof(ll));

    lucy_dp();

    ll ans = N;

    int prime_limit = (int)(N / L);
    gen_primes(prime_limit);

    for (int i = 0; i < num_primes && sieve_primes[i] <= prime_limit; i++) {
        ans -= sieve_primes[i];
    }

    for (int d = 1; d < L; d++) {
        ll count = pi(N / d) - pi(N / (d + 1));
        ans -= (ll)d * count;
    }

    printf("%lld\n", ans);

    free(small_vals);
    free(large_vals);
    free(sieve_primes);
    return 0;
}
