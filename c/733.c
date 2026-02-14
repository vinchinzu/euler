/* Project Euler 733: Ascending subsequences.
 * BIT (Fenwick tree) with coordinate compression for 4-term ascending subsequences.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAXN 1000001
#define MOD 1000000007LL
#define K 4

static long long seq[MAXN];
static int sorted_vals[MAXN];
static int rank_map[MAXN];

static long long count_bit[K+1][MAXN];
static long long sum_bit[K+1][MAXN];

int N;

int cmp_int(const void *a, const void *b) {
    int va = *(const int *)a, vb = *(const int *)b;
    if (va < vb) return -1;
    if (va > vb) return 1;
    return 0;
}

void bit_add(long long *tree, int i, long long val) {
    for (; i <= N; i += i & (-i))
        tree[i] = (tree[i] + val) % MOD;
}

long long bit_sum(long long *tree, int i) {
    long long s = 0;
    for (; i > 0; i -= i & (-i))
        s = (s + tree[i]) % MOD;
    return s;
}

int main(void) {
    N = 1000000;
    int i, k;

    long long a = 153;
    for (i = 0; i < N; i++) {
        seq[i] = a;
        sorted_vals[i] = (int)a;
        a = (a * 153) % 10000019LL;
    }

    qsort(sorted_vals, N, sizeof(int), cmp_int);
    int unique = 0;
    for (i = 0; i < N; i++) {
        if (i == 0 || sorted_vals[i] != sorted_vals[i-1])
            sorted_vals[unique++] = sorted_vals[i];
    }

    for (i = 0; i < N; i++) {
        int lo = 0, hi = unique - 1, target = (int)seq[i];
        while (lo < hi) {
            int mid = (lo + hi) / 2;
            if (sorted_vals[mid] < target) lo = mid + 1;
            else hi = mid;
        }
        rank_map[i] = lo + 1;
    }

    memset(count_bit, 0, sizeof(count_bit));
    memset(sum_bit, 0, sizeof(sum_bit));

    for (i = 0; i < N; i++) {
        int r = rank_map[i];
        long long val = seq[i] % MOD;

        for (k = K; k >= 2; k--) {
            long long cnt = bit_sum(count_bit[k-1], r - 1);
            long long sm = bit_sum(sum_bit[k-1], r - 1);
            if (cnt > 0 || sm > 0) {
                bit_add(count_bit[k], r, cnt);
                bit_add(sum_bit[k], r, (cnt % MOD * val % MOD + sm) % MOD);
            }
        }
        bit_add(count_bit[1], r, 1);
        bit_add(sum_bit[1], r, val);
    }

    long long ans = bit_sum(sum_bit[K], N);
    ans = ((ans % MOD) + MOD) % MOD;
    printf("%lld\n", ans);
    return 0;
}
