/*
 * Project Euler 432 - Totient sum
 *
 * S(K, N) = sum of phi(K*i) for i=1 to N, where K=510510, N=10^11, mod 10^9.
 * Uses phi sieve + quotient enumeration for totient_sum.
 * Translated from python/432.py.
 *
 * Since totient_sum values can be huge (up to ~N^2/2 ~ 5*10^21),
 * we need __int128 for intermediate values.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef long long ll;
typedef __int128 lll;

#define N 100000000000LL
#define K 510510
#define MOD 1000000000LL

static int primes_of_K[] = {2, 3, 5, 7, 11, 13, 17};
static int n_primes = 7;

/* Sieve limit: N^(2/3) + buffer */
#define LIMIT 5000000

static int phi_arr[LIMIT + 1];
static ll sum_phi_small[LIMIT + 1];

/* Hash map for totient_sum cache */
#define HASH_SIZE (1 << 20)
#define HASH_MASK (HASH_SIZE - 1)
typedef struct { ll key; lll val; int used; } HEntry;
static HEntry ts_cache[HASH_SIZE];

static void compute_phi_sieve(void) {
    for (int i = 0; i <= LIMIT; i++) phi_arr[i] = i;
    for (int i = 2; i <= LIMIT; i++) {
        if (phi_arr[i] == i) { /* prime */
            for (int j = i; j <= LIMIT; j += i)
                phi_arr[j] -= phi_arr[j] / i;
        }
    }
    sum_phi_small[0] = 0;
    for (int i = 1; i <= LIMIT; i++)
        sum_phi_small[i] = sum_phi_small[i-1] + phi_arr[i];
}

static lll totient_sum(ll q);

static lll ts_lookup(ll q) {
    if (q <= LIMIT) return (lll)sum_phi_small[q];

    int h = (int)((unsigned long long)q * 0x9E3779B97F4A7C15ULL >> 44) & HASH_MASK;
    for (int i = 0; i < HASH_SIZE; i++) {
        int idx = (h + i) & HASH_MASK;
        if (!ts_cache[idx].used) return -1; /* sentinel: not found */
        if (ts_cache[idx].key == q) return ts_cache[idx].val;
    }
    return -1;
}

static void ts_store(ll q, lll val) {
    int h = (int)((unsigned long long)q * 0x9E3779B97F4A7C15ULL >> 44) & HASH_MASK;
    for (int i = 0; i < HASH_SIZE; i++) {
        int idx = (h + i) & HASH_MASK;
        if (!ts_cache[idx].used) {
            ts_cache[idx].key = q;
            ts_cache[idx].val = val;
            ts_cache[idx].used = 1;
            return;
        }
    }
}

static lll totient_sum(ll q) {
    if (q <= LIMIT) return (lll)sum_phi_small[q];

    lll cached = ts_lookup(q);
    if (cached != -1) return cached;

    lll result = (lll)q * (q + 1) / 2;
    ll sqrt_q = (ll)sqrtl((long double)q);

    for (ll d = 2; d <= sqrt_q; d++) {
        ll qd = q / d;
        result -= totient_sum(qd);
    }

    for (ll m = 1; m <= sqrt_q; m++) {
        if (q / m > sqrt_q) {
            ll count = q / m - q / (m + 1);
            result -= (lll)count * totient_sum(m);
        }
    }

    ts_store(q, result);
    return result;
}

/* Enumerate all d that are products of prime factors of K, with d <= N */
static ll ans;

static void enumerate_d(int idx, ll d) {
    if (idx == n_primes) {
        lll ts = totient_sum(N / d);
        ans = (ans + (ll)(ts % MOD)) % MOD;
        return;
    }
    /* Don't include this prime */
    enumerate_d(idx + 1, d);
    /* Include primes_of_K[idx] repeatedly */
    ll d2 = d * primes_of_K[idx];
    while (d2 <= N) {
        enumerate_d(idx + 1, d2);
        d2 *= primes_of_K[idx];
    }
}

int main(void) {
    compute_phi_sieve();
    memset(ts_cache, 0, sizeof(ts_cache));

    /* Precompute totient_sum for all quotient values bottom-up */
    ll sqrt_N = (ll)sqrtl((long double)N);

    /* Enumerate quotient values in increasing order */
    /* Small quotients: 1..sqrt_N */
    for (ll q = 1; q <= sqrt_N + 1; q++) {
        if (q > LIMIT) totient_sum(q);
    }
    /* Large quotients: N/k for k = sqrt_N down to 1 */
    for (ll k = sqrt_N + 1; k >= 1; k--) {
        ll q = N / k;
        if (q > LIMIT) totient_sum(q);
    }

    /* phi(K) where K = 2*3*5*7*11*13*17 */
    ll phi_K = 1;
    for (int i = 0; i < n_primes; i++)
        phi_K *= (primes_of_K[i] - 1);
    /* phi_K = 92160 */

    ans = 0;
    enumerate_d(0, 1);

    ans = ((ans % MOD) * (phi_K % MOD)) % MOD;
    ans = (ans % MOD + MOD) % MOD;

    printf("%lld\n", ans);
    return 0;
}
