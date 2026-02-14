/*
 * Project Euler Problem 710: One Million Members.
 *
 * t(n) = 2*t(n-2) - t(n-4) + t(n-6) + 2^(n/2 - 3)
 * Find smallest N > 42 such that t(N) % 1000000 == 0.
 */
#include <stdio.h>

typedef long long ll;

ll pow_mod(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) result = result * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return result;
}

int main() {
    ll m = 1000000;

    /* ts[0..5] = {0, 0, 1, 0, 2, 1} corresponding to t(0)..t(5) */
    /* Use circular buffer */
    ll ts[7];
    ts[0] = 0; ts[1] = 0; ts[2] = 1; ts[3] = 0; ts[4] = 2; ts[5] = 1;

    int n = 6;
    while (1) {
        ll term = (2 * ts[(n - 2) % 7]
                   - ts[(n - 4) % 7]
                   + ts[(n - 6) % 7]
                   + pow_mod(2, n / 2 - 3, m)) % m;
        if (term < 0) term += m;
        ts[n % 7] = term;
        if (term == 0) {
            printf("%d\n", n);
            return 0;
        }
        n++;
    }
}
