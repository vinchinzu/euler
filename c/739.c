/* Project Euler 739: Summation of Summations.
 * Lucas sequence with binomial coefficient iteration.
 */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

#define N 100000000
#define M 1000000007LL

typedef long long ll;
typedef __int128 lll;

ll mod_pow(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) result = (lll)result * base % mod;
        exp >>= 1;
        base = (lll)base * base % mod;
    }
    return result;
}

int main() {
    ll *mod_invs = malloc((N + 1) * sizeof(ll));
    mod_invs[1] = 1;
    for (int i = 2; i <= N; i++) {
        mod_invs[i] = M - (M / i) * mod_invs[M % i] % M;
    }

    ll *lucas = malloc((N + 1) * sizeof(ll));
    lucas[1] = 1;
    lucas[2] = 3;
    for (int i = 3; i <= N; i++) {
        lucas[i] = (lucas[i - 2] + lucas[i - 1]) % M;
    }

    ll nCr1 = 1, nCr2 = 1;
    ll ans = lucas[N];

    for (int k = N - 1; k >= 2; k--) {
        ll mult = (ll)(2 * N - 2 - k) * mod_invs[N - k] % M;
        nCr1 = (lll)nCr1 * mult % M;
        ans = (ans + (lll)lucas[k] * ((nCr1 - nCr2 + M) % M)) % M;
        nCr2 = (nCr2 + nCr1) % M;
    }

    printf("%lld\n", ans);

    free(mod_invs);
    free(lucas);
    return 0;
}
