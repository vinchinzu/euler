/*
 * Project Euler Problem 544: Chromatic Polynomial.
 * Find sum_{k=1}^{N} f(R, C, k) mod M for a R x C grid graph.
 * N = 1112131415, R = 9, C = 10, M = 10^9+7.
 *
 * Uses DP to compute the chromatic polynomial values at small points,
 * then Lagrange interpolation to extrapolate the cumulative sum to N.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef __int128 i128;

#define R_VAL 9
#define C_VAL 10
#define MOD 1000000007LL
#define RC (R_VAL * C_VAL)
#define F_SIZE (2 * RC + 5)

/* Hash table for memoization of F states */
#define HT_SIZE (1 << 20)
#define HT_MASK (HT_SIZE - 1)

/* Encode state: (r, lastColors as normalized tuple)
 * lastColors has at most R_VAL elements, each 1..R_VAL+1
 * We can encode them as a base-(R_VAL+2) number.
 */
typedef struct {
    ll key;       /* encoded state */
    int used;
    ll *F_arr;    /* array of F_SIZE values */
} HState;

static HState *ht = NULL;

static void ht_init(void) {
    ht = (HState*)calloc(HT_SIZE, sizeof(HState));
}

static ll encode_state(int r, int *lastColors, int ncolors) {
    ll key = r;
    for (int i = 0; i < ncolors; i++)
        key = key * (R_VAL + 2) + lastColors[i];
    key = key * (R_VAL + 2) + ncolors; /* disambiguate lengths */
    return key;
}

static HState* ht_find(ll key) {
    unsigned int idx = (unsigned int)((unsigned long long)key * 2654435761ULL) & HT_MASK;
    for (int i = 0; i < HT_SIZE; i++) {
        unsigned int pos = (idx + i) & HT_MASK;
        if (!ht[pos].used) return &ht[pos];
        if (ht[pos].key == key) return &ht[pos];
    }
    return NULL;
}

/* Recursive F computation */
static ll* F(int r, int *colors, int ncolors) {
    /* Normalize: keep only last R colors, relabel starting from 1 */
    int lastColors[R_VAL + 1];
    int nlast = 0;
    int mapping[2 * R_VAL + 2];
    memset(mapping, 0, sizeof(mapping));
    int currMax = 0;

    int start = (ncolors > R_VAL) ? ncolors - R_VAL : 0;
    for (int i = start; i < ncolors; i++) {
        int c = colors[i];
        if (mapping[c] == 0) {
            currMax++;
            mapping[c] = currMax;
        }
        lastColors[nlast++] = mapping[c];
    }
    int maxColor = currMax;

    ll key = encode_state(r, lastColors, nlast);
    HState *entry = ht_find(key);
    if (entry->used && entry->key == key) return entry->F_arr;

    ll *F_arr = (ll*)calloc(F_SIZE, sizeof(ll));
    entry->key = key;
    entry->F_arr = F_arr;
    entry->used = 1;

    if (r == RC) {
        for (int i = 0; i < F_SIZE; i++) F_arr[i] = 1;
        return F_arr;
    }

    for (int color = 1; color <= maxColor + 1; color++) {
        /* Check horizontal neighbor */
        if (r % R_VAL != 0 && nlast > 0 && color == lastColors[nlast - 1])
            continue;
        /* Check vertical neighbor */
        if (nlast >= R_VAL && color == lastColors[nlast - R_VAL])
            continue;

        /* Build new colors array */
        int newColors[R_VAL + 1];
        int nnew = 0;
        int s2 = (nlast + 1 > R_VAL) ? nlast + 1 - R_VAL : 0;
        for (int i = s2; i < nlast; i++)
            newColors[nnew++] = lastColors[i];
        newColors[nnew++] = color;

        /* But we need to pass the full color list for proper normalization
         * in the recursive call. Actually, since we normalized already,
         * we can pass lastColors + [color] and the recursion will normalize again. */
        int passColors[R_VAL + 2];
        for (int i = 0; i < nlast; i++) passColors[i] = lastColors[i];
        passColors[nlast] = color;

        ll *nextF = F(r + 1, passColors, nlast + 1);

        for (int n_idx = 0; n_idx < F_SIZE; n_idx++) {
            ll choices = (color == maxColor + 1) ? (n_idx - color + 1) : 1;
            if (choices <= 0) continue;
            F_arr[n_idx] = (F_arr[n_idx] + choices * nextF[n_idx]) % MOD;
        }
    }

    return F_arr;
}

/* Lagrange extrapolation */
static ll lagrange_extrapolation(ll *values, int n, ll x) {
    /* values[0..n-1] are at points 1, 2, ..., n */
    ll *prefix = (ll*)malloc((n + 1) * sizeof(ll));
    ll *suffix = (ll*)malloc((n + 1) * sizeof(ll));

    prefix[0] = 1;
    for (int i = 0; i < n; i++)
        prefix[i + 1] = (i128)prefix[i] * ((x - (i + 1)) % MOD + MOD) % MOD;

    suffix[n] = 1;
    for (int i = n - 1; i >= 0; i--)
        suffix[i] = (i128)suffix[i + 1] * ((x - (i + 1)) % MOD + MOD) % MOD;

    ll *fact = (ll*)malloc((n + 1) * sizeof(ll));
    ll *inv_fact = (ll*)malloc((n + 1) * sizeof(ll));
    fact[0] = 1;
    for (int i = 1; i <= n; i++) fact[i] = (i128)fact[i-1] * i % MOD;

    /* Fermat inverse of fact[n] */
    ll base = fact[n], e = MOD - 2, inv_n = 1;
    while (e > 0) {
        if (e & 1) inv_n = (i128)inv_n * base % MOD;
        base = (i128)base * base % MOD;
        e >>= 1;
    }
    inv_fact[n] = inv_n;
    for (int i = n - 1; i >= 0; i--)
        inv_fact[i] = (i128)inv_fact[i+1] * (i+1) % MOD;

    ll result = 0;
    for (int i = 0; i < n; i++) {
        ll num = (i128)prefix[i] * suffix[i + 1] % MOD;
        ll denom = (i128)inv_fact[i] * inv_fact[n - 1 - i] % MOD;
        if ((n - 1 - i) % 2 == 1)
            denom = (MOD - denom) % MOD;
        result = (result + (i128)values[i] % MOD * num % MOD * denom) % MOD;
    }

    free(prefix); free(suffix); free(fact); free(inv_fact);
    return (result % MOD + MOD) % MOD;
}

int main(void) {
    ll N = 1112131415LL;

    ht_init();

    int empty[1];
    ll *F_result = F(0, empty, 0);

    /* Compute cumulative sum values at points 1, 2, ..., RC+2 */
    int n_points = RC + 2;
    ll *sum_values = (ll*)malloc(n_points * sizeof(ll));

    for (int k = 1; k <= n_points; k++) {
        ll S = 0;
        for (int i = 0; i <= k && i < F_SIZE; i++) {
            S = (S + F_result[i]) % MOD;
        }
        sum_values[k - 1] = S;
    }

    ll ans = lagrange_extrapolation(sum_values, n_points, N);
    printf("%lld\n", (long long)ans);

    free(sum_values);
    /* Note: not freeing all ht entries for simplicity, program exits anyway */
    free(ht);
    return 0;
}
