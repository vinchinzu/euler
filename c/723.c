/* Project Euler 723: Pythagorean Quadrilaterals.
 * Count Pythagorean quadrilaterals on circles using combinatorics.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;

/* N = 5^6 * 13^3 * 17^2 * 29 * 37 * 41 * 53 * 61 */
/* Prime factors and their exponents in N */
#define NUM_PRIMES 8
static int primes[NUM_PRIMES] = {5, 13, 17, 29, 37, 41, 53, 61};
static int exps[NUM_PRIMES]   = {6,  3,  2,  1,  1,  1,  1,  1};

ll sq(ll n) { return n * n; }

ll nCr(ll n, ll r) {
    if (r < 0 || r > n) return 0;
    if (r == 0 || r == n) return 1;
    if (r > n - r) r = n - r;
    ll result = 1;
    for (ll i = 0; i < r; i++) {
        result = result * (n - i) / (i + 1);
    }
    return result;
}

ll num_factors(int *exp_arr, int len) {
    ll result = 1;
    for (int i = 0; i < len; i++)
        result *= exp_arr[i] + 1;
    return result;
}

/* Compute f(d) for the given prime factorization of d.
 * d_exps[i] is the exponent of primes[i] in d. */
ll f_from_exps(int *d_exps, int nprimes) {
    /* Count number of non-zero exponents */
    int active = 0;
    int active_exps[NUM_PRIMES];
    int active_idx[NUM_PRIMES];
    for (int i = 0; i < nprimes; i++) {
        if (d_exps[i] > 0) {
            active_idx[active] = i;
            active_exps[active] = d_exps[i];
            active++;
        }
    }
    if (active == 0) {
        /* d = 1: k_total = 1, result = 2*1*1 - 0 = 2 but actually f(1) should be trivial */
        /* For d=1, exponents are all 0, num_factors = 1 */
        ll k_total = 1;
        /* Iterate: only ds = (0,...,0) => k=1, mult=1 */
        /* 4*nCr(1,2) = 0, parallel: k_parallel = 1 (all even since all 0), so k_parallel=0, nCr(0,2)=0 */
        ll result = 0;
        /* Diameter: 2*1*1 - nCr(2,2) = 2 - 1 = 1 */
        result += 2 * k_total * sq(2 * k_total - 1) - nCr(2 * k_total, 2);
        return result;
    }

    /* Enumerate all combinations of ds */
    int axes_sizes[NUM_PRIMES];
    for (int i = 0; i < active; i++)
        axes_sizes[i] = active_exps[i] + 1;

    /* Total combinations */
    int total_combos = 1;
    for (int i = 0; i < active; i++)
        total_combos *= axes_sizes[i];

    ll result = 0;
    int ds[NUM_PRIMES];

    for (int combo = 0; combo < total_combos; combo++) {
        int tmp = combo;
        for (int i = 0; i < active; i++) {
            ds[i] = tmp % axes_sizes[i];
            tmp /= axes_sizes[i];
        }

        ll k = num_factors(ds, active);
        ll mult = 1;
        for (int i = 0; i < active; i++) {
            if (ds[i] < active_exps[i])
                mult *= 2;
        }

        /* 45 degree pairs */
        result += 4 * mult * nCr(k, 2);

        /* Parallel pairs */
        ll k_parallel = k;
        int all_even = 1;
        for (int i = 0; i < active; i++) {
            if (ds[i] % 2 != 0) { all_even = 0; break; }
        }
        if (all_even) k_parallel -= 1;
        result += 4 * mult * nCr(k_parallel, 2);
    }

    /* Diameter-based quadrilaterals */
    ll k_total = num_factors(active_exps, active);
    result += 2 * k_total * sq(2 * k_total - 1) - nCr(2 * k_total, 2);

    return result;
}

int main() {
    /* Enumerate all divisors of N by iterating over exponent combinations */
    int total_divisors = 1;
    for (int i = 0; i < NUM_PRIMES; i++)
        total_divisors *= (exps[i] + 1);

    ll ans = 0;
    int d_exps[NUM_PRIMES];

    for (int combo = 0; combo < total_divisors; combo++) {
        int tmp = combo;
        for (int i = 0; i < NUM_PRIMES; i++) {
            d_exps[i] = tmp % (exps[i] + 1);
            tmp /= (exps[i] + 1);
        }
        ans += f_from_exps(d_exps, NUM_PRIMES);
    }

    printf("%lld\n", ans);
    return 0;
}
