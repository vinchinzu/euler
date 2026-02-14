#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* num_divisors sieve up to limit */
static int *num_divs;

static long long gcd(long long a, long long b) {
    while (b) { long long t = b; b = a % b; a = t; }
    return a;
}

int main() {
    long long N = 10000000LL;
    long long limit = 2 * N;

    num_divs = (int *)calloc(limit + 1, sizeof(int));
    if (!num_divs) { fprintf(stderr, "alloc fail\n"); return 1; }

    /* Sieve divisor counts */
    for (long long i = 1; i <= limit; i++) {
        for (long long j = i; j <= limit; j += i) {
            num_divs[j]++;
        }
    }

    long long ans = 0;

    /* isqrt(N) */
    long long sqN = 1;
    while ((sqN + 1) * (sqN + 1) <= N) sqN++;

    /* isqrt(2*N) */
    long long sq2N = 1;
    while ((sq2N + 1) * (sq2N + 1) <= 2 * N) sq2N++;

    for (long long T1 = 1; T1 <= sqN; T1++) {
        /* T1^2 + T2^2 < 2*N  (strict: trace < N) */
        long long T2_max_sq = 2 * N - 1 - T1 * T1;
        if (T2_max_sq < 0) continue;
        long long T2_max = 1;
        while ((T2_max + 1) * (T2_max + 1) <= T2_max_sq) T2_max++;

        for (long long T2 = T1 + 2; T2 <= T2_max; T2 += 2) {
            long long g = gcd(T1, T2);
            long long r_start = g % 2;
            /* d >= 1 requires r*T2/g <= T1-2, i.e., r <= (T1-2)*g/T2 */
            long long r_max = (T1 - 2) * g / T2;

            for (long long r = r_start; r <= r_max; r += 2) {
                long long val = (g * g - r * r) / 4;
                if (val > 0 && val <= limit) {
                    if (r == 0)
                        ans += num_divs[val];
                    else
                        ans += 2 * num_divs[val];
                }
            }
        }
    }

    printf("%lld\n", ans);
    free(num_divs);
    return 0;
}
