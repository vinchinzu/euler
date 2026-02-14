/*
 * Project Euler 677 - Coloured Graphs
 *
 * Count colored trees with degree/color constraints.
 * DP + Burnside approach. Extracted from embedded C in Python solution.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef __int128 lll;

#define MOD_VAL 1000000007LL
static const ll M = MOD_VAL;
#define N 10000
#define K 4

ll mod_inv(ll a) {
    ll result = 1;
    ll exp = M - 2;
    a %= M; if (a < 0) a += M;
    while (exp > 0) {
        if (exp & 1) result = (lll)result * a % M;
        a = (lll)a * a % M;
        exp >>= 1;
    }
    return result;
}

static ll f[K+1][2][N+1];
static ll g[K+1][2][N+1];
static ll h_r[N+1], h_b[N+1], h_y[N+1];
static ll H[2][N+1];

int main() {
    ll fact[K+1];
    fact[0] = 1;
    for (int i = 1; i <= K; i++) fact[i] = fact[i-1] * i % M;

    ll inv_fact[K+1];
    for (int i = 0; i <= K; i++) inv_fact[i] = mod_inv(fact[i]);

    ll nCr_table[K+1][K+1];
    memset(nCr_table, 0, sizeof(nCr_table));
    for (int nn = 0; nn <= K; nn++) {
        nCr_table[nn][0] = 1;
        for (int rr = 1; rr <= nn; rr++)
            nCr_table[nn][rr] = (nCr_table[nn-1][rr-1] + nCr_table[nn-1][rr]) % M;
    }

    memset(f, 0, sizeof(f));
    memset(g, 0, sizeof(g));
    memset(h_r, 0, sizeof(h_r));
    memset(h_b, 0, sizeof(h_b));
    memset(h_y, 0, sizeof(h_y));
    memset(H, 0, sizeof(H));

    for (int size = 0; size <= N; size++) {
        for (int yellow_root = 0; yellow_root < 2; yellow_root++) {
            for (int nc = 0; nc <= K; nc++) {
                if (nc == K && size != N) continue;

                if (nc == 0) {
                    f[nc][yellow_root][size] = (size == 1) ? 1 : 0;
                } else {
                    ll count = 0;
                    for (int cs = 1; cs < size; cs++) {
                        if (2 * cs <= N) {
                            count = (count + (lll)H[yellow_root][cs] * f[nc-1][yellow_root][size - cs]) % M;
                        }
                    }
                    f[nc][yellow_root][size] = count;
                }

                if (size > N/2 && size != N) continue;

                ll count = f[nc][yellow_root][size];
                for (int kk = 2; kk <= nc; kk++) {
                    ll multiplier = (lll)fact[kk-1] * nCr_table[nc][kk] % M;
                    for (int cs = 1; cs < size; cs++) {
                        if ((ll)kk * cs < size && 2 * cs <= N) {
                            count = (count + (lll)multiplier % M
                                    * ((lll)H[yellow_root][cs] % M
                                    * f[nc - kk][yellow_root][size - kk * cs] % M)) % M;
                        }
                    }
                }
                g[nc][yellow_root][size] = (lll)count * inv_fact[nc] % M;
            }
        }

        h_r[size] = 0;
        for (int nc = 0; nc < K; nc++)
            h_r[size] = (h_r[size] + g[nc][0][size]) % M;

        h_b[size] = 0;
        for (int nc = 0; nc < K-1; nc++)
            h_b[size] = (h_b[size] + g[nc][0][size]) % M;

        h_y[size] = 0;
        for (int nc = 0; nc < K-1; nc++)
            h_y[size] = (h_y[size] + g[nc][1][size]) % M;

        H[0][size] = (h_r[size] + h_b[size] + h_y[size]) % M;
        H[1][size] = (h_r[size] + h_b[size]) % M;
    }

    ll ans = 0;
    for (int nc = 0; nc <= K; nc++)
        ans = (ans + g[nc][0][N]) % M;
    for (int nc = 0; nc <= K-1; nc++)
        ans = (ans + g[nc][0][N]) % M;
    for (int nc = 0; nc <= K-1; nc++)
        ans = (ans + g[nc][1][N]) % M;

    if (N % 2 == 0) {
        ans = ans * 2 % M;
        ll hr = h_r[N/2], hb = h_b[N/2], hy = h_y[N/2];
        ll h_arr[3] = {hr, hb, hy};
        for (int i = 0; i < 3; i++) {
            for (int j = 0; j < 3; j++) {
                if (i != 2 || j != 2) {
                    ans = (ans - (lll)h_arr[i] * h_arr[j] % M + M) % M;
                }
            }
        }
        ans = (ans + H[1][N/2]) % M;
        ans = (lll)ans * mod_inv(2) % M;
    }

    printf("%lld\n", (ans % M + M) % M);
    return 0;
}
