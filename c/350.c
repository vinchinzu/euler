/* Project Euler 350 - Constrained sequences
 *
 * Find f(G, L, N) mod 101^4, where G=10^6, L=10^12, N=10^18.
 * f counts lists of size N with gcd >= G and lcm <= L.
 */

#include <stdio.h>

typedef long long ll;
typedef __int128 i128;

#define G 1000000LL
#define L 1000000000000LL
#define NN 1000000000000000000LL
#define M 104060401LL  /* 101^4 */

static int spf[1000001];

ll mod_pow(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) result = (i128)result * base % mod;
        base = (i128)base * base % mod;
        exp >>= 1;
    }
    return result;
}

int main(void) {
    ll R = L / G; /* 10^6 */

    /* Sieve smallest prime factor */
    for (int i = 0; i <= R; i++) spf[i] = i;
    for (int i = 2; (ll)i * i <= R; i++) {
        if (spf[i] == i) {
            for (int j = i * i; j <= R; j += i)
                if (spf[j] == j) spf[j] = i;
        }
    }

    /* Precompute powers of e mod M for e up to ~20 */
    int max_e = 22;
    ll powN[24];
    for (int e = 0; e <= max_e + 1; e++)
        powN[e] = mod_pow(e, NN, M);

    ll ans = 0;
    for (int r = 1; r <= R; r++) {
        /* Factor r using spf */
        int temp = r;
        ll res = 1;
        while (temp > 1) {
            int p = spf[temp];
            int e = 0;
            while (temp % p == 0) { temp /= p; e++; }
            ll factor = (powN[e + 1] - 2 * powN[e] + (e > 0 ? powN[e - 1] : 0)) % M;
            if (factor < 0) factor += M;
            res = (i128)res * factor % M;
        }

        ll count = (L / r - G + 1) % M;
        if (count < 0) count += M;
        ans = (ans + (i128)count * res) % M;
    }

    printf("%lld\n", ans % M);
    return 0;
}
