/* Project Euler Problem 741: Binary Grid Colourings.
 * Extracted from embedded C in python/741.py
 */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

typedef long long ll;
typedef __int128 lll;

#define M 1000000007LL
#define N1 823543    /* 7^7 */
#define N2 16777216  /* 8^8 */

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

ll mod_inv(ll a, ll mod) {
    return mod_pow(a, mod - 2, mod);
}

ll f(int n) {
    ll *f_arr = calloc(n + 1, sizeof(ll));
    ll *fp = calloc(n + 1, sizeof(ll));
    f_arr[0] = 1;
    for (int k = 2; k <= n; k++) {
        fp[k] = (lll)k * (k - 1) % M * (((lll)(k - 1) * f_arr[k - 2] % M + fp[k - 1]) % M) % M;
        f_arr[k] = (lll)((M + 1) / 2) * fp[k] % M;
    }
    ll result = f_arr[n];
    free(f_arr);
    free(fp);
    return result;
}

ll rotate90(int n) {
    if (n % 2 == 1) return 0;
    ll *f_arr = calloc(n + 1, sizeof(ll));
    ll *fp = calloc(n + 1, sizeof(ll));
    f_arr[0] = f_arr[2] = 1;
    for (int k = 4; k <= n; k += 2) {
        fp[k] = (lll)(k - 2) * ((f_arr[k - 4] + fp[k - 2]) % M) % M;
        f_arr[k] = (f_arr[k - 2] + (lll)(k / 2 - 1) * f_arr[k - 4] % M + (lll)(k - 2) * fp[k - 2] % M) % M;
    }
    ll result = f_arr[n];
    free(f_arr);
    free(fp);
    return result;
}

ll rotate180(int n) {
    ll *f_arr = calloc(n + 1, sizeof(ll));
    ll *fp = calloc(n + 1, sizeof(ll));
    f_arr[0] = f_arr[2] = 1;
    fp[2] = 2;
    for (int k = 3; k <= n; k++) {
        if (k % 2 == 0) {
            fp[k] = (lll)k * ((((lll)(k - 2) * f_arr[k - 4] % M + fp[k - 2]) * (k - 2) % M + f_arr[k - 2]) % M) % M;
            f_arr[k] = (lll)((M + 1) / 2) * fp[k] % M;
        } else {
            fp[k] = (lll)(k - 1) * ((f_arr[k - 3] + (lll)(k - 3) * fp[k - 2] % M) % M) % M;
            f_arr[k] = (lll)(k / 2) * fp[k] % M;
        }
    }
    ll result = f_arr[n];
    free(f_arr);
    free(fp);
    return result;
}

ll flipY(int n) {
    if (n % 2 == 1) return 0;
    ll fact = 1;
    for (int i = 1; i <= n; i++) {
        fact = (lll)fact * i % M;
    }
    return (lll)fact * mod_inv(mod_pow(2, n / 2, M), M) % M;
}

ll flipDiagonal(int n) {
    ll *f_arr = calloc(n + 1, sizeof(ll));
    ll *fp = calloc(n + 1, sizeof(ll));
    ll *fpp = calloc(n + 1, sizeof(ll));
    f_arr[0] = fp[1] = 1;
    for (int k = 2; k <= n; k++) {
        fp[k] = (f_arr[k - 1] + (lll)(k - 1) * fp[k - 1] % M) % M;
        fpp[k] = (f_arr[k - 2] + fp[k - 1] + (lll)(k - 2) * fpp[k - 1] % M) % M;
        ll nCr = (lll)(k - 1) * (k - 2) / 2 % M;
        f_arr[k] = ((lll)(k - 1) * fp[k - 1] % M + (lll)nCr * fpp[k - 1] % M) % M;
    }
    ll result = f_arr[n];
    free(f_arr);
    free(fp);
    free(fpp);
    return result;
}

ll g(int n) {
    ll val = (f(n) + 2 * rotate90(n) + rotate180(n) + 2 * flipY(n) + 2 * flipDiagonal(n)) % M;
    return (lll)val * mod_inv(8, M) % M;
}

int main() {
    ll ans = (g(N1) + g(N2)) % M;
    printf("%lld\n", ans);
    return 0;
}
