/*
 * Project Euler Problem 716: Grid Graphs.
 *
 * Compute C(h, w) for small h and w by brute force, use Berlekamp-Massey
 * to find recurrence in w, extrapolate to W=20000 via Kitamasa. Then find
 * recurrence in h and extrapolate to H=10000.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef __int128 lll;

#define MOD 1000000007LL
#define MAX_H 10
#define MAX_W 10

ll mod(ll a) {
    return ((a % MOD) + MOD) % MOD;
}

ll power(ll base, ll exp, ll m) {
    ll result = 1;
    base = ((base % m) + m) % m;
    while (exp > 0) {
        if (exp & 1) result = (lll)result * base % m;
        base = (lll)base * base % m;
        exp >>= 1;
    }
    return result;
}

ll inv(ll a) { return power(a, MOD - 2, MOD); }

/* Berlekamp-Massey: find shortest linear recurrence mod MOD (prime) */
/* Returns length L and fills rec[0..L-1] with recurrence coefficients. */
int berlekamp_massey(ll *seq, int n, ll *rec) {
    ll *C = calloc(n + 2, sizeof(ll));
    ll *B = calloc(n + 2, sizeof(ll));
    int clen = 1, blen = 1;
    C[0] = 1; B[0] = 1;
    int L = 0, m = 1;
    ll b = 1;

    for (int i = 0; i < n; i++) {
        ll d = seq[i];
        for (int j = 1; j <= L; j++)
            d = (d + C[j] * seq[i - j]) % MOD;
        d = mod(d);

        if (d == 0) {
            m++;
        } else if (2 * L <= i) {
            ll *T = malloc((clen + 1) * sizeof(ll));
            memcpy(T, C, clen * sizeof(ll));
            int tlen = clen;

            ll coeff = (lll)d * inv(b) % MOD;
            int newlen = blen + m;
            while (clen < newlen) C[clen++] = 0;
            for (int j = 0; j < blen; j++)
                C[j + m] = mod(C[j + m] - (lll)coeff * B[j] % MOD);

            L = i + 1 - L;
            memcpy(B, T, tlen * sizeof(ll));
            blen = tlen;
            b = d;
            m = 1;
            free(T);
        } else {
            ll coeff = (lll)d * inv(b) % MOD;
            int newlen = blen + m;
            while (clen < newlen) C[clen++] = 0;
            for (int j = 0; j < blen; j++)
                C[j + m] = mod(C[j + m] - (lll)coeff * B[j] % MOD);
            m++;
        }
    }

    for (int i = 0; i < L; i++)
        rec[i] = mod(-C[i + 1]);

    free(C);
    free(B);
    return L;
}

/* Polynomial multiplication mod characteristic polynomial and MOD */
void poly_mult_mod(ll *a, int alen, ll *b, int blen, ll *rec, int L, ll *out) {
    ll *raw = calloc(alen + blen, sizeof(ll));
    for (int i = 0; i < alen; i++) {
        if (a[i] == 0) continue;
        for (int j = 0; j < blen; j++)
            raw[i + j] = (raw[i + j] + (lll)a[i] * b[j]) % MOD;
    }

    /* Reduce mod characteristic polynomial x^L - rec[0]*x^{L-1} - ... - rec[L-1] */
    ll *rep = calloc(L, sizeof(ll));
    for (int i = 0; i < L; i++) rep[i] = rec[L - 1 - i];

    for (int i = alen + blen - 2; i >= L; i--) {
        if (raw[i] == 0) continue;
        ll c = raw[i];
        raw[i] = 0;
        for (int j = 0; j < L; j++)
            raw[i - L + j] = (raw[i - L + j] + (lll)c * rep[j]) % MOD;
    }

    for (int i = 0; i < L; i++) out[i] = mod(raw[i]);
    free(raw);
    free(rep);
}

ll eval_recurrence(ll *rec, ll *init, int L, ll n) {
    if (n < L) return mod(init[n]);

    ll *result = calloc(L, sizeof(ll));
    ll *base = calloc(L, sizeof(ll));
    ll *tmp = calloc(L, sizeof(ll));
    result[0] = 1;
    if (L > 1) base[1] = 1;
    else base[0] = mod(rec[0]);

    ll exp = n;
    while (exp > 0) {
        if (exp & 1) {
            poly_mult_mod(result, L, base, L, rec, L, tmp);
            memcpy(result, tmp, L * sizeof(ll));
        }
        poly_mult_mod(base, L, base, L, rec, L, tmp);
        memcpy(base, tmp, L * sizeof(ll));
        exp >>= 1;
    }

    ll ans = 0;
    for (int i = 0; i < L; i++)
        ans = (ans + (lll)result[i] * init[i]) % MOD;

    free(result);
    free(base);
    free(tmp);
    return mod(ans);
}

/* Precompute properties for all bitmasks of length n */
typedef struct {
    int first0, first1, last0, last1, b0, blast;
} Props;

void precompute_props(int n, Props *props) {
    for (int mask = 0; mask < (1 << n); mask++) {
        int f0 = -1, f1 = -1, l0 = -1, l1 = -1;
        int b0_val = (mask >> 0) & 1;
        int blast_val = (mask >> (n - 1)) & 1;
        for (int j = 0; j < n; j++) {
            int bit = (mask >> j) & 1;
            if (bit == 0) {
                if (f0 == -1) f0 = j;
                l0 = j;
            } else {
                if (f1 == -1) f1 = j;
                l1 = j;
            }
        }
        props[mask] = (Props){f0, f1, l0, l1, b0_val, blast_val};
    }
}

int imax(int a, int b) { return a > b ? a : b; }

ll compute_C(int h, int w) {
    int vsz = 1 << h;
    int hsz = 1 << w;
    Props *vprops = malloc(vsz * sizeof(Props));
    Props *hprops = malloc(hsz * sizeof(Props));
    precompute_props(h, vprops);
    precompute_props(w, hprops);

    ll total = 0;
    for (int vi = 0; vi < vsz; vi++) {
        Props *vp = &vprops[vi];
        for (int hi_val = 0; hi_val < hsz; hi_val++) {
            Props *hp = &hprops[hi_val];

            int x1 = vp->b0 == 0 ? hp->first1 : hp->first0;
            int y1 = hp->b0 == 0 ? vp->first1 : vp->first0;
            int x2 = vp->b0 == 0 ? hp->last0 : hp->last1;
            int y2 = hp->blast == 0 ? vp->first0 : vp->first1;
            int x3 = vp->blast == 0 ? hp->first0 : hp->first1;
            int y3 = hp->b0 == 0 ? vp->last0 : vp->last1;
            int x4 = vp->blast == 0 ? hp->last1 : hp->last0;
            int y4 = hp->blast == 0 ? vp->last1 : vp->last0;

            if (x1 == -1 || x2 == -1 || y1 == -1 || y3 == -1) {
                total += w * h;
            } else {
                int area = x1 * y1
                    + (w - 1 - imax(x1 - 1, x2)) * y2
                    + x3 * (h - 1 - imax(y1 - 1, y3))
                    + (w - 1 - imax(x3 - 1, x4)) * (h - 1 - imax(y2 - 1, y4));
                if (area < w * h) area += 1;
                total += area;
            }
        }
    }

    free(vprops);
    free(hprops);
    return total;
}

int main() {
    int H = 10000;
    int W = 20000;

    /* Step 1: For each h, compute C(h, w) for small w, find recurrence, extrapolate to W */
    ll vals_at_W[MAX_H];
    for (int h = 1; h <= MAX_H; h++) {
        ll row[MAX_W];
        for (int w = 1; w <= MAX_W; w++) {
            row[w - 1] = compute_C(h, w) % MOD;
        }

        ll rec[MAX_W];
        int L = berlekamp_massey(row, MAX_W, rec);
        ll init[MAX_W];
        for (int i = 0; i < L; i++) init[i] = row[i];
        vals_at_W[h - 1] = eval_recurrence(rec, init, L, W - 1);
    }

    /* Step 2: Find recurrence in h and extrapolate to H */
    ll rec_h[MAX_H];
    int Lh = berlekamp_massey(vals_at_W, MAX_H, rec_h);
    ll init_h[MAX_H];
    for (int i = 0; i < Lh; i++) init_h[i] = vals_at_W[i];
    ll result = eval_recurrence(rec_h, init_h, Lh, H - 1);

    printf("%lld\n", mod(result));
    return 0;
}
