/*
 * Project Euler Problem 953
 * Factorisation Nim: S(10^14) mod 10^9+7
 * Translated from embedded C++ in Python solution.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <math.h>

typedef long long ll;
typedef __int128 int128;

#define N_VAL 100000000000000LL
#define MOD 1000000007LL
#define LIMIT_PRIME 22000000
#define SMALL_M_LIMIT 100000

static bool *is_prime_vec;
static int *primes;
static int num_primes;

static int lp[SMALL_M_LIMIT + 1];
static int g_arr[SMALL_M_LIMIT + 1];
static int max_p_arr[SMALL_M_LIMIT + 1];
static bool sq[SMALL_M_LIMIT + 1];

static ll total_sum = 0;
static int *primes_small;
static int num_primes_small;

static inline ll S2_contribution(ll k) {
    if (k > N_VAL) return 0;
    ll m = (ll)sqrt((double)(N_VAL / k));
    /* Adjust sqrt */
    while ((m + 1) * (m + 1) <= N_VAL / k) m++;
    while (m * m > N_VAL / k) m--;
    if (m == 0) return 0;

    int128 v = (int128)m * (m + 1);
    v = v * (2 * m + 1);
    v /= 6;

    ll s2 = (ll)(v % MOD);
    return (k % MOD * s2) % MOD;
}

static bool is_prime_large(ll p) {
    if (p <= LIMIT_PRIME) return is_prime_vec[p];
    if (p % 2 == 0) return false;
    for (ll d = 3; d * d <= p; d += 2) {
        if (p % d == 0) return false;
    }
    return true;
}

static void dfs(int idx, ll current_m, int current_g, int q, ll limit_m) {
    int p = current_g ^ q;

    if (p > q) {
        bool valid = false;
        if (p <= LIMIT_PRIME) {
            if (is_prime_vec[p]) valid = true;
        } else {
            if (is_prime_large(p)) valid = true;
        }

        if (valid) {
            int128 k = (int128)current_m * q * p;
            if (k <= N_VAL) {
                total_sum = (total_sum + S2_contribution((ll)k)) % MOD;
            }
        }
    }

    for (int i = idx; i >= 0; --i) {
        int next_p = primes_small[i];
        ll next_m = current_m * next_p;

        if (next_m > limit_m) continue;

        dfs(i - 1, next_m, current_g ^ next_p, q, limit_m);
    }
}

int main(void) {
    /* Sieve */
    is_prime_vec = (bool *)malloc((LIMIT_PRIME + 1) * sizeof(bool));
    memset(is_prime_vec, 1, LIMIT_PRIME + 1);
    is_prime_vec[0] = is_prime_vec[1] = false;
    for (int i = 2; (ll)i * i <= LIMIT_PRIME; i++) {
        if (is_prime_vec[i]) {
            for (int j = i * i; j <= LIMIT_PRIME; j += i)
                is_prime_vec[j] = false;
        }
    }

    primes = (int *)malloc(1600000 * sizeof(int));
    num_primes = 0;
    for (int i = 2; i <= LIMIT_PRIME; i++) {
        if (is_prime_vec[i]) primes[num_primes++] = i;
    }

    /* Precompute small m arrays */
    int *pr = (int *)malloc(100000 * sizeof(int));
    int num_pr = 0;
    memset(sq, 1, sizeof(sq));
    memset(lp, 0, sizeof(lp));

    for (int i = 2; i <= SMALL_M_LIMIT; i++) {
        if (lp[i] == 0) {
            lp[i] = i;
            pr[num_pr++] = i;
            g_arr[i] = i;
            max_p_arr[i] = i;
        }
        for (int pi = 0; pi < num_pr; pi++) {
            int p = pr[pi];
            if (p > lp[i] || (ll)i * p > SMALL_M_LIMIT) break;
            lp[i * p] = p;
            max_p_arr[i * p] = max_p_arr[i];
            if (p == lp[i]) {
                sq[i * p] = false;
                g_arr[i * p] = g_arr[i] ^ p;
            } else {
                sq[i * p] = sq[i];
                g_arr[i * p] = g_arr[i] ^ p;
            }
        }
    }

    /* k=1 case */
    total_sum = (total_sum + S2_contribution(1)) % MOD;

    ll max_q = (ll)sqrt((double)(N_VAL / 2));

    primes_small = (int *)malloc(num_primes * sizeof(int));

    for (int qi = 0; qi < num_primes; qi++) {
        int q = primes[qi];
        if (q > max_q) break;

        ll q_sq = (ll)q * q;
        ll limit_m = N_VAL / q_sq;

        if (limit_m == 0) break;

        if (limit_m <= SMALL_M_LIMIT) {
            /* Direct iteration */
            for (int m = 2; m <= limit_m; m++) {
                if (sq[m]) {
                    if (max_p_arr[m] < q) {
                        int p = g_arr[m] ^ q;
                        if (p > q) {
                            bool valid = false;
                            if (p <= LIMIT_PRIME) {
                                if (is_prime_vec[p]) valid = true;
                            } else {
                                if (is_prime_large(p)) valid = true;
                            }

                            if (valid) {
                                int128 k = (int128)m * q * p;
                                total_sum = (total_sum + S2_contribution((ll)k)) % MOD;
                            }
                        }
                    }
                }
            }
        } else {
            if (2 * q_sq > N_VAL) continue;

            num_primes_small = 0;
            for (int pi = 0; pi < num_primes; pi++) {
                if (primes[pi] >= q) break;
                primes_small[num_primes_small++] = primes[pi];
            }

            dfs(num_primes_small - 1, 1, 0, q, limit_m);
        }
    }

    printf("%lld\n", total_sum);

    free(is_prime_vec);
    free(primes);
    free(pr);
    free(primes_small);
    return 0;
}
