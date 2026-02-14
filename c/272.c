/*
 * Project Euler 272: Modular Cubes, part 2
 *
 * Count n <= 10^11 such that x^3 = 1 (mod n) has more than one solution.
 * x^3 = 1 (mod n) has > 1 solution iff n has a factor p = 1 (mod 3) or 9|n.
 *
 * Equivalently, count n that are products of at least K=5 factors from
 * {primes p = 1 mod 3} times a "smooth" cofactor from {1-mod-3 smooth numbers}.
 *
 * Uses Lucy DP style prefix sums.
 */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <math.h>

typedef long long ll;

#define MAX_SIEVE 200000

static int all_primes[MAX_SIEVE];
static int num_all_primes = 0;

/* Primes = 1 mod 3 */
static int ps[MAX_SIEVE];
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
            all_primes[num_all_primes++] = i;
            if (i % 3 == 1) ps[num_ps++] = i;
        }
    free(is_p);
}

/* prod_qs: numbers that are products of primes NOT = 1 mod 3 (and not divisible by 9 for case 2) */
/* We build prefix sums of such numbers */

static ll N;
static int K = 5;

static char *prod_qs = NULL;
static int max_q;
static ll *prefix_sum = NULL;

static void build_prefix_sum(void) {
    prefix_sum = (ll *)calloc(max_q + 2, sizeof(ll));
    for (int i = 0; i <= max_q; i++)
        prefix_sum[i + 1] = prefix_sum[i] + (prod_qs[i] ? i : 0);
}

static ll ans = 0;

/* Binary search: find index of last ps[i] <= val */
static int upper_bound(int start, int end, int val) {
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
    int hi = upper_bound(index, num_ps - 1, (int)max_prime);
    if (hi < index) return;

    ll sqrt_limit = (ll)sqrt((double)(N / prod));
    int mid = upper_bound(index, hi, (int)sqrt_limit);

    /* Single-power primes: from mid+1 to hi */
    for (int ni = mid + 1; ni <= hi; ni++) {
        ll new_prod = prod * ps[ni];
        ll limit = N / new_prod;
        if (limit > max_q) limit = max_q;
        ans += new_prod * prefix_sum[limit + 1];
    }

    /* Multi-power primes: from index to mid */
    for (int ni = index; ni <= mid; ni++) {
        ll new_prod = prod * ps[ni];
        while (new_prod <= N) {
            ll limit = N / new_prod;
            if (limit > max_q) limit = max_q;
            ans += new_prod * prefix_sum[limit + 1];
            new_prod *= ps[ni];
        }
    }
}

/* Stack-based enumeration */
typedef struct {
    int index;
    ll prod;
    int num_ps_used;
} Frame;

static void run_helper(ll start_prod, int start_num_ps) {
    Frame stack[100];
    int sp = 0;
    stack[sp++] = (Frame){0, start_prod, start_num_ps};

    while (sp > 0) {
        Frame f = stack[--sp];
        int index = f.index;
        ll prod = f.prod;
        int nps = f.num_ps_used;

        if (nps >= K) {
            ll limit = N / prod;
            if (limit > max_q) limit = max_q;
            ans += prod * prefix_sum[limit + 1];
            continue;
        }

        int remaining = K - nps;
        if (remaining == 1) {
            accumulate_last_level(index, prod);
            continue;
        }

        int max_start = num_ps - remaining;
        for (int ni = index; ni <= max_start; ni++) {
            /* Check if we can fit remaining primes */
            ll min_prod = prod;
            int ok = 1;
            for (int i = 0; i < remaining; i++) {
                if (ni + i >= num_ps) { ok = 0; break; }
                min_prod *= ps[ni + i];
                if (min_prod > N) { ok = 0; break; }
            }
            if (!ok) break;

            ll new_prod = prod * ps[ni];
            while (new_prod <= N) {
                if (sp < 100)
                    stack[sp++] = (Frame){ni + 1, new_prod, nps + 1};
                new_prod *= ps[ni];
            }
        }
    }
}

int main(void) {
    N = 100000000000LL; /* 10^11 */

    /* Need primes up to N/9/p1/p2/p3 for K-2 smallest primes */
    /* Smallest p=1 mod 3 primes: 7, 13, 19, 31, 37 */
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

    /* Mark out numbers divisible by primes = 1 mod 3 */
    for (int i = 0; i < num_ps && ps[i] <= max_q; i++)
        for (int j = ps[i]; j <= max_q; j += ps[i])
            prod_qs[j] = 0;

    build_prefix_sum();

    /* Case 1: with factor of 9 */
    run_helper(9, 1);

    /* Case 2: without factor of 3 - exclude multiples of 9 from prod_qs */
    for (int i = 9; i <= max_q; i += 9)
        prod_qs[i] = 0;
    free(prefix_sum);
    build_prefix_sum();

    /* Case 2: without factor of 3 */
    run_helper(1, 0);

    printf("%lld\n", ans);

    free(prod_qs);
    free(prefix_sum);
    return 0;
}
