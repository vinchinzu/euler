/*
 * Project Euler 682 - 5-Smooth Pairs
 *
 * The number of 5-smooth pairs satisfies a linear recurrence.
 * Compute small values, find recurrence via Berlekamp-Massey,
 * then use polynomial exponentiation to evaluate at N = 10^7.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef __int128 lll;

#define MOD 1000000007LL
#define MAX_REC 80

static ll vals[MAX_REC];

ll power_mod(ll base, ll exp, ll mod) {
    ll r = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) r = (lll)r * base % mod;
        base = (lll)base * base % mod;
        exp >>= 1;
    }
    return r;
}

ll inv_mod(ll a) { return power_mod(a, MOD - 2, MOD); }

/* Berlekamp-Massey: find shortest linear recurrence for s[] mod MOD.
   Returns length L, coefficients in rec[0..L-1] such that
   s[n] = rec[0]*s[n-1] + rec[1]*s[n-2] + ... + rec[L-1]*s[n-L]. */
static ll bm_C[MAX_REC + 1], bm_B[MAX_REC + 1], bm_T[MAX_REC + 1];
static ll rec[MAX_REC];
static int rec_len;

void berlekamp_massey(ll *s, int n) {
    memset(bm_C, 0, sizeof(bm_C));
    memset(bm_B, 0, sizeof(bm_B));
    bm_C[0] = 1;
    bm_B[0] = 1;
    int L = 0, m = 1;
    ll b = 1;
    int len_C = 1, len_B = 1;

    for (int i = 0; i < n; i++) {
        ll d = s[i];
        for (int j = 1; j <= L; j++)
            d = (d + (lll)bm_C[j] * s[i - j]) % MOD;
        d = (d % MOD + MOD) % MOD;

        if (d == 0) {
            m++;
        } else if (2 * L <= i) {
            memcpy(bm_T, bm_C, sizeof(ll) * len_C);
            int old_len_C = len_C;
            ll coeff = (lll)d * inv_mod(b) % MOD;
            if (len_B + m > len_C) {
                len_C = len_B + m;
            }
            for (int j = 0; j < len_B; j++)
                bm_C[j + m] = (bm_C[j + m] - (lll)coeff * bm_B[j] % MOD + MOD) % MOD;
            L = i + 1 - L;
            memcpy(bm_B, bm_T, sizeof(ll) * old_len_C);
            len_B = old_len_C;
            b = d;
            m = 1;
        } else {
            ll coeff = (lll)d * inv_mod(b) % MOD;
            if (len_B + m > len_C) {
                len_C = len_B + m;
            }
            for (int j = 0; j < len_B; j++)
                bm_C[j + m] = (bm_C[j + m] - (lll)coeff * bm_B[j] % MOD + MOD) % MOD;
            m++;
        }
    }

    rec_len = L;
    for (int i = 0; i < L; i++)
        rec[i] = (MOD - bm_C[i + 1]) % MOD;
}

/* Polynomial multiplication mod characteristic polynomial, degree < L */
static ll poly_a[MAX_REC], poly_b[MAX_REC], poly_tmp[2 * MAX_REC];

void poly_mult(ll *a, ll *b, ll *out, int L) {
    memset(poly_tmp, 0, sizeof(ll) * 2 * L);
    for (int i = 0; i < L; i++) {
        if (a[i] == 0) continue;
        for (int j = 0; j < L; j++)
            poly_tmp[i + j] = (poly_tmp[i + j] + (lll)a[i] * b[j]) % MOD;
    }
    /* Reduce: x^L = rec[0]*x^(L-1) + ... + rec[L-1] */
    for (int i = 2 * L - 1; i >= L; i--) {
        if (poly_tmp[i] == 0) continue;
        ll c = poly_tmp[i];
        poly_tmp[i] = 0;
        for (int j = 0; j < L; j++)
            poly_tmp[i - L + j] = (poly_tmp[i - L + j] + (lll)c * rec[L - 1 - j]) % MOD;
    }
    memcpy(out, poly_tmp, sizeof(ll) * L);
}

ll linear_recurrence_nth(ll *init, int L, ll n) {
    if (n < L) return init[n] % MOD;

    ll base[MAX_REC], result[MAX_REC], tmp[MAX_REC];
    memset(base, 0, sizeof(ll) * L);
    memset(result, 0, sizeof(ll) * L);
    result[0] = 1;
    if (L > 1) base[1] = 1;
    else base[0] = rec[0];

    ll exp = n;
    while (exp > 0) {
        if (exp & 1) {
            poly_mult(result, base, tmp, L);
            memcpy(result, tmp, sizeof(ll) * L);
        }
        poly_mult(base, base, tmp, L);
        memcpy(base, tmp, sizeof(ll) * L);
        exp >>= 1;
    }

    ll ans = 0;
    for (int i = 0; i < L; i++)
        ans = (ans + (lll)result[i] * init[i]) % MOD;
    return ans;
}

/* Compute f(n) by brute force for small n */
ll f(int n) {
    ll count = 0;
    for (int p2 = 0; 2 * p2 < n; p2++) {
        for (int p3 = 0; 2 * p2 + 3 * p3 < n; p3++) {
            for (int p5 = 0; 2 * p2 + 3 * p3 + 5 * p5 < n; p5++) {
                for (int q2 = 0; 2 * p2 + 3 * p3 + 5 * p5 + 2 * q2 <= n; q2++) {
                    for (int q3 = 0; 2 * p2 + 3 * p3 + 5 * p5 + 2 * q2 + 3 * q3 <= n; q3++) {
                        int q5 = p2 + p3 + p5 - q2 - q3;
                        if (q5 >= 0 && 2*p2 + 3*p3 + 5*p5 + 2*q2 + 3*q3 + 5*q5 == n)
                            count++;
                    }
                }
            }
        }
    }
    return count;
}

int main(void) {
    ll N = 10000000LL;
    int num_values = 80;

    for (int i = 0; i < num_values; i++)
        vals[i] = f(i) % MOD;

    berlekamp_massey(vals, num_values);

    ll ans = linear_recurrence_nth(vals, rec_len, N);
    printf("%lld\n", ans);
    return 0;
}
