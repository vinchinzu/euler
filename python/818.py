"""Project Euler Problem 818: SET.

Find sum of S(C)^4 over all 12-card subsets C of the 81-card SET deck,
where S(C) counts the number of SETs in C.

Uses C for the expensive combinatorial enumeration.
"""

import subprocess, tempfile, os

def solve():
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define B 3
#define D 4
#define NCARDS 81   /* B^D */
#define NN 12
#define KK 4

/* cards[i] is a D-tuple encoded as int */
static int cards[NCARDS][D];
static int num_sets = 0;
static int sets[1200][3];  /* each SET is 3 card indices */

static long long nCr_table[NCARDS+1][NN+1];

static void init_nCr(void) {
    for (int n = 0; n <= NCARDS; n++) {
        nCr_table[n][0] = 1;
        for (int r = 1; r <= NN && r <= n; r++) {
            nCr_table[n][r] = nCr_table[n-1][r-1] + nCr_table[n-1][r];
        }
    }
}

static long long nCr(int n, int r) {
    if (r < 0 || r > n || r > NN) return 0;
    return nCr_table[n][r];
}

static void generate_cards(void) {
    for (int i = 0; i < NCARDS; i++) {
        int v = i;
        for (int d = D - 1; d >= 0; d--) {
            cards[i][d] = v % B;
            v /= B;
        }
    }
}

static int card_index(int c[D]) {
    int idx = 0;
    for (int d = 0; d < D; d++) {
        idx = idx * B + c[d];
    }
    return idx;
}

static void find_sets(void) {
    num_sets = 0;
    for (int i = 0; i < NCARDS; i++) {
        for (int j = i + 1; j < NCARDS; j++) {
            int c3[D];
            for (int d = 0; d < D; d++) {
                c3[d] = ((-(cards[i][d] + cards[j][d])) % B + B) % B;
            }
            int k = card_index(c3);
            if (k > j) {
                sets[num_sets][0] = i;
                sets[num_sets][1] = j;
                sets[num_sets][2] = k;
                num_sets++;
            }
        }
    }
}

static int card_counts[NCARDS];
static long long num_distinct_counts[NN+1];

static void helper(int min_set_idx, int num_remaining, int num_distinct) {
    if (num_remaining == 0) {
        if (num_distinct <= NN)
            num_distinct_counts[num_distinct]++;
        return;
    }
    for (int si = min_set_idx; si < num_sets; si++) {
        int new_distinct = num_distinct;
        for (int c = 0; c < 3; c++) {
            int ci = sets[si][c];
            card_counts[ci]++;
            if (card_counts[ci] == 1) new_distinct++;
        }
        /* Prune: if new_distinct already > NN, remaining cards can't help */
        if (new_distinct <= NN) {
            helper(si + 1, num_remaining - 1, new_distinct);
        }
        for (int c = 0; c < 3; c++) {
            int ci = sets[si][c];
            card_counts[ci]--;
        }
    }
}

static long long parity(int n) {
    return (n % 2 == 0) ? 1 : -1;
}

int main(void) {
    generate_cards();
    find_sets();
    init_nCr();

    /* The first SET is always sets[0] = {0, 1, 2} (cards (0,0,0,0), (0,0,0,1), (0,0,0,2))
       We fix this SET and choose e-1 more from the remaining sets (index >= 1).
       Then multiply by num_sets (any SET could be the fixed one) and divide by e.
    */

    long long ans = 0;

    for (int e = 1; e <= KK; e++) {
        memset(num_distinct_counts, 0, sizeof(num_distinct_counts));
        memset(card_counts, 0, sizeof(card_counts));

        /* Mark the first SET's cards */
        for (int c = 0; c < 3; c++) {
            card_counts[sets[0][c]] = 1;
        }

        helper(1, e - 1, B);  /* B=3 distinct cards from first SET */

        /* Compute number of shapes using inclusion-exclusion:
           Stirling number S(K, e) * e! = sum_{i=0}^{e} (-1)^i * C(e,i) * (e-i)^K */
        long long num_shapes = 0;
        for (int i = 0; i <= e; i++) {
            long long term = parity(i) * nCr(e, i);
            /* (e-i)^K */
            long long pw = 1;
            for (int j = 0; j < KK; j++) pw *= (e - i);
            num_shapes += term * pw;
        }

        for (int k = 0; k <= NN; k++) {
            if (num_distinct_counts[k] == 0) continue;
            ans += (num_shapes / e) * num_sets * nCr(NCARDS - k, NN - k) * num_distinct_counts[k];
        }
    }

    printf("%lld\n", ans);
    return 0;
}
""";

    with tempfile.NamedTemporaryFile(suffix='.c', mode='w', delete=False) as f:
        f.write(c_code)
        c_path = f.name

    bin_path = c_path.replace('.c', '')
    try:
        subprocess.run(['gcc', '-O2', '-o', bin_path, c_path], check=True,
                       capture_output=True, text=True)
        result = subprocess.run([bin_path], capture_output=True, text=True, timeout=30)
        print(result.stdout.strip())
    finally:
        os.unlink(c_path)
        if os.path.exists(bin_path):
            os.unlink(bin_path)

if __name__ == "__main__":
    solve()
