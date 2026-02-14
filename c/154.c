/*
 * Project Euler 154 - Exploring Pascal's pyramid
 *
 * Count coefficients in (x+y+z)^200000 divisible by 10^12.
 * multinomial(N; a,b,c) = N!/(a!b!c!). Divisible by 10^12 means
 * v5(N!) - v5(a!) - v5(b!) - v5(c!) >= 12 AND same for v2.
 */
#include <stdio.h>
#include <stdlib.h>

#define N_VAL 200000
#define K 12

int main(void) {
    /* Precompute v5[i] = number of times 5 divides i */
    /* F[i] = v5(i!) = cumulative sum of v5[j] for j=1..i */
    /* T[i] = v2(i!) = cumulative sum of v2[j] for j=1..i */
    int *F = calloc(N_VAL + 1, sizeof(int));
    int *T = calloc(N_VAL + 1, sizeof(int));

    for (int i = 1; i <= N_VAL; i++) {
        int v5 = 0, v2 = 0;
        int n = i;
        while (n % 5 == 0) { v5++; n /= 5; }
        n = i;
        while (n % 2 == 0) { v2++; n /= 2; }
        F[i] = F[i-1] + v5;
        T[i] = T[i-1] + v2;
    }

    int FN = F[N_VAL];
    int TN = T[N_VAL];

    long long ans = 0;

    for (int a = 0; a <= N_VAL / 3; a++) {
        if (3 * a > N_VAL) break;

        int tempF = F[a] + K - FN; /* This is F[a] - F[N] + K; if F[b] + F[c] + tempF <= 0, divisible by 5^K */
        int tempT = T[a] + K - TN;

        int b_lo = a + 1;
        int b_hi = (N_VAL - a - 1) / 2;
        if (b_lo > b_hi) continue;

        /* For each b in [b_lo, b_hi], c = N - a - b, check:
         * F[b] + F[c] + tempF <= 0  and  T[b] + T[c] + tempT <= 0
         * where c = N - a - b, so F[c] = F[N-a-b]
         */
        for (int b = b_lo; b <= b_hi; b++) {
            int c = N_VAL - a - b;
            int d5 = F[b] + F[c] + tempF;
            int d2 = T[b] + T[c] + tempT;
            if (d5 <= 0 && d2 <= 0) {
                ans += 6;
            }
        }

        /* Case a == b: c = N - 2a */
        {
            int c = N_VAL - 2 * a;
            if (c > a) {
                int d5 = F[a] + F[c] + tempF;
                int d2 = T[a] + T[c] + tempT;
                if (d5 <= 0 && d2 <= 0) {
                    ans += 3;
                }
            }
        }

        /* Case b == c: b = c = (N-a)/2 */
        if ((N_VAL - a) % 2 == 0) {
            int half = (N_VAL - a) / 2;
            if (half > a) {
                int d5 = 2 * F[half] + tempF;
                int d2 = 2 * T[half] + tempT;
                if (d5 <= 0 && d2 <= 0) {
                    ans += 3;
                }
            }
        }
    }

    free(F);
    free(T);

    printf("%lld\n", ans);
    return 0;
}
