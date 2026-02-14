/*
 * Project Euler 663 - Sums of Subarrays
 *
 * Segment tree for maximum subarray sum. Tribonacci-based updates.
 * N = 10000003, L1 = 10000000, L2 = 10200000.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;

static int seg_L;
static ll *sums, *max_pre, *max_suf, *max_sub;

static inline ll max2(ll a, ll b) { return a > b ? a : b; }
static inline ll max3(ll a, ll b, ll c) { ll m = a > b ? a : b; return m > c ? m : c; }

static void seg_init(int size) {
    seg_L = 1;
    while (seg_L < size) seg_L *= 2;
    sums = (ll *)calloc(2 * seg_L, sizeof(ll));
    max_pre = (ll *)calloc(2 * seg_L, sizeof(ll));
    max_suf = (ll *)calloc(2 * seg_L, sizeof(ll));
    max_sub = (ll *)calloc(2 * seg_L, sizeof(ll));
}

static void seg_update_leaf(int index, ll val) {
    int idx = seg_L + index;
    sums[idx] = val;
    max_pre[idx] = val;
    max_suf[idx] = val;
    max_sub[idx] = val;
}

static void seg_merge(int index) {
    int left = 2 * index;
    int right = 2 * index + 1;
    sums[index] = sums[left] + sums[right];
    max_pre[index] = max2(max_pre[left], sums[left] + max_pre[right]);
    max_suf[index] = max2(max_suf[right], max_suf[left] + sums[right]);
    max_sub[index] = max3(max_sub[left], max_sub[right],
                          max_suf[left] + max_pre[right]);
}

static void seg_build(void) {
    for (int j = seg_L - 1; j > 0; j--) {
        seg_merge(j);
    }
}

static void seg_update_and_rebuild(int index, ll val) {
    seg_update_leaf(index, val);
    int j = (seg_L + index) / 2;
    while (j > 0) {
        seg_merge(j);
        j /= 2;
    }
}

int main() {
    int N = 10000003;
    int L1 = 10000000;
    int L2 = 10200000;

    ll *A = (ll *)calloc(N, sizeof(ll));
    seg_init(N);

    ll a = 0, b = 0, c = 1;
    ll ans = 0;

    for (int i = 1; i <= L2; i++) {
        A[a] += 2 * b - N + 1;
        if (i == L1) {
            for (int j = 0; j < N; j++) {
                seg_update_leaf(j, A[j]);
            }
            seg_build();
        } else if (i > L1) {
            seg_update_and_rebuild((int)a, A[a]);
            ans += max_sub[1];
        }

        ll new_a = c;
        ll new_b = (a + b + new_a) % N;
        ll new_c = (b + c + new_b) % N;
        a = new_a;
        b = new_b;
        c = new_c;
    }

    printf("%lld\n", ans);

    free(A);
    free(sums);
    free(max_pre);
    free(max_suf);
    free(max_sub);
    return 0;
}
