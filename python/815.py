"""Project Euler Problem 815: Grouping Cards.

K=4 copies of each of N=60 values. Deal one at a time, remove all K copies
when complete. Find expected max distinct values present at any time.

Ported to C for performance.
"""

import subprocess
import tempfile
import os


def solve():
    c_code = r"""
#include <stdio.h>
#include <string.h>

#define N 60
#define K 4
// counts: c[0]..c[K], where c[i] = number of values with exactly i copies dealt
// c[K] = completed values (removed). Sum c[0]+...+c[K] = N.
// The "distinct values present" = N - c[0] - c[K] = c[1]+c[2]+c[3]
// max_val tracks the running maximum of this quantity.

// State indexing: we need to index (c[0], c[1], c[2], c[3]) since c[4] = N - sum.
// Index using combinatorial number system.

// nCr table
static long long nCr_table[N + K + 1][K + 1];

void precompute_nCr() {
    for (int n = 0; n <= N + K; n++) {
        nCr_table[n][0] = 1;
        for (int r = 1; r <= K && r <= n; r++) {
            nCr_table[n][r] = nCr_table[n-1][r-1] + nCr_table[n-1][r];
        }
    }
}

// Convert counts c[0..K-1] (c[K] derived) to index in range [0, C(N+K,K))
int index_from_counts(int c[]) {
    int total = -1;
    int idx = 0;
    for (int i = 0; i < K; i++) {
        total += c[i] + 1;
        if (total >= i + 1) {
            idx += nCr_table[total][i + 1];
        }
    }
    return idx;
}

// Cache: indexed by (counts_index, max_val)
// max_index = C(N+K, K), max_val in [0, N]
#define MAX_INDEX 635376  // C(64,4) = 635376
#define MAX_VAL (N + 1)

static double cache[MAX_INDEX][MAX_VAL];
static char computed[MAX_INDEX][MAX_VAL];

double solve_recursive(int c[], int max_val) {
    int idx = index_from_counts(c);
    if (computed[idx][max_val]) return cache[idx][max_val];

    int remaining = 0;
    for (int i = 0; i < K; i++) {
        remaining += (K - i) * c[i];
    }
    if (remaining == 0) {
        computed[idx][max_val] = 1;
        cache[idx][max_val] = (double)max_val;
        return (double)max_val;
    }

    double result = 0.0;
    for (int t = 0; t < K; t++) {
        if (c[t] > 0) {
            int count = c[t];
            c[t]--;
            c[t + 1]++;
            int distinct = N - c[0] - c[K];
            int new_max = max_val > distinct ? max_val : distinct;
            result += solve_recursive(c, new_max) * (K - t) * count;
            c[t]++;
            c[t + 1]--;
        }
    }

    computed[idx][max_val] = 1;
    cache[idx][max_val] = result / remaining;
    return cache[idx][max_val];
}

int main() {
    precompute_nCr();
    memset(computed, 0, sizeof(computed));

    int c[K + 1];
    c[0] = N;
    for (int i = 1; i <= K; i++) c[i] = 0;

    double result = solve_recursive(c, 0);
    printf("%.8f\n", result);
    return 0;
}
"""

    with tempfile.NamedTemporaryFile(mode='w', suffix='.c', delete=False) as f:
        f.write(c_code)
        c_path = f.name

    bin_path = c_path.replace('.c', '')
    try:
        subprocess.run(['gcc', '-O2', '-o', bin_path, c_path, '-lm'],
                      check=True, capture_output=True)
        result = subprocess.run([bin_path], capture_output=True, text=True, check=True,
                              timeout=30)
        return result.stdout.strip()
    finally:
        os.unlink(c_path)
        if os.path.exists(bin_path):
            os.unlink(bin_path)


def main():
    result = solve()
    print(result)


if __name__ == "__main__":
    main()
