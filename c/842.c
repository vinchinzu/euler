/*
 * Project Euler 842: n-Star Polygons
 *
 * T(n) = sum of I(S) over all n-star polygons.
 * For odd n: all intersections have multiplicity 2, simple formula.
 * For even n: compute multiplicities geometrically.
 * Sum T(n) for n=3..60, mod 10^9+7.
 */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

#define MOD 1000000007LL
typedef long long ll;
typedef __int128 i128;

/* Exact factorials for small n (n <= 60) using __int128 */
static i128 fact_exact[62];

static void init_factorials(void) {
    fact_exact[0] = 1;
    for (int i = 1; i <= 61; i++)
        fact_exact[i] = fact_exact[i-1] * i;
}

static double get_x(int k, int n) {
    double angle = 2.0 * M_PI * k / n;
    return cos(angle);
}
static double get_y(int k, int n) {
    double angle = 2.0 * M_PI * k / n;
    return sin(angle);
}

static int get_intersection(double x1, double y1, double x2, double y2,
                            double x3, double y3, double x4, double y4,
                            double *px, double *py) {
    double denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
    if (fabs(denom) < 1e-15) return 0;
    double t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / denom;
    *px = x1 + t * (x2 - x1);
    *py = y1 + t * (y2 - y1);
    return 1;
}

/* Hash table for grouping intersection points */
#define HT_SIZE (1 << 20)
#define HT_MASK (HT_SIZE - 1)

typedef struct ht_entry {
    long long kx, ky;
    int count;
    struct ht_entry *next;
} ht_entry;

static ht_entry *ht_table[HT_SIZE];
static ht_entry ht_pool[600000];
static int ht_pool_idx;

static void ht_clear(void) {
    memset(ht_table, 0, sizeof(ht_table));
    ht_pool_idx = 0;
}

static unsigned int hash2(long long a, long long b) {
    unsigned long long h = (unsigned long long)a * 2654435761ULL ^ (unsigned long long)b * 40503ULL;
    return (unsigned int)(h & HT_MASK);
}

static void ht_add(long long kx, long long ky) {
    unsigned int h = hash2(kx, ky);
    for (ht_entry *e = ht_table[h]; e; e = e->next) {
        if (e->kx == kx && e->ky == ky) { e->count++; return; }
    }
    ht_entry *e = &ht_pool[ht_pool_idx++];
    e->kx = kx; e->ky = ky; e->count = 1; e->next = ht_table[h];
    ht_table[h] = e;
}

static ll compute_t(int n) {
    if (n < 4) return 0;

    if (n % 2 == 1) {
        /* All intersections have multiplicity 2 */
        /* T(n) = binom(n,4) * 2 * (n-3)! mod MOD */
        i128 bn4 = fact_exact[n] / (fact_exact[4] * fact_exact[n-4]);
        i128 result = bn4 * 2 * fact_exact[n-3];
        return (ll)(result % MOD);
    }

    /* Even n: find multiplicities */
    ht_clear();

    for (int a = 0; a < n; a++) {
        for (int b = a + 1; b < n; b++) {
            for (int c = b + 1; c < n; c++) {
                for (int d = c + 1; d < n; d++) {
                    /* Diagonals (a,c) and (b,d) intersect inside */
                    double px, py;
                    if (get_intersection(get_x(a,n), get_y(a,n),
                                         get_x(c,n), get_y(c,n),
                                         get_x(b,n), get_y(b,n),
                                         get_x(d,n), get_y(d,n),
                                         &px, &py)) {
                        long long kx = (long long)round(px * 1e9);
                        long long ky = (long long)round(py * 1e9);
                        ht_add(kx, ky);
                    }
                }
            }
        }
    }

    /* Process multiplicities */
    ll total_val = 0;
    for (int i = 0; i < ht_pool_idx; i++) {
        int pairs = ht_pool[i].count;
        /* m(m-1)/2 = pairs => m = (1+sqrt(1+8*pairs))/2 */
        int delta = 1 + 8 * pairs;
        int sq = (int)round(sqrt((double)delta));
        int m = (1 + sq) / 2;

        /* Contribution for one point of multiplicity m:
         * c = sum_{k=2}^m (-1)^k * (k-1) * C(m,k) * 2^(k-1) * (n-k-1)! */
        i128 c = 0;
        for (int k = 2; k <= m; k++) {
            i128 term = (k % 2 == 0) ? 1 : -1;
            term *= (k - 1);
            i128 bin_mk = fact_exact[m] / (fact_exact[k] * fact_exact[m - k]);
            i128 pow2 = 1;
            for (int p = 0; p < k - 1; p++) pow2 *= 2;
            i128 fact_rem = fact_exact[n - k - 1];
            c += term * bin_mk * pow2 * fact_rem;
        }

        ll term_total = (ll)((c % MOD + MOD) % MOD);
        total_val = (total_val + term_total) % MOD;
    }

    return total_val;
}

int main(void) {
    init_factorials();

    ll total_sum = 0;
    for (int n = 3; n <= 60; n++) {
        ll tn = compute_t(n);
        total_sum = (total_sum + tn) % MOD;
    }

    printf("%lld\n", total_sum);
    return 0;
}
