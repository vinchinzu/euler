"""Project Euler Problem 733: Ascending subsequences.

Find S(10^6) mod 10^9+7, where S(n) is the sum of all elements in all
ascending 4-term subsequences of the sequence a_i = 153^i mod 10000019.

Uses BIT (Fenwick tree) with coordinate compression. For each element,
maintain count and sum of ascending subsequences of length 1..4 ending
at values < current. Implemented in C for performance.
"""

from __future__ import annotations

import subprocess
import tempfile
import os


def solve() -> int:
    """Solve Problem 733 using compiled C."""
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAXN 1000001
#define MOD 1000000007LL
#define K 4

static long long seq[MAXN];
static int sorted_vals[MAXN];
static int rank_map[MAXN]; /* rank of seq[i] after coord compression */

/* BIT arrays: count_bit[k][i] and sum_bit[k][i] for subsequences of length k */
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

    /* Generate sequence */
    long long a = 153;
    for (i = 0; i < N; i++) {
        seq[i] = a;
        sorted_vals[i] = (int)a;
        a = (a * 153) % 10000019LL;
    }

    /* Coordinate compression */
    qsort(sorted_vals, N, sizeof(int), cmp_int);
    /* Remove duplicates and assign ranks */
    int unique = 0;
    for (i = 0; i < N; i++) {
        if (i == 0 || sorted_vals[i] != sorted_vals[i-1])
            sorted_vals[unique++] = sorted_vals[i];
    }

    /* For each seq[i], find its rank via binary search */
    for (i = 0; i < N; i++) {
        int lo = 0, hi = unique - 1, target = (int)seq[i];
        while (lo < hi) {
            int mid = (lo + hi) / 2;
            if (sorted_vals[mid] < target) lo = mid + 1;
            else hi = mid;
        }
        rank_map[i] = lo + 1;  /* 1-indexed */
    }

    /* Initialize BIT arrays to 0 */
    memset(count_bit, 0, sizeof(count_bit));
    memset(sum_bit, 0, sizeof(sum_bit));

    /* Process each element */
    for (i = 0; i < N; i++) {
        int r = rank_map[i];
        long long val = seq[i] % MOD;

        /* For length k = K down to 2, update using length k-1 */
        for (k = K; k >= 2; k--) {
            long long cnt = bit_sum(count_bit[k-1], r - 1);
            long long sm = bit_sum(sum_bit[k-1], r - 1);
            if (cnt > 0 || sm > 0) {
                bit_add(count_bit[k], r, cnt);
                bit_add(sum_bit[k], r, (cnt % MOD * val % MOD + sm) % MOD);
            }
        }
        /* Length 1 */
        bit_add(count_bit[1], r, 1);
        bit_add(sum_bit[1], r, val);
    }

    long long ans = bit_sum(sum_bit[K], N);
    ans = ((ans % MOD) + MOD) % MOD;
    printf("%lld\n", ans);
    return 0;
}
"""
    with tempfile.NamedTemporaryFile(suffix='.c', mode='w', delete=False) as f:
        f.write(c_code)
        c_path = f.name

    bin_path = c_path.replace('.c', '')
    try:
        subprocess.run(['gcc', '-O2', '-o', bin_path, c_path, '-lm'],
                       check=True, capture_output=True)
        result = subprocess.run([bin_path], capture_output=True, text=True, check=True)
        return int(result.stdout.strip())
    finally:
        os.unlink(c_path)
        if os.path.exists(bin_path):
            os.unlink(bin_path)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
