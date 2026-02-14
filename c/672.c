/*
 * Project Euler 672 - One More One
 *
 * Berlekamp-Massey to find linear recurrence of H(k), then
 * polynomial exponentiation to evaluate at N=10^9.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef __int128 lll;

#define MOD 1117117717LL
#define B_VAL 7
#define K_VAL 11
#define MAX_REC 60

static ll mod_p(ll a) {
    a %= MOD;
    if (a < 0) a += MOD;
    return a;
}

static ll mod_inv(ll a) {
    ll result = 1;
    ll exp = MOD - 2;
    a = mod_p(a);
    while (exp > 0) {
        if (exp & 1) result = (lll)result * a % MOD;
        a = (lll)a * a % MOD;
        exp >>= 1;
    }
    return result;
}

static ll tr(ll n) {
    return n * (n + 1) / 2;
}

static ll H_func(int k) {
    ll n = B_VAL - 1;
    ll n_div_b = 0;
    ll g = 0;
    ll H_val = 0;
    for (int i = 1; i < k; i++) {
        n = n * B_VAL + (B_VAL - 1);
        ll digit = n / K_VAL;
        H_val = mod_p(B_VAL * H_val + n_div_b * tr(B_VAL - 1) + digit * g
                       + tr(B_VAL - 2) - tr(B_VAL - 1 - digit));
        n -= digit * K_VAL;
        n_div_b = mod_p(B_VAL * n_div_b + digit);
        g += B_VAL - 1 - digit;
    }
    return H_val;
}

/* Berlekamp-Massey */
static int berlekamp_massey(ll *s, int n, ll *rec) {
    ll C[MAX_REC + 1], B_arr[MAX_REC + 1], T[MAX_REC + 1];
    int lenC = 1, lenB = 1;
    memset(C, 0, sizeof(C)); C[0] = 1;
    memset(B_arr, 0, sizeof(B_arr)); B_arr[0] = 1;
    int L = 0, m = 1;
    ll b = 1;

    for (int i = 0; i < n; i++) {
        ll d = s[i];
        for (int j = 1; j <= L; j++) {
            d = mod_p(d + (lll)C[j] * s[i - j] % MOD);
        }
        if (d == 0) {
            m++;
        } else if (2 * L <= i) {
            memcpy(T, C, sizeof(ll) * (L + 2));
            int oldL = L;
            ll coeff = (lll)d * mod_inv(b) % MOD;
            int newlen = lenB + m;
            if (newlen > lenC) {
                for (int j = lenC; j < newlen; j++) C[j] = 0;
                lenC = newlen;
            }
            for (int j = 0; j < lenB; j++) {
                C[j + m] = mod_p(C[j + m] - (lll)coeff * B_arr[j] % MOD);
            }
            L = i + 1 - L;
            memcpy(B_arr, T, sizeof(ll) * (oldL + 2));
            lenB = oldL + 1;
            b = d;
            m = 1;
        } else {
            ll coeff = (lll)d * mod_inv(b) % MOD;
            int newlen = lenB + m;
            if (newlen > lenC) {
                for (int j = lenC; j < newlen; j++) C[j] = 0;
                lenC = newlen;
            }
            for (int j = 0; j < lenB; j++) {
                C[j + m] = mod_p(C[j + m] - (lll)coeff * B_arr[j] % MOD);
            }
            m++;
        }
    }

    for (int i = 0; i < L; i++) {
        rec[i] = mod_p(-C[i + 1]);
    }
    return L;
}

/* Polynomial multiplication modulo characteristic polynomial */
static void poly_mult(ll *a, ll *b, ll *rec, int L, ll *result) {
    ll tmp[2 * MAX_REC];
    memset(tmp, 0, sizeof(ll) * 2 * L);
    for (int i = 0; i < L; i++) {
        if (a[i] == 0) continue;
        for (int j = 0; j < L; j++) {
            tmp[i + j] = mod_p(tmp[i + j] + (lll)a[i] * b[j] % MOD);
        }
    }
    for (int i = 2 * L - 1; i >= L; i--) {
        if (tmp[i] == 0) continue;
        ll c = tmp[i];
        tmp[i] = 0;
        for (int j = 0; j < L; j++) {
            tmp[i - L + j] = mod_p(tmp[i - L + j] + (lll)c * rec[L - 1 - j] % MOD);
        }
    }
    memcpy(result, tmp, sizeof(ll) * L);
}

static ll linear_recurrence_nth(ll *rec, ll *init, int L, ll n) {
    if (n < L) return init[n] % MOD;

    ll base[MAX_REC], result[MAX_REC];
    memset(base, 0, sizeof(ll) * L);
    memset(result, 0, sizeof(ll) * L);
    if (L > 1) base[1] = 1;
    else base[0] = rec[0];
    result[0] = 1;

    ll exp = n;
    while (exp > 0) {
        if (exp & 1) {
            ll tmp[MAX_REC];
            poly_mult(result, base, rec, L, tmp);
            memcpy(result, tmp, sizeof(ll) * L);
        }
        ll tmp[MAX_REC];
        poly_mult(base, base, rec, L, tmp);
        memcpy(base, tmp, sizeof(ll) * L);
        exp >>= 1;
    }

    ll ans = 0;
    for (int i = 0; i < L; i++) {
        ans = mod_p(ans + (lll)result[i] * init[i] % MOD);
    }
    return ans;
}

int main() {
    ll N_target = 1000000000LL;  /* 10^9 */
    int num_values = 60;

    ll vals[60];
    for (int k = 1; k <= num_values; k++) {
        vals[k - 1] = H_func(k);
    }

    ll rec[MAX_REC];
    int L = berlekamp_massey(vals, num_values, rec);

    ll ans = linear_recurrence_nth(rec, vals, L, N_target - 1);
    printf("%lld\n", ans % MOD);
    return 0;
}
