/*
 * Project Euler 691 - Long Substring
 *
 * Build suffix array + LCP array, then sweep LCP thresholds from high to low
 * with union-find to compute L(k) = longest substring appearing >= k times.
 * Sum L(k) for k=1..N.
 *
 * Extracted from embedded C in Python solution.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define MAXN 5000002

static int sa[MAXN], rnk[MAXN], tmp2[MAXN], lcp_arr[MAXN];
static int cnt2[MAXN];
static unsigned char S[MAXN];
static int parent[MAXN], sz_arr[MAXN];

void build_sa(int n) {
    int i, k, classes;
    for (i = 0; i < n; i++) { sa[i] = i; rnk[i] = S[i]; }
    int max_val = 256;
    memset(cnt2, 0, max_val * sizeof(int));
    for (i = 0; i < n; i++) cnt2[rnk[i]]++;
    for (i = 1; i < max_val; i++) cnt2[i] += cnt2[i-1];
    for (i = n-1; i >= 0; i--) sa[--cnt2[rnk[i]]] = i;

    for (k = 1; k < n; k <<= 1) {
        int p = 0;
        for (i = n - k; i < n; i++) tmp2[p++] = i;
        for (i = 0; i < n; i++) if (sa[i] >= k) tmp2[p++] = sa[i] - k;

        max_val = 0;
        for (i = 0; i < n; i++) if (rnk[i] > max_val) max_val = rnk[i];
        max_val++;
        memset(cnt2, 0, max_val * sizeof(int));
        for (i = 0; i < n; i++) cnt2[rnk[tmp2[i]]]++;
        for (i = 1; i < max_val; i++) cnt2[i] += cnt2[i-1];
        for (i = n-1; i >= 0; i--) sa[--cnt2[rnk[tmp2[i]]]] = tmp2[i];

        tmp2[sa[0]] = 0;
        classes = 1;
        for (i = 1; i < n; i++) {
            int s1 = sa[i-1], s2 = sa[i];
            int r1a = rnk[s1], r1b = (s1+k < n) ? rnk[s1+k] : -1;
            int r2a = rnk[s2], r2b = (s2+k < n) ? rnk[s2+k] : -1;
            if (r1a != r2a || r1b != r2b) classes++;
            tmp2[sa[i]] = classes - 1;
        }
        memcpy(rnk, tmp2, n * sizeof(int));
        if (classes == n) break;
    }
}

void build_lcp(int n) {
    int i, h = 0;
    for (i = 0; i < n; i++) tmp2[sa[i]] = i;
    for (i = 0; i < n; i++) {
        if (tmp2[i] > 0) {
            int j = sa[tmp2[i] - 1];
            while (i + h < n && j + h < n && S[i+h] == S[j+h]) h++;
            lcp_arr[tmp2[i]] = h;
            if (h > 0) h--;
        } else {
            lcp_arr[0] = 0;
            h = 0;
        }
    }
}

int find(int x) {
    while (parent[x] != x) { parent[x] = parent[parent[x]]; x = parent[x]; }
    return x;
}

int global_max_sz;
void unite(int a, int b) {
    a = find(a); b = find(b);
    if (a == b) return;
    if (sz_arr[a] < sz_arr[b]) { int t = a; a = b; b = t; }
    parent[b] = a;
    sz_arr[a] += sz_arr[b];
    if (sz_arr[a] > global_max_sz) global_max_sz = sz_arr[a];
}

int main(void) {
    int N = 5000000;
    long double PHI = (1.0L + sqrtl(5.0L)) / 2.0L;

    unsigned char *a = (unsigned char *)calloc(N + 1, 1);
    a[0] = 0;
    for (int i = 1; i <= N; i++) {
        if (i % 2 == 0) a[i] = a[i/2];
        else a[i] = 1 - a[i/2];
    }
    for (int i = 0; i < N; i++) {
        int bn = (int)floorl((i + 1) / PHI) - (int)floorl(i / PHI);
        S[i] = (a[i] ^ bn) + '0';
    }
    free(a);

    S[N] = 0;
    int n = N + 1;
    build_sa(n);
    build_lcp(n);

    int max_lcp = 0;
    for (int i = 1; i < n; i++)
        if (lcp_arr[i] > max_lcp) max_lcp = lcp_arr[i];

    int *lcp_cnt = (int *)calloc(max_lcp + 2, sizeof(int));
    for (int i = 1; i < n; i++)
        if (lcp_arr[i] > 0) lcp_cnt[lcp_arr[i]]++;

    int *prefix_sum = (int *)calloc(max_lcp + 2, sizeof(int));
    prefix_sum[1] = 0;
    for (int v = 2; v <= max_lcp; v++)
        prefix_sum[v] = prefix_sum[v-1] + lcp_cnt[v-1];
    int total = prefix_sum[max_lcp] + lcp_cnt[max_lcp];

    int *idx_buf = (int *)malloc((total + 1) * sizeof(int));
    int *pos = (int *)malloc((max_lcp + 2) * sizeof(int));
    memcpy(pos, prefix_sum, (max_lcp + 2) * sizeof(int));
    for (int i = 1; i < n; i++)
        if (lcp_arr[i] > 0)
            idx_buf[pos[lcp_arr[i]]++] = i;

    for (int i = 0; i < n; i++) { parent[i] = i; sz_arr[i] = 1; }
    int *activated = (int *)calloc(n, sizeof(int));

    long long *L = (long long *)calloc(N + 2, sizeof(long long));
    L[1] = N;
    int max_freq = 1;
    global_max_sz = 0;

    for (int v = max_lcp; v >= 1; v--) {
        int si = prefix_sum[v];
        int ei = si + lcp_cnt[v];

        for (int j = si; j < ei; j++) {
            int i = idx_buf[j];
            activated[i] = 1;
            if (global_max_sz == 0) global_max_sz = 1;
            if (i > 1 && activated[i-1]) unite(i, i-1);
            if (i < n-1 && activated[i+1]) unite(i, i+1);
        }

        int freq = global_max_sz + 1;
        if (freq > max_freq) {
            for (int k = max_freq + 1; k <= freq; k++)
                L[k] = v;
            max_freq = freq;
        }
    }

    long long ans = 0;
    for (int k = 1; k <= N; k++) ans += L[k];

    printf("%lld\n", ans);

    free(lcp_cnt); free(prefix_sum); free(activated); free(L);
    free(idx_buf); free(pos);
    return 0;
}
