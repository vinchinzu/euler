/*
 * Project Euler 634: Numbers of the form a^2 * b^3
 *
 * Count integers x <= N = 9*10^18 expressible as x = a^2 * b^3 with a,b > 1.
 *
 * Non-square case: for each squarefree b >= 2 with b^3 <= N, count sqrt(N/b^3) - 1.
 * Perfect square correction: inclusion-exclusion using Mobius function.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef long long ll;
typedef unsigned long long ull;
typedef __int128 lll;

/* isqrt for large numbers */
ll isqrt_ll(ll n) {
    if (n <= 0) return 0;
    ll x = (ll)sqrt((double)n);
    while (x * x > n) x--;
    while ((x+1) * (x+1) <= n) x++;
    return x;
}

/* Cube root */
ll icbrt(ll n) {
    if (n <= 0) return 0;
    ll x = (ll)cbrt((double)n);
    while (x > 0 && x * x * x > n) x--;
    while ((x+1) * (x+1) * (x+1) <= n) x++;
    return x;
}

int main() {
    ll N = 9000000000000000000LL; /* 9 * 10^18 */
    ll L = icbrt(N); /* ~2.08 * 10^6 */

    /* Sieve Mobius function up to L */
    int lim = (int)L + 1;
    int *mobius = (int *)malloc((lim + 1) * sizeof(int));
    char *is_prime_arr = (char *)calloc(lim + 1, 1);
    for (int i = 0; i <= lim; i++) mobius[i] = 1;
    for (int i = 2; i <= lim; i++) is_prime_arr[i] = 1;

    for (int i = 2; i <= lim; i++) {
        if (is_prime_arr[i]) {
            for (int j = i; j <= lim; j += i) {
                if (j != i) is_prime_arr[j] = 0;
                mobius[j] *= -1;
            }
            for (ll j = (ll)i * i; j <= lim; j += (ll)i * i) {
                mobius[j] = 0;
            }
        }
    }

    ll ans = 0;

    /* Count non-square numbers */
    for (ll b = 2; b * b * b <= N; b++) {
        if (mobius[(int)b] != 0) {
            ll q = N / (b * b * b);
            ans += isqrt_ll(q) - 1;
        }
    }

    /* Subtract perfect squares that are also a^2*b^3 */
    ll sqrt_N = isqrt_ll(N);
    /* i^6 <= N => i <= N^{1/6} */
    ll i_max = icbrt(sqrt_N); /* N^{1/6} approx */
    /* Actually need i^6 <= N, so i <= N^(1/6) */
    while ((i_max + 1) * (i_max + 1) * (i_max + 1) <= sqrt_N) i_max++;
    while (i_max > 0 && i_max * i_max * i_max > sqrt_N) i_max--;

    for (ll i = 2; i <= i_max; i++) {
        ll i3 = i * i * i;
        ans -= mobius[(int)i] * (sqrt_N / i3);
        if (is_prime_arr[(int)i]) {
            ans -= 1;
        }
    }

    printf("%lld\n", ans);

    free(mobius);
    free(is_prime_arr);
    return 0;
}
