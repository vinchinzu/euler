/*
 * Project Euler 690 - Tom and Jerry
 *
 * Count graphs on N=2019 vertices where every connected component is a
 * "lobster" graph. Uses generating functions with partition numbers,
 * truncated to degree N, and DP for counting labeled graphs
 * whose components are all lobsters.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef __int128 lll;

#define MAXN 2020
#define MOD 1000000007LL

static ll P[MAXN]; /* partition numbers */
static ll P2[MAXN]; /* P evaluated at x^2 */
static ll recip_1mx[MAXN]; /* 1/(1-x) */
static ll recip_1mx2[MAXN]; /* 1/(1-x^2) */
static ll num_lobsters[MAXN];
static ll mod_invs[MAXN + 2];

/* dp[j] for current i */
static ll dp_prev[MAXN];
static ll dp_cur[MAXN];

ll power_mod(ll base, ll exp) {
    ll r = 1;
    base %= MOD;
    if (base < 0) base += MOD;
    while (exp > 0) {
        if (exp & 1) r = (lll)r * base % MOD;
        base = (lll)base * base % MOD;
        exp >>= 1;
    }
    return r;
}

ll inv_mod(ll a) { return power_mod(a, MOD - 2); }

/* GF operations truncated to degree N-1 (index 0..MAXN-1) */
static ll tmp1[MAXN], tmp2[MAXN], tmp3[MAXN];

void gf_mul(ll *a, ll *b, ll *out) {
    memset(out, 0, sizeof(ll) * MAXN);
    for (int i = 0; i < MAXN; i++) {
        if (a[i] == 0) continue;
        for (int j = 0; j < MAXN - i; j++)
            out[i + j] = (out[i + j] + (lll)a[i] * b[j]) % MOD;
    }
}

void gf_recip(ll *f, ll *out) {
    memset(out, 0, sizeof(ll) * MAXN);
    ll inv_f0 = inv_mod(f[0]);
    for (int i = 0; i < MAXN; i++) {
        if (i == 0) {
            out[0] = inv_f0;
        } else {
            ll s = 0;
            int jmax = (i + 1 < MAXN) ? i + 1 : MAXN;
            for (int j = 1; j < jmax; j++)
                s = (s + (lll)f[j] * out[i - j]) % MOD;
            out[i] = (MOD - s) % MOD * inv_f0 % MOD;
        }
    }
}

int main(void) {
    int N = 2019;

    /* Partition numbers */
    memset(P, 0, sizeof(P));
    P[0] = 1;
    for (int k = 1; k < MAXN; k++)
        for (int i = k; i < MAXN; i++)
            P[i] = (P[i] + P[i - k]) % MOD;

    /* P2[2i] = P[i] */
    memset(P2, 0, sizeof(P2));
    for (int i = 0; 2 * i < MAXN; i++)
        P2[2 * i] = P[i];

    /* 1/(1-x) */
    memset(recip_1mx, 0, sizeof(recip_1mx));
    for (int i = 0; i < MAXN; i++) recip_1mx[i] = 1;

    /* 1/(1-x^2) */
    memset(recip_1mx2, 0, sizeof(recip_1mx2));
    for (int i = 0; i < MAXN; i += 2) recip_1mx2[i] = 1;

    /* term1 = P - 1/(1-x) */
    ll term1[MAXN];
    for (int i = 0; i < MAXN; i++)
        term1[i] = (P[i] - recip_1mx[i] + MOD) % MOD;

    /* term1_sq = term1^2 */
    ll term1_sq[MAXN];
    gf_mul(term1, term1, term1_sq);

    /* one_minus_xP = 1 - x*P */
    ll xP[MAXN];
    memset(xP, 0, sizeof(xP));
    for (int i = 1; i < MAXN; i++) xP[i] = P[i - 1];

    ll one_minus_xP[MAXN];
    memset(one_minus_xP, 0, sizeof(one_minus_xP));
    one_minus_xP[0] = 1;
    for (int i = 0; i < MAXN; i++)
        one_minus_xP[i] = (one_minus_xP[i] - xP[i] + MOD) % MOD;

    /* part_a = term1_sq / one_minus_xP */
    ll recip_omxP[MAXN];
    gf_recip(one_minus_xP, recip_omxP);
    ll part_a[MAXN];
    gf_mul(term1_sq, recip_omxP, part_a);

    /* term2 = P2 - 1/(1-x^2) */
    ll term2[MAXN];
    for (int i = 0; i < MAXN; i++)
        term2[i] = (P2[i] - recip_1mx2[i] + MOD) % MOD;

    /* one_plus_xP = 1 + x*P */
    ll one_plus_xP[MAXN];
    memset(one_plus_xP, 0, sizeof(one_plus_xP));
    one_plus_xP[0] = 1;
    for (int i = 0; i < MAXN; i++)
        one_plus_xP[i] = (one_plus_xP[i] + xP[i]) % MOD;

    /* x2P2 = x^2 * P2 */
    ll x2P2[MAXN];
    memset(x2P2, 0, sizeof(x2P2));
    for (int i = 2; i < MAXN; i++)
        x2P2[i] = P2[i - 2];

    /* one_minus_x2P2 = 1 - x^2 * P2 */
    ll one_minus_x2P2[MAXN];
    memset(one_minus_x2P2, 0, sizeof(one_minus_x2P2));
    one_minus_x2P2[0] = 1;
    for (int i = 0; i < MAXN; i++)
        one_minus_x2P2[i] = (one_minus_x2P2[i] - x2P2[i] + MOD) % MOD;

    /* part_b = term2 * one_plus_xP / one_minus_x2P2 */
    ll recip_omx2P2[MAXN];
    gf_recip(one_minus_x2P2, recip_omx2P2);
    ll tmp_b1[MAXN];
    gf_mul(term2, one_plus_xP, tmp_b1);
    ll part_b[MAXN];
    gf_mul(tmp_b1, recip_omx2P2, part_b);

    /* inner = x^2 * (part_a + part_b) / 2 */
    ll inv2 = inv_mod(2);
    ll inner[MAXN];
    memset(inner, 0, sizeof(inner));
    for (int i = 2; i < MAXN; i++)
        inner[i] = (lll)((part_a[i - 2] + part_b[i - 2]) % MOD) * inv2 % MOD;

    /* correction = x^3 / ((1-x)^2 * (1+x)) */
    /* denom = (1-x)^2 * (1+x) = 1 - x - x^2 + x^3 */
    ll denom[MAXN];
    memset(denom, 0, sizeof(denom));
    denom[0] = 1;
    if (MAXN > 1) denom[1] = MOD - 1;
    if (MAXN > 2) denom[2] = MOD - 1;
    if (MAXN > 3) denom[3] = 1;

    ll recip_denom[MAXN];
    gf_recip(denom, recip_denom);

    ll correction[MAXN];
    memset(correction, 0, sizeof(correction));
    for (int i = 3; i < MAXN; i++)
        correction[i] = recip_denom[i - 3];

    /* num_lobsters = inner + x*P - correction */
    memset(num_lobsters, 0, sizeof(num_lobsters));
    for (int i = 0; i < MAXN; i++) {
        num_lobsters[i] = (inner[i] + xP[i] - correction[i] + 2 * MOD) % MOD;
    }

    /* DP for counting graphs whose components are all lobsters.
       dp[i][j] = number of labeled graphs on i vertices where each
       component has <= j vertices and is a lobster. */
    mod_invs[1] = 1;
    for (int i = 2; i <= N + 1; i++)
        mod_invs[i] = (MOD - (MOD / i) * mod_invs[MOD % i] % MOD) % MOD;

    /* We use two 1D arrays since dp[i][j] depends on dp[i - j*k][j-1] */
    /* dp_prev[i] = dp[i][j-1], dp_cur[i] = dp[i][j] */
    memset(dp_prev, 0, sizeof(dp_prev));
    dp_prev[0] = 1;

    for (int j = 1; j <= N; j++) {
        memcpy(dp_cur, dp_prev, sizeof(ll) * (N + 1));
        for (int i = j; i <= N; i++) {
            ll nCr = 1;
            for (int k = 1; k * j <= i; k++) {
                nCr = (lll)nCr * ((num_lobsters[j] + k - 1 + MOD) % MOD) % MOD * mod_invs[k] % MOD;
                dp_cur[i] = (dp_cur[i] + (lll)nCr * dp_prev[i - j * k]) % MOD;
            }
        }
        memcpy(dp_prev, dp_cur, sizeof(ll) * (N + 1));
    }

    printf("%lld\n", dp_prev[N]);
    return 0;
}
