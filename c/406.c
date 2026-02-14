/*
 * Project Euler Problem 406: Guessing Game.
 *
 * C(n, a, b) = minimum worst-case cost to guess a number in [1, n].
 * f(c) = max n guessable with budget c = 1 + f(c-a) + f(c-b).
 *
 * Compute sum_{k=1}^{30} C(10^12, sqrt(k), sqrt(F_k)).
 */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

typedef long long ll;

ll fibonacci(int k) {
    if (k <= 2) return 1;
    ll a = 1, b = 1;
    for (int i = 2; i < k; i++) {
        ll c = a + b;
        a = b;
        b = c;
    }
    return b;
}

/* Compare doubles for sorting */
int cmp_double(const void *a, const void *b) {
    double da = *(const double *)a;
    double db = *(const double *)b;
    if (da < db) return -1;
    if (da > db) return 1;
    return 0;
}

/* Binary search: find largest index where sorted_costs[idx] <= target */
int bisect_right(double *arr, int n, double target) {
    int lo = 0, hi = n;
    while (lo < hi) {
        int mid = (lo + hi) / 2;
        if (arr[mid] <= target) lo = mid + 1;
        else hi = mid;
    }
    return lo - 1;
}

double C_func(ll n, double a_val, double b_val) {
    double max_budget = 80.0 * (a_val > b_val ? a_val : b_val);

    /* Generate all candidate cost values c = i*a + j*b */
    /* Estimate max number of costs */
    int max_i = (int)(max_budget / a_val) + 2;
    int max_j = (int)(max_budget / b_val) + 2;
    int cap = (max_i + 1) * (max_j + 1);
    double *costs = (double *)malloc(cap * sizeof(double));
    int ncosts = 0;

    for (int i = 0; i * a_val <= max_budget; i++) {
        for (int j = 0; i * a_val + j * b_val <= max_budget; j++) {
            costs[ncosts++] = i * a_val + j * b_val;
        }
    }

    /* Sort and deduplicate */
    qsort(costs, ncosts, sizeof(double), cmp_double);
    int unique = 0;
    for (int i = 0; i < ncosts; i++) {
        if (i == 0 || costs[i] - costs[unique-1] > 1e-12) {
            costs[unique++] = costs[i];
        }
    }
    ncosts = unique;

    ll *f_vals = (ll *)malloc(ncosts * sizeof(ll));
    double eps = 1e-9;

    for (int idx = 0; idx < ncosts; idx++) {
        double c = costs[idx];
        if (c < -eps) {
            f_vals[idx] = 0;
            continue;
        }

        double target_a = c - a_val + eps;
        int pos_a = bisect_right(costs, idx + 1, target_a);
        ll fa = (pos_a >= 0) ? f_vals[pos_a] : 0;

        double target_b = c - b_val + eps;
        int pos_b = bisect_right(costs, idx + 1, target_b);
        ll fb = (pos_b >= 0) ? f_vals[pos_b] : 0;

        ll f_c = 1 + fa + fb;

        if (f_c >= n) {
            double result = c;
            free(costs);
            free(f_vals);
            return result;
        }

        f_vals[idx] = f_c;
    }

    double result = costs[ncosts - 1];
    free(costs);
    free(f_vals);
    return result;
}

int main(void) {
    ll n = 1000000000000LL;  /* 10^12 */
    double total = 0.0;

    for (int k = 1; k <= 30; k++) {
        double a = sqrt((double)k);
        double b = sqrt((double)fibonacci(k));
        double c = C_func(n, a, b);
        total += c;
    }

    printf("%.8f\n", total);
    return 0;
}
