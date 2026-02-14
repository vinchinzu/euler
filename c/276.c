/*
 * Project Euler 276: Primitive Triangles
 *
 * Count primitive integer triangles with perimeter <= 10^7.
 * T(k) = number of triangles with perimeter exactly k.
 * Sum mobius(d) * cumulative_T(N/d) for d = 1..N.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define LIMIT 10000000

static int *mobius;
static long long *sum_tri;

int main(void) {
    int N = LIMIT;

    /* Sieve Mobius function */
    mobius = (int *)calloc(N + 1, sizeof(int));
    for (int i = 0; i <= N; i++) mobius[i] = 1;

    char *is_prime = (char *)calloc(N + 1, 1);
    memset(is_prime, 1, N + 1);
    is_prime[0] = is_prime[1] = 0;

    for (int i = 2; i <= N; i++) {
        if (is_prime[i]) {
            for (int j = i; j <= N; j += i) {
                is_prime[j] = 0;
                mobius[j] *= -1;
            }
            for (long long j = (long long)i * i; j <= N; j += (long long)i * i) {
                mobius[j] = 0;
            }
        }
    }
    /* Fix: is_prime was corrupted, re-mark i as prime */
    /* Actually the sieve above marks is_prime[i]=0 for i itself when j=i.
       We need a separate approach. */
    free(is_prime);

    /* Redo Mobius properly */
    /* Use a proper linear sieve for Mobius */
    for (int i = 0; i <= N; i++) mobius[i] = 0;
    mobius[1] = 1;

    /* Smallest prime factor sieve + Mobius */
    int *smallest_pf = (int *)calloc(N + 1, sizeof(int));
    for (int i = 2; i <= N; i++) {
        if (smallest_pf[i] == 0) {
            /* i is prime */
            for (int j = i; j <= N; j += i) {
                if (smallest_pf[j] == 0) smallest_pf[j] = i;
            }
        }
    }

    /* Compute Mobius using SPF */
    for (int i = 2; i <= N; i++) {
        int p = smallest_pf[i];
        int prev = i / p;
        if (prev % p == 0) {
            mobius[i] = 0;  /* p^2 divides i */
        } else {
            mobius[i] = -mobius[prev];
        }
    }
    free(smallest_pf);

    /* Compute T(k): number of triangles with perimeter exactly k */
    /* T(k) = round(k^2/12) for even k, round((k+3)^2/12)/... */
    /* Actually from the Python code:
       if k even: (k*k + 24) / 48
       if k odd:  ((k+3)^2 + 24) / 48  (integer division)
    */
    /* Cumulative sum */
    sum_tri = (long long *)calloc(N + 1, sizeof(long long));
    for (int k = 1; k <= N; k++) {
        long long t;
        if (k % 2 == 0) {
            t = ((long long)k * k + 24) / 48;
        } else {
            long long kp3 = k + 3;
            t = (kp3 * kp3 + 24) / 48;
        }
        sum_tri[k] = sum_tri[k-1] + t;
    }

    long long ans = 0;
    for (int d = 1; d <= N; d++) {
        if (mobius[d] != 0) {
            ans += (long long)mobius[d] * sum_tri[N / d];
        }
    }

    printf("%lld\n", ans);

    free(mobius);
    free(sum_tri);
    return 0;
}
