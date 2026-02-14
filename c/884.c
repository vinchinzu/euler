#include <stdio.h>
#include <math.h>

typedef long long ll;
typedef __int128 i128;

ll integer_cbrt(ll n) {
    if (n <= 0) return 0;
    ll x = (ll)cbrt((double)n);
    /* Refine */
    while ((x + 1) * (x + 1) * (x + 1) <= n) x++;
    while (x * x * x > n) x--;
    return x;
}

/* K_max can be up to cbrt(10^17 - 1) ~ 464158 */
#define MAX_K 470000

ll prefix_T[MAX_K + 1];

ll recursive_S(ll n) {
    if (n <= 1) return 0;
    ll k = integer_cbrt(n - 1);

    /* Contribution from full intervals 1..k-1 */
    ll full_intervals_sum = prefix_T[k - 1];

    /* Contribution from partial interval [k^3, n-1] */
    ll L = n - k * k * k;
    ll partial_sum = L + recursive_S(L);

    return full_intervals_sum + partial_sum;
}

int main(void) {
    ll N = 100000000000000000LL; /* 10^17 */

    if (N <= 1) {
        printf("0\n");
        return 0;
    }

    ll K_max = integer_cbrt(N - 1);

    prefix_T[0] = 0;

    for (ll k = 1; k <= K_max; k++) {
        ll L_k = 3 * k * k + 3 * k + 1;
        ll val = recursive_S(L_k);
        ll term = L_k + val;
        prefix_T[k] = prefix_T[k - 1] + term;
    }

    printf("%lld\n", recursive_S(N));
    return 0;
}
