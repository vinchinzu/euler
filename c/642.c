/*
 * Project Euler 642 - Sum of largest prime factors
 * Lucy hedgehog for sum of primes, then recursive enumeration.
 */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

typedef long long ll;

#define MOD 1000000000LL

static ll N;
static int R; /* isqrt(N) */

/* Lucy hedgehog: small[k] = sum of primes <= k, big[i] = sum of primes <= N/i */
static ll *small_arr;
static ll *big_arr;

static void compute_prime_sums(void) {
    R = (int)sqrt((double)N);
    while ((ll)(R+1)*(R+1) <= N) R++;
    while ((ll)R*R > N) R--;

    small_arr = (ll *)calloc(R + 1, sizeof(ll));
    big_arr = (ll *)calloc(R + 1, sizeof(ll));

    for (int k = 2; k <= R; k++) {
        small_arr[k] = ((ll)k * (k + 1) / 2 - 1) % MOD;
    }
    for (int i = 1; i <= R; i++) {
        ll v = N / i;
        big_arr[i] = (v % MOD) * ((v + 1) % MOD) % MOD;
        big_arr[i] = (big_arr[i] * 500000000LL) % MOD; /* inv2 mod 10^9 = 500000000 */
        big_arr[i] = (big_arr[i] - 1 + MOD) % MOD;
    }

    for (int p = 2; p <= R; p++) {
        if (small_arr[p] == small_arr[p - 1]) continue;
        ll sp = small_arr[p - 1];
        ll p2 = (ll)p * p;

        for (int i = 1; i <= R && N / i >= p2; i++) {
            ll d = N / i / p;
            if (d <= R) {
                big_arr[i] = (big_arr[i] - p * ((small_arr[d] - sp + MOD) % MOD) % MOD + MOD) % MOD;
            } else {
                int idx = (int)(N / d);
                big_arr[i] = (big_arr[i] - p * ((big_arr[idx] - sp + MOD) % MOD) % MOD + MOD) % MOD;
            }
        }

        for (int k = R; k >= p2; k--) {
            small_arr[k] = (small_arr[k] - p * ((small_arr[k / p] - sp + MOD) % MOD) % MOD + MOD) % MOD;
        }
    }
}

static ll get_sum(ll v) {
    if (v <= R)
        return small_arr[v];
    else
        return big_arr[(int)(N / v)];
}

/* Sieve primes up to R */
static int *primes;
static int nprimes;

static void sieve_primes(void) {
    char *is_p = (char *)malloc(R + 1);
    memset(is_p, 1, R + 1);
    is_p[0] = is_p[1] = 0;
    int sq = (int)sqrt((double)R);
    for (int i = 2; i <= sq; i++)
        if (is_p[i])
            for (int j = i*i; j <= R; j += i)
                is_p[j] = 0;
    nprimes = 0;
    for (int i = 2; i <= R; i++)
        if (is_p[i]) nprimes++;
    primes = (int *)malloc(nprimes * sizeof(int));
    int idx = 0;
    for (int i = 2; i <= R; i++)
        if (is_p[i]) primes[idx++] = i;
    free(is_p);
}

static ll ans;

static void helper(int min_index, ll n) {
    ll max_p = N / n;
    if (min_index >= nprimes) return;
    ll min_p = primes[min_index];
    if (min_p > max_p) return;

    ll contrib = (get_sum(max_p) - get_sum(min_p - 1) + MOD) % MOD;
    ans = (ans + contrib) % MOD;

    for (int index = min_index; index < nprimes; index++) {
        ll p = primes[index];
        if (n * p > N / p) break; /* n * p * p > N */
        helper(index, n * p);
    }
}

int main(void) {
    N = 201820182018LL;

    compute_prime_sums();
    sieve_primes();

    ans = 0;
    helper(0, 1);
    printf("%lld\n", ans);

    free(small_arr);
    free(big_arr);
    free(primes);
    return 0;
}
