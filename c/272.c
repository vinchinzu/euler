/*
 * Project Euler 272: Modular Cubes, part 2
 *
 * Count sum of n <= 10^11 such that x^3 = 1 (mod n) has more than one solution.
 * n has more than one cube root of unity iff n is divisible by 9 or by a prime p = 1 (mod 3).
 *
 * The answer equals sum of n = P * Q where:
 *   P = product of at least K=5 primes p = 1 mod 3 (with multiplicity)
 *   Q = product of primes NOT = 1 mod 3 (called "q-smooth")
 *
 * Two cases: (1) 9 | n, (2) 9 does not divide n.
 *
 * Uses prefix sums over q-smooth numbers for efficient summation.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef long long ll;
typedef __int128 i128;

#define MAX_PRIMES 500000

static int all_primes[MAX_PRIMES];
static int num_all_primes = 0;

/* Primes = 1 mod 3 */
static int ps[MAX_PRIMES];
static int num_ps = 0;

static void sieve(int limit) {
    char *is_p = calloc(limit + 1, 1);
    memset(is_p, 1, limit + 1);
    is_p[0] = is_p[1] = 0;
    for (int i = 2; (ll)i * i <= limit; i++)
        if (is_p[i])
            for (int j = i * i; j <= limit; j += i)
                is_p[j] = 0;
    for (int i = 2; i <= limit; i++)
        if (is_p[i]) {
            if (num_all_primes < MAX_PRIMES)
                all_primes[num_all_primes++] = i;
            if (i % 3 == 1 && num_ps < MAX_PRIMES)
                ps[num_ps++] = i;
        }
    free(is_p);
}

static ll N;
static int K = 5;

static char *prod_qs = NULL;
static int max_q;
static ll *prefix_sum = NULL;

static void build_prefix_sum(void) {
    if (prefix_sum) free(prefix_sum);
    prefix_sum = (ll *)calloc(max_q + 2, sizeof(ll));
    for (int i = 0; i <= max_q; i++)
        prefix_sum[i + 1] = prefix_sum[i] + (prod_qs[i] ? i : 0);
}

static ll ans = 0;

/* Binary search: find index of last ps[i] <= val, in range [start, end] */
static int upper_bound(int start, int end, ll val) {
    int lo = start, hi = end, res = start - 1;
    while (lo <= hi) {
        int mid = (lo + hi) / 2;
        if (ps[mid] <= val) { res = mid; lo = mid + 1; }
        else hi = mid - 1;
    }
    return res;
}

static void accumulate_last_level(int index, ll prod) {
    ll max_prime = N / prod;
    int hi = upper_bound(index, num_ps - 1, max_prime);
    if (hi < index) return;

    ll sqrt_limit = (ll)sqrt((double)(N / prod));
    int mid = upper_bound(index, hi, sqrt_limit);

    /* Single-power primes: from mid+1 to hi */
    for (int ni = mid + 1; ni <= hi; ni++) {
        ll new_prod = prod * ps[ni];
        ll limit = N / new_prod;
        if (limit > max_q) limit = max_q;
        ans += (i128)new_prod * prefix_sum[limit + 1];
    }

    /* Multi-power primes: from index to mid */
    for (int ni = index; ni <= mid; ni++) {
        ll new_prod = prod * ps[ni];
        while (new_prod <= N) {
            ll limit = N / new_prod;
            if (limit > max_q) limit = max_q;
            ans += (i128)new_prod * prefix_sum[limit + 1];
            if (new_prod > N / ps[ni]) break;
            new_prod *= ps[ni];
        }
    }
}

/* Recursive enumeration instead of stack-based (cleaner and no size limit) */
static void enumerate(int index, ll prod, int nps) {
    if (nps >= K) {
        ll limit = N / prod;
        if (limit > max_q) limit = max_q;
        ans += (i128)prod * prefix_sum[limit + 1];
        return;
    }

    int remaining = K - nps;
    if (remaining == 1) {
        accumulate_last_level(index, prod);
        return;
    }

    int max_start = num_ps - remaining;
    for (int ni = index; ni <= max_start; ni++) {
        /* Check if we can fit remaining primes */
        ll min_prod = prod;
        int ok = 1;
        for (int i = 0; i < remaining; i++) {
            if (ni + i >= num_ps) { ok = 0; break; }
            if (min_prod > N / ps[ni + i]) { ok = 0; break; }
            min_prod *= ps[ni + i];
        }
        if (!ok) break;

        ll new_prod = prod * ps[ni];
        while (new_prod <= N) {
            enumerate(ni + 1, new_prod, nps + 1);
            if (new_prod > N / ps[ni]) break;
            new_prod *= ps[ni];
        }
    }
}

int main(void) {
    N = 100000000000LL; /* 10^11 */

    /* Smallest p=1 mod 3 primes: 7, 13, 19, 31, 37, ... */
    int small_ps[] = {7, 13, 19, 31, 37, 43, 61, 67};

    ll max_p_val = N / 9;
    for (int i = 0; i < K - 2; i++) max_p_val /= small_ps[i];

    ll max_q_val = N / 9;
    for (int i = 0; i < K - 1; i++) max_q_val /= small_ps[i];

    sieve((int)max_p_val + 10);

    max_q = (int)max_q_val;
    if (max_q < 1) max_q = 1;

    /* Build prod_qs sieve: numbers not divisible by any prime = 1 mod 3 */
    prod_qs = (char *)calloc(max_q + 1, 1);
    memset(prod_qs, 1, max_q + 1);
    prod_qs[0] = 0;

    for (int i = 0; i < num_ps && ps[i] <= max_q; i++)
        for (int j = ps[i]; j <= max_q; j += ps[i])
            prod_qs[j] = 0;

    build_prefix_sum();

    /* Case 1: with factor of 9 */
    enumerate(0, 9, 1);

    /* Case 2: without factor of 3 - exclude multiples of 9 from prod_qs */
    for (int i = 9; i <= max_q; i += 9)
        prod_qs[i] = 0;
    build_prefix_sum();

    /* Case 2: without factor of 3 */
    enumerate(0, 1, 0);

    printf("%lld\n", (ll)ans);

    free(prod_qs);
    free(prefix_sum);
    return 0;
}
