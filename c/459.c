/*
 * Project Euler 459 - Flipping game
 *
 * Extracted from embedded C in python/459.py.
 * Uses Sprague-Grundy theory with nim-products.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;

#define N 1000000
#define L 512

static int nim_prod_cache[L][L];
static int nim_prod_computed[L][L];

int nim_prod(int a, int b);

int nim_prod(int a, int b) {
    if (a == 0 || b == 0) return 0;
    if (a == 1) return b;
    if (b == 1) return a;
    if (a < L && b < L && nim_prod_computed[a][b]) return nim_prod_cache[a][b];

    int result;

    if (a & (a-1)) {
        int low = a & (-a);
        result = nim_prod(low, b) ^ nim_prod(a ^ low, b);
    } else if (b & (b-1)) {
        int low = b & (-b);
        result = nim_prod(a, low) ^ nim_prod(a, b ^ low);
    } else {
        int i = __builtin_ctz(a);
        int j = __builtin_ctz(b);
        if ((i & j) == 0) {
            result = a * b;
        } else {
            int common = i & j;
            int common_bit = 0;
            {
                int temp = common;
                while (temp > 1) { temp >>= 1; common_bit++; }
            }
            int fk = 1 << common_bit;
            int Fk = 1 << fk;
            int sq_Fk = Fk + (Fk >> 1);

            int a2 = 1 << (i - fk);
            int b2 = 1 << (j - fk);
            result = nim_prod(nim_prod(a2, b2), sq_Fk);
        }
    }

    if (a < L && b < L) {
        nim_prod_cache[a][b] = result;
        nim_prod_computed[a][b] = 1;
    }
    return result;
}

int main(void) {
    memset(nim_prod_computed, 0, sizeof(nim_prod_computed));

    int nsq = 0, ntr = 0;
    int *sq_steps = (int*)malloc((N+1) * sizeof(int));
    int *tr_steps = (int*)malloc((N+1) * sizeof(int));
    for (int i = 1; (ll)i*i <= N; i++) sq_steps[nsq++] = i*i;
    for (int i = 1; (ll)i*(i+1)/2 <= N; i++) tr_steps[ntr++] = i*(i+1)/2;

    int *rnX = (int*)calloc(N+1, sizeof(int));
    int *rnY = (int*)calloc(N+1, sizeof(int));

    for (int j = 1; j <= N; j++) {
        int used[L];
        memset(used, 0, sizeof(used));
        for (int k = 0; k < nsq && sq_steps[k] <= j; k++) {
            int val = rnX[j-1] ^ rnX[j - sq_steps[k]];
            if (val < L) used[val] = 1;
        }
        int mex = 0;
        while (mex < L && used[mex]) mex++;
        rnX[j] = rnX[j-1] ^ mex;
    }

    for (int j = 1; j <= N; j++) {
        int used[L];
        memset(used, 0, sizeof(used));
        for (int k = 0; k < ntr && tr_steps[k] <= j; k++) {
            int val = rnY[j-1] ^ rnY[j - tr_steps[k]];
            if (val < L) used[val] = 1;
        }
        int mex = 0;
        while (mex < L && used[mex]) mex++;
        rnY[j] = rnY[j-1] ^ mex;
    }

    ll *cntX = (ll*)calloc(L, sizeof(ll));
    ll *cntY = (ll*)calloc(L, sizeof(ll));

    for (int j = 1; j <= N; j++) {
        for (int k = 0; k < nsq && sq_steps[k] <= j; k++) {
            int val = rnX[j] ^ rnX[j - sq_steps[k]];
            if (val < L) cntX[val]++;
        }
    }
    for (int j = 1; j <= N; j++) {
        for (int k = 0; k < ntr && tr_steps[k] <= j; k++) {
            int val = rnY[j] ^ rnY[j - tr_steps[k]];
            if (val < L) cntY[val]++;
        }
    }

    int target = nim_prod(rnX[N], rnY[N]);

    ll ans = 0;
    for (int n0 = 0; n0 < L; n0++) {
        if (cntX[n0] == 0) continue;
        for (int n1 = 0; n1 < L; n1++) {
            if (cntY[n1] == 0) continue;
            if (nim_prod(n0, n1) == target) {
                ans += cntX[n0] * cntY[n1];
            }
        }
    }

    printf("%lld\n", ans);

    free(sq_steps); free(tr_steps);
    free(rnX); free(rnY);
    free(cntX); free(cntY);
    return 0;
}
