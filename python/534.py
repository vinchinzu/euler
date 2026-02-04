"""Project Euler Problem 534: Weak Queens.

Let Q(n, w) be the number of ways to arrange n queens on n x n board where each
queen can move at most n-1-w squares vertically or diagonally.
Find sum of Q(14, w) for w = 0 to 13.

For each weakness level w (attack range k = N-1-w, from 0 to N-1):
- k=0: no attacks, answer is N^N
- k=1..8: profile DP with precomputed transitions between valid k-tuples
- k=9..13: DFS with bitmask pruning
"""

import subprocess, tempfile, os

def solve():
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef unsigned long long ull;
#define N 14

/* ============ Bitmask-accelerated DFS for large k ============ */
static ll dfs_count;
static int dfs_cols[N];
static int dfs_k;

void dfs_fast(int row) {
    if (row == N) {
        dfs_count++;
        return;
    }
    int blocked = 0;
    int start = row - dfs_k;
    if (start < 0) start = 0;
    for (int r = start; r < row; r++) {
        int d = row - r;
        int c = dfs_cols[r];
        blocked |= (1 << c);
        if (c + d < N) blocked |= (1 << (c + d));
        if (c - d >= 0) blocked |= (1 << (c - d));
    }
    int avail = ((1 << N) - 1) & ~blocked;
    while (avail) {
        int bit = avail & (-avail);
        avail &= avail - 1;
        dfs_cols[row] = __builtin_ctz(bit);
        dfs_fast(row + 1);
    }
}

/* ============ Profile DP for small/medium k ============ */
#define MAX_CONFIGS 2500000
#define MAX_TRANS 10000000

static int *config_store;
static ll *dp_count, *dp_count2;
static int nconfigs;
static int tmp_cols_dp[N];

static int check_valid(int *cols, int pos, int c) {
    for (int j = 0; j < pos; j++) {
        int dr = pos - j;
        int dc = c - cols[j];
        if (dc == 0 || dc == dr || dc == -dr) return 0;
    }
    return 1;
}

static void gen_configs(int k, int pos) {
    if (pos == k) {
        memcpy(config_store + nconfigs * N, tmp_cols_dp, k * sizeof(int));
        nconfigs++;
        return;
    }
    for (int c = 0; c < N; c++) {
        if (check_valid(tmp_cols_dp, pos, c)) {
            tmp_cols_dp[pos] = c;
            gen_configs(k, pos + 1);
        }
    }
}

#define HT_SIZE (1 << 23)
#define HT_MASK (HT_SIZE - 1)

typedef struct { ull key; int idx; int occupied; } HTEntry;
static HTEntry *ht;

static ull encode_config(int *cols, int len) {
    ull key = 0, base = 1;
    for (int i = 0; i < len; i++) { key += cols[i] * base; base *= N; }
    return key;
}

static ull ht_hash(ull key) {
    key ^= key >> 33; key *= 0xff51afd7ed558ccdULL;
    key ^= key >> 33; key *= 0xc4ceb9fe1a85ec53ULL;
    key ^= key >> 33; return key;
}

static void ht_clear(void) { memset(ht, 0, HT_SIZE * sizeof(HTEntry)); }

static void ht_insert(ull key, int idx) {
    ull h = ht_hash(key) & HT_MASK;
    while (ht[h].occupied) h = (h + 1) & HT_MASK;
    ht[h].key = key; ht[h].idx = idx; ht[h].occupied = 1;
}

static int ht_lookup(ull key) {
    ull h = ht_hash(key) & HT_MASK;
    while (ht[h].occupied) {
        if (ht[h].key == key) return ht[h].idx;
        h = (h + 1) & HT_MASK;
    }
    return -1;
}

static int *trans_target;
static int *trans_offset;
static int ntrans;

static ll solve_dp(int k) {
    if (k == 0) { ll r = 1; for (int i = 0; i < N; i++) r *= N; return r; }

    nconfigs = 0;
    gen_configs(k, 0);

    ht_clear();
    for (int i = 0; i < nconfigs; i++)
        ht_insert(encode_config(config_store + i * N, k), i);

    ntrans = 0;
    for (int i = 0; i < nconfigs; i++) {
        trans_offset[i] = ntrans;
        int *cols = config_store + i * N;
        for (int c = 0; c < N; c++) {
            int ok = 1;
            for (int j = 0; j < k; j++) {
                int dr = k - j;
                int dc = c - cols[j];
                if (dc == 0 || dc == dr || dc == -dr) { ok = 0; break; }
            }
            if (ok) {
                int new_cols[N];
                for (int j = 1; j < k; j++) new_cols[j-1] = cols[j];
                new_cols[k-1] = c;
                int target = ht_lookup(encode_config(new_cols, k));
                if (target >= 0) trans_target[ntrans++] = target;
            }
        }
    }
    trans_offset[nconfigs] = ntrans;

    for (int i = 0; i < nconfigs; i++) dp_count[i] = 1;

    for (int row = k; row < N; row++) {
        memset(dp_count2, 0, nconfigs * sizeof(ll));
        for (int i = 0; i < nconfigs; i++) {
            if (dp_count[i] == 0) continue;
            for (int t = trans_offset[i]; t < trans_offset[i+1]; t++)
                dp_count2[trans_target[t]] += dp_count[i];
        }
        memcpy(dp_count, dp_count2, nconfigs * sizeof(ll));
    }

    ll total = 0;
    for (int i = 0; i < nconfigs; i++) total += dp_count[i];
    return total;
}

int main(void) {
    config_store = (int*)malloc(MAX_CONFIGS * N * sizeof(int));
    dp_count = (ll*)calloc(MAX_CONFIGS, sizeof(ll));
    dp_count2 = (ll*)calloc(MAX_CONFIGS, sizeof(ll));
    trans_target = (int*)malloc(MAX_TRANS * sizeof(int));
    trans_offset = (int*)malloc((MAX_CONFIGS + 1) * sizeof(int));
    ht = (HTEntry*)calloc(HT_SIZE, sizeof(HTEntry));

    ll ans = 0;
    for (int k = 0; k < N; k++) {
        ll q;
        if (k <= 8) {
            q = solve_dp(k);
        } else {
            dfs_k = k;
            dfs_count = 0;
            dfs_fast(0);
            q = dfs_count;
        }
        ans += q;
    }
    printf("%lld\n", ans);

    free(config_store); free(dp_count); free(dp_count2);
    free(trans_target); free(trans_offset); free(ht);
    return 0;
}
"""
    with tempfile.NamedTemporaryFile(suffix='.c', mode='w', delete=False) as src:
        src.write(c_code)
        src_path = src.name
    bin_path = src_path.replace('.c', '')
    try:
        subprocess.run(['gcc', '-O3', '-o', bin_path, src_path, '-lm'], check=True,
                       capture_output=True, text=True)
        result = subprocess.run([bin_path], capture_output=True, text=True, check=True, timeout=30)
        print(result.stdout.strip())
    except subprocess.CalledProcessError as e:
        print(f"Error: {e.stderr}", flush=True)
        raise
    finally:
        os.unlink(src_path)
        if os.path.exists(bin_path):
            os.unlink(bin_path)

if __name__ == "__main__":
    solve()
