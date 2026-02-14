/* Project Euler 354 - Honeycomb distance distribution — standalone C.
 * Extracted from embedded C in Python solution.
 */

#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <stdint.h>

#define N_VAL 500000000000LL
#define K_VAL 450

static double L1;
static int L2;
static int *spf;
static int *num_2mod3s;
static long long ans = 0;

int is_prime_large(long long n) {
    if (n < 2) return 0;
    if (n == 2 || n == 3) return 1;
    if (n % 2 == 0 || n % 3 == 0) return 0;
    for (long long i = 5; i * i <= n; i += 6) {
        if (n % i == 0 || n % (i + 2) == 0) return 0;
    }
    return 1;
}

int is_prime(long long p) {
    if (p <= 1) return 0;
    if (p <= (long long)L2) return spf[p] == p;
    return is_prime_large(p);
}

void find_nums_for_template(int index, long long prod_primes, long long min_prime,
                            double limit, int *tmpl, int tmpl_len) {
    if (index == tmpl_len) {
        double remaining = limit;
        while (remaining > 1.0) {
            int idx = (int)sqrt(remaining);
            if (idx > L2) idx = L2;
            ans += num_2mod3s[idx];
            remaining /= 3.0;
        }
        return;
    }

    int e = tmpl[index];
    long long p = (min_prime > 1) ? min_prime : 1;
    /* advance to p = 1 mod 3 */
    if (p % 3 == 0) p += 1;
    else if (p % 3 == 2) p += 2;

    while (1) {
        double pe = 1.0;
        int ok = 1;
        for (int j = 0; j < e; j++) {
            pe *= (double)p;
            if (pe > limit) { ok = 0; break; }
        }
        if (!ok) break;

        if (prod_primes % p != 0 && is_prime(p)) {
            long long next_min;
            if (index + 1 < tmpl_len && tmpl[index] == tmpl[index + 1])
                next_min = p + 3;
            else
                next_min = 1;

            find_nums_for_template(index + 1, prod_primes * p, next_min,
                                   limit / pe, tmpl, tmpl_len);
        }
        p += 3;
    }
}

void find_all_templates(int n, int max_d, int *tmpl, int tmpl_len) {
    if (n == 1) {
        find_nums_for_template(0, 1, 1, L1, tmpl, tmpl_len);
        return;
    }
    for (int d = 2; d <= max_d; d++) {
        if (n % d == 0) {
            tmpl[tmpl_len] = d - 1;
            find_all_templates(n / d, d, tmpl, tmpl_len + 1);
        }
    }
}

int main() {
    L1 = (double)N_VAL / sqrt(3.0);
    L1 = L1 * L1;
    L2 = (int)sqrt(L1 / (pow(7.0, 4) * pow(13.0, 4)));

    spf = (int*)malloc((L2 + 1) * sizeof(int));
    for (int i = 0; i <= L2; i++) spf[i] = i;
    for (int i = 2; (long long)i * i <= L2; i++) {
        if (spf[i] == i) {
            for (int j = i * i; j <= L2; j += i)
                if (spf[j] == j) spf[j] = i;
        }
    }

    num_2mod3s = (int*)calloc(L2 + 1, sizeof(int));
    for (int n = 1; n <= L2; n++) {
        int ok = 1, temp = n;
        while (temp > 1) {
            int p = spf[temp];
            if (p % 3 != 2) { ok = 0; break; }
            while (temp % p == 0) temp /= p;
        }
        num_2mod3s[n] = num_2mod3s[n - 1] + ok;
    }

    int tmpl[32];
    find_all_templates(K_VAL / 6, K_VAL / 6, tmpl, 0);

    printf("%lld\n", ans);
    free(spf);
    free(num_2mod3s);
    return 0;
}
