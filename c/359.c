/* Project Euler 359 - Hilbert's New Hotel
 *
 * P(f, r) follows closed-form formulas by parity of f and r.
 * Sum P(f, r) for all divisor pairs (f, r) of 71328803586048, mod 10^8.
 */

#include <stdio.h>

typedef long long ll;
typedef __int128 i128;

#define TARGET 71328803586048LL
#define MOD 100000000LL

/* Compute P(f,r) mod MOD */
ll P(ll f, ll r) {
    /* All formulas produce integer results (even before /2).
     * We compute mod 2*MOD first, then divide by 2 to get mod MOD. */
    ll M2 = 2 * MOD; /* 200000000 */

    if (f == 1) {
        /* r*(r+1)/2 */
        ll rm = r % M2;
        ll rp1m = (r + 1) % M2;
        ll prod = (i128)rm * rp1m % M2;
        return (prod / 2) % MOD;
    }

    ll fm = f % M2;
    ll rm = r % M2;
    ll fpr = (fm + rm) % M2;
    ll sq = (i128)fpr * fpr % M2;

    if (f % 2 == 0) {
        if (r % 2 == 1) {
            /* ((f+r)^2 - 2f - r) / 2 */
            ll val = (sq - 2 * fm - rm + 4 * M2) % M2;
            return (val / 2) % MOD;
        } else {
            /* ((f+r)^2 - r) / 2 */
            ll val = (sq - rm + M2) % M2;
            return (val / 2) % MOD;
        }
    } else {
        if (r % 2 == 1) {
            /* ((f+r)^2 - 2f - 3r + 1) / 2 */
            ll val = (sq - 2 * fm - 3 * rm + 1 + 6 * M2) % M2;
            return (val / 2) % MOD;
        } else {
            /* ((f+r)^2 - 4f - 3r + 3) / 2 */
            ll val = (sq - 4 * fm - 3 * rm + 3 + 8 * M2) % M2;
            return (val / 2) % MOD;
        }
    }
}

int main(void) {
    ll n = TARGET;

    ll total = 0;
    for (ll d = 1; d * d <= n; d++) {
        if (n % d == 0) {
            ll f = d, r = n / d;
            total = (total + P(f, r)) % MOD;
            if (f != r) {
                total = (total + P(r, f)) % MOD;
            }
        }
    }

    printf("%lld\n", total);
    return 0;
}
