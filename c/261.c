/*
 * Project Euler 261: Pivotal Square Sums
 *
 * Pell equation approach: for each m, solve x^2 - D*y^2 = m^2*(m+1)
 * with D = m*(m+1). Generate solutions via Brahmagupta-Pell composition.
 * Uses __int128 for large intermediate values.
 */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <math.h>
#include <string.h>

typedef __int128 i128;

static int is_square_64(int64_t n) {
    if (n < 0) return 0;
    int64_t r = (int64_t)sqrt((double)n);
    /* Adjust for floating point imprecision */
    while (r > 0 && r * r > n) r--;
    while ((r + 1) * (r + 1) <= n) r++;
    return r * r == n;
}

static int64_t isqrt64(int64_t n) {
    if (n <= 0) return 0;
    int64_t r = (int64_t)sqrt((double)n);
    while (r > 0 && r * r > n) r--;
    while ((r + 1) * (r + 1) <= n) r++;
    return r;
}

/* Factorize n, store prime factors and exponents */
#define MAX_FACTORS 20
typedef struct {
    int64_t primes[MAX_FACTORS];
    int exps[MAX_FACTORS];
    int count;
} Factorization;

static void factorize(int64_t n, Factorization *f) {
    f->count = 0;
    int64_t d = 2;
    while (d * d <= n) {
        if (n % d == 0) {
            f->primes[f->count] = d;
            f->exps[f->count] = 0;
            while (n % d == 0) {
                f->exps[f->count]++;
                n /= d;
            }
            f->count++;
        }
        d++;
    }
    if (n > 1) {
        f->primes[f->count] = n;
        f->exps[f->count] = 1;
        f->count++;
    }
}

static int64_t ipow(int64_t base, int exp) {
    int64_t result = 1;
    for (int i = 0; i < exp; i++) result *= base;
    return result;
}

/* Pivot set using sorted array + binary search for dedup */
#define MAX_PIVOTS 200000
static int64_t pivots[MAX_PIVOTS];
static int num_pivots = 0;

static int cmp64(const void *a, const void *b) {
    int64_t va = *(const int64_t *)a;
    int64_t vb = *(const int64_t *)b;
    if (va < vb) return -1;
    if (va > vb) return 1;
    return 0;
}

static void add_pivot(int64_t k) {
    if (num_pivots < MAX_PIVOTS)
        pivots[num_pivots++] = k;
}

int main(void) {
    int64_t N = 10000000000LL; /* 10^10 */
    int64_t L = isqrt64(N / 2);

    for (int64_t m = 1; m <= L; m++) {
        i128 D = (i128)m * (m + 1);

        /* Factorize m and m+1 */
        Factorization fm, fm1;
        factorize(m, &fm);
        factorize(m + 1, &fm1);

        /* Compute sD (squarefree part of D) and sqrt(D/sD) */
        /* Combined factorization of m*(m+1) */
        int64_t combined_p[40];
        int combined_e[40];
        int combined_count = 0;

        for (int i = 0; i < fm.count; i++) {
            combined_p[combined_count] = fm.primes[i];
            combined_e[combined_count] = fm.exps[i];
            combined_count++;
        }
        for (int i = 0; i < fm1.count; i++) {
            int found = 0;
            for (int j = 0; j < combined_count; j++) {
                if (combined_p[j] == fm1.primes[i]) {
                    combined_e[j] += fm1.exps[i];
                    found = 1;
                    break;
                }
            }
            if (!found) {
                combined_p[combined_count] = fm1.primes[i];
                combined_e[combined_count] = fm1.exps[i];
                combined_count++;
            }
        }

        /* sD = product of p^(e%2) */
        int64_t sD = 1;
        for (int i = 0; i < combined_count; i++)
            if (combined_e[i] % 2 == 1)
                sD *= combined_p[i];

        /* sqrt(D/sD) */
        int64_t sqrt_D_over_sD = 1;
        for (int i = 0; i < combined_count; i++)
            sqrt_D_over_sD *= ipow(combined_p[i], combined_e[i] / 2);

        /* sm = product of p^ceil(e/2) for factors of m */
        int64_t sm = 1;
        for (int i = 0; i < fm.count; i++)
            sm *= ipow(fm.primes[i], (fm.exps[i] + 1) / 2);

        /* Find base solutions: y from 0 to m, step sm */
        i128 base_x[200], base_y[200];
        int num_base = 0;

        for (int64_t y = 0; y <= m; y += sm) {
            int64_t res = m + y * y;
            if (res % sD != 0) continue;
            int64_t quotient = res / sD;
            if (!is_square_64(quotient)) continue;
            i128 x = (i128)sD * sqrt_D_over_sD * isqrt64(quotient);
            if (num_base < 200) {
                base_x[num_base] = x;
                base_y[num_base] = y;
                num_base++;
            }
        }

        /* Fundamental solution: (2m+1, 2) */
        i128 xf = 2 * m + 1;
        i128 yf = 2;

        for (int b = 0; b < num_base; b++) {
            /* Direct chain */
            i128 x = base_x[b], y = base_y[b];
            while (1) {
                if (y + m > 2 * N) break;
                /* Check and add */
                if (x % m == 0) {
                    i128 val = x / m - m - 1;
                    if (val >= 0 && val % 2 == 0) {
                        i128 ym = y + m;
                        if (ym >= 0 && ym % 2 == 0) {
                            i128 n_val = val / 2;
                            i128 k = ym / 2;
                            if (n_val >= k && k > 0 && k <= N) {
                                add_pivot((int64_t)k);
                            }
                        }
                    }
                }
                i128 nx = xf * x + D * yf * y;
                i128 ny = xf * y + yf * x;
                x = nx; y = ny;
            }

            /* Conjugate chain */
            if (base_y[b] > 0) {
                x = xf * base_x[b] - D * yf * base_y[b];
                y = yf * base_x[b] - xf * base_y[b];
                while (y < 0) {
                    i128 nx = xf * x + D * yf * y;
                    i128 ny = xf * y + yf * x;
                    x = nx; y = ny;
                }
                while (1) {
                    if (y + m > 2 * N) break;
                    if (x % m == 0) {
                        i128 val = x / m - m - 1;
                        if (val >= 0 && val % 2 == 0) {
                            i128 ym = y + m;
                            if (ym >= 0 && ym % 2 == 0) {
                                i128 n_val = val / 2;
                                i128 k = ym / 2;
                                if (n_val >= k && k > 0 && k <= N) {
                                    add_pivot((int64_t)k);
                                }
                            }
                        }
                    }
                    i128 nx = xf * x + D * yf * y;
                    i128 ny = xf * y + yf * x;
                    x = nx; y = ny;
                }
            }
        }
    }

    /* Deduplicate and sum */
    qsort(pivots, num_pivots, sizeof(int64_t), cmp64);
    int64_t ans = 0;
    for (int i = 0; i < num_pivots; i++) {
        if (i == 0 || pivots[i] != pivots[i-1])
            ans += pivots[i];
    }
    printf("%lld\n", (long long)ans);
    return 0;
}
