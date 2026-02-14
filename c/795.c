/*
 * Project Euler 795 - Alternating GCD Sum
 *
 * g(n) = sum_{i=1}^n (-1)^i * GCD(n, i^2)
 *
 * For odd n: g(n) = -n (cancellation argument)
 * For even n: g(n) is multiplicative. For prime power p^e:
 *   g(p^e) = sum over k of count(k) * sign * p^min(2k,e)
 *   where count(k) = p^(e-k) - p^(e-k-1), sign = -1 if p==2 && k==0 else 1
 *
 * Compute sum_{n=1}^N g(n) using smallest-prime-factor sieve.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define N 12345678

static int ff[N + 1];
static long long f[N + 1];

int main(void) {
    /* Sieve smallest prime factor */
    memset(ff, 0, sizeof(ff));
    for (int i = 2; i <= N; i++) {
        if (ff[i] == 0) {
            ff[i] = i;
            if ((long long)i * i <= N) {
                for (int j = i * i; j <= N; j += i) {
                    if (ff[j] == 0) ff[j] = i;
                }
            }
        }
    }

    /* Compute multiplicative function f[n] for even n */
    f[1] = 1;
    for (int n = 2; n <= N; n++) {
        int nn = n;
        int p = ff[nn];
        int e = 0;
        while (nn % p == 0) {
            nn /= p;
            e++;
        }

        if (nn > 1) {
            f[n] = f[nn] * f[n / nn];
        } else {
            /* n = p^e, compute f(p^e) */
            long long val = 0;
            /* Precompute powers of p */
            long long pe_k = 1;  /* p^0 initially, we'll build p^(e-k) differently */
            /* We need p^(e-k) for k=0..e */
            /* pe_arr[k] = p^(e-k) */
            long long pe_arr[40];
            pe_arr[0] = 1;
            for (int i = 1; i <= e; i++) pe_arr[i] = pe_arr[i-1] * p;
            /* pe_arr[i] = p^i, so p^(e-k) = pe_arr[e-k] */

            for (int k = 0; k <= e; k++) {
                long long count = pe_arr[e - k] - (e - k > 0 ? pe_arr[e - k - 1] : 0);
                int sign = (p == 2 && k == 0) ? -1 : 1;
                long long power = (2 * k <= e) ? pe_arr[2 * k] : pe_arr[e];
                val += count * sign * power;
            }
            f[n] = val;
        }
    }

    long long ans = 0;
    for (int n = 1; n <= N; n++) {
        if (n % 2 == 0) {
            ans += f[n];
        } else {
            ans -= n;
        }
    }

    printf("%lld\n", ans);
    return 0;
}
