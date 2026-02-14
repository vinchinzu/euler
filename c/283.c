/*
 * Project Euler 283 - Integer Sided Triangles for all integer area/perimeter ratios
 *
 * Extracted from embedded C in the Python solution.
 */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

#define N 1000
static int spf[16000001];
static long long divisors[100000];

void build_spf(int limit) {
    for (int i = 0; i <= limit; i++) spf[i] = i;
    for (int i = 2; (long long)i * i <= limit; i++) {
        if (spf[i] == i) {
            for (int j = i * i; j <= limit; j += i) {
                if (spf[j] == j) spf[j] = i;
            }
        }
    }
}

static inline int sq_le(long long val, long long product) {
    return (__int128)val * val <= (__int128)product;
}

static long long smallest_factor_large(long long n) {
    if (n % 2 == 0) return 2;
    for (long long i = 3; i * i <= n; i += 2) {
        if (n % i == 0) return i;
    }
    return n;
}

int main() {
    int L = (int)(2.0 * sqrt(3.0) * N);
    int max_val = 4 * N * N + L * L;
    build_spf(max_val);

    long long ans = 0;
    divisors[0] = 1;

    for (int r = 1; r <= N; r++) {
        long long r2_4 = 4LL * r * r;

        int factors[20];
        int nfactors = 0;
        {
            int tmp = (int)r2_4;
            while (tmp > 1) {
                int f = spf[tmp];
                factors[nfactors++] = f;
                while (tmp % f == 0) tmp /= f;
            }
        }

        for (int x = 1; x <= L; x++) {
            long long product = r2_4 * (r2_4 + (long long)x * x);
            long long k = product;
            int divisors_size = 1;

            for (int fi = 0; fi < nfactors; fi++) {
                int d = factors[fi];
                int e = 0;
                while (k % d == 0) { k /= d; e++; }
                int old_size = divisors_size;
                for (int i = old_size - 1; i >= 0; i--) {
                    long long mult = d;
                    for (int j = 0; j < e; j++) {
                        long long val = divisors[i] * mult;
                        if (sq_le(val, product))
                            divisors[divisors_size++] = val;
                        mult *= d;
                    }
                }
            }

            while (k > 1) {
                long long d;
                if (k <= max_val)
                    d = spf[(int)k];
                else
                    d = smallest_factor_large(k);
                int e = 0;
                while (k % d == 0) { k /= d; e++; }
                int old_size = divisors_size;
                for (int i = old_size - 1; i >= 0; i--) {
                    long long mult = d;
                    for (int j = 0; j < e; j++) {
                        long long val = divisors[i] * mult;
                        if (sq_le(val, product))
                            divisors[divisors_size++] = val;
                        mult *= d;
                    }
                }
            }

            for (int i = 0; i < divisors_size; i++) {
                long long xy = divisors[i] + r2_4;
                long long xz = product / divisors[i] + r2_4;
                if (xy % x == 0 && xz % x == 0 && (long long)x * x <= xy)
                    ans += 2 * (x + xy / x + xz / x);
            }
        }
    }

    printf("%lld\n", ans);
    return 0;
}
