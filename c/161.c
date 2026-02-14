/*
 * Project Euler Problem 161: Triominoes
 *
 * Count the number of ways to tile a 9x12 grid with L-trominoes and I-trominoes.
 * Uses profile-based DP: process columns left to right, tracking which cells in
 * the current and next columns are already filled.
 *
 * Alternative approach: backtracking with bitmask memoization on the grid state,
 * but we use a hash map for the DP states since the state space is sparse.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

#define WIDTH 9
#define HEIGHT 12

/* Hash map for memoization: key = grid bitmask (up to 108 bits, but we use
 * row-by-row processing so we only need states for a few rows at a time).
 *
 * Actually, let's use the same approach as Python: full grid bitmask with
 * backtracking + memoization via hash table.
 * Grid is 9x12 = 108 bits, but we only track the "profile" of filled cells
 * near the frontier. Let's try a different approach: process row by row.
 *
 * Actually, the Python solution uses full 108-bit grid keys which is too large
 * for C hash tables. Let's use a column-profile DP instead.
 *
 * Better approach: scan cells left-to-right, top-to-bottom. Track a "profile"
 * of the next WIDTH*3 cells (since trominoes span at most 3 rows and 3 cols
 * from the current position). But that's still complex.
 *
 * Simplest correct approach: backtracking with memoization on column profiles.
 * Process one column at a time. State = bitmask of filled cells in current
 * column spillover (HEIGHT bits).
 *
 * Actually, let's just do what the Python does but more efficiently:
 * backtracking with memoization. The key insight is that when we always fill
 * the first empty cell, the relevant state is just the grid configuration.
 * With 108 cells, we can't store all states, but in practice the number of
 * reachable states is manageable.
 *
 * For efficiency, let's use a different well-known approach for this problem:
 * profile dynamic programming on rows.
 */

/*
 * Profile DP approach:
 * Process the grid row by row, left to right within each row.
 * State = a bitmask of WIDTH bits for the current row (which cells are filled)
 *       + a bitmask of WIDTH bits for the next row (spillover from pieces placed)
 *       + a bitmask of WIDTH bits for the row after next (spillover from vertical pieces)
 *
 * But trominoes can extend up to 2 rows down, so we need 2 rows of lookahead.
 * State: (current_row_mask, next_row_mask, next_next_row_mask) when placing in current row.
 *
 * Actually, let's do cell-by-cell processing with profile DP.
 * State = bitmask showing which of the next W cells (in reading order) are already filled.
 * Since trominoes cover at most 3 rows and 2 columns (or 2 rows and 3 columns),
 * the maximum lookahead is about 2*W + 3 cells. With W=9, that's ~21 bits = 2M states.
 * That's feasible.
 *
 * Wait, let me reconsider. The standard approach for small-width tiling:
 * Process column by column. State = which cells in the current column boundary are filled.
 * With HEIGHT=12, state is 12 bits = 4096 states per column.
 *
 * For each column transition, we enumerate all ways to place trominoes that
 * complete the column and may extend into the next column.
 *
 * Let me implement the backtracking approach with memoization, similar to Python,
 * but using a hash table. The state is the full 108-bit grid, but we represent it
 * as two 64-bit integers.
 */

/* Use a hash map with 128-bit keys */
#define HASH_SIZE (1 << 22)
#define HASH_MASK (HASH_SIZE - 1)

typedef unsigned long long u64;

typedef struct Entry {
    u64 key_lo, key_hi;
    long long value;
    int used;
} Entry;

static Entry htable[HASH_SIZE];
static int ht_count = 0;

static unsigned int hash128(u64 lo, u64 hi) {
    u64 h = lo * 0x9E3779B97F4A7C15ULL + hi * 0x517CC1B727220A95ULL;
    h ^= h >> 33;
    h *= 0xFF51AFD7ED558CCDULL;
    h ^= h >> 33;
    return (unsigned int)(h & HASH_MASK);
}

static long long ht_get(u64 lo, u64 hi) {
    unsigned int idx = hash128(lo, hi);
    while (1) {
        if (!htable[idx].used) return -1;
        if (htable[idx].key_lo == lo && htable[idx].key_hi == hi)
            return htable[idx].value;
        idx = (idx + 1) & HASH_MASK;
    }
}

static void ht_set(u64 lo, u64 hi, long long val) {
    unsigned int idx = hash128(lo, hi);
    while (1) {
        if (!htable[idx].used) {
            htable[idx].key_lo = lo;
            htable[idx].key_hi = hi;
            htable[idx].value = val;
            htable[idx].used = 1;
            ht_count++;
            return;
        }
        if (htable[idx].key_lo == lo && htable[idx].key_hi == hi) {
            htable[idx].value = val;
            return;
        }
        idx = (idx + 1) & HASH_MASK;
    }
}

/* Grid is HEIGHT rows x WIDTH cols = 12 x 9 = 108 bits
 * Bit (r * WIDTH + c) represents cell (r, c)
 * We store as two u64: lo = bits 0-63, hi = bits 64-107 */

static inline int get_bit(u64 lo, u64 hi, int pos) {
    if (pos < 64) return (int)((lo >> pos) & 1);
    return (int)((hi >> (pos - 64)) & 1);
}

static inline void set_bit(u64 *lo, u64 *hi, int pos) {
    if (pos < 64) *lo |= (1ULL << pos);
    else *hi |= (1ULL << (pos - 64));
}

static inline void clear_bit(u64 *lo, u64 *hi, int pos) {
    if (pos < 64) *lo &= ~(1ULL << pos);
    else *hi &= ~(1ULL << (pos - 64));
}

/* Tromino shapes: 6 shapes, each with 3 cells as (dr, dc) offsets */
static const int shapes[6][3][2] = {
    {{0,0}, {0,1}, {0,2}},    /* I horizontal */
    {{0,0}, {1,0}, {2,0}},    /* I vertical */
    {{0,0}, {0,1}, {1,0}},    /* L shape 1 */
    {{0,0}, {1,0}, {1,1}},    /* L shape 2 */
    {{0,0}, {0,1}, {1,1}},    /* L shape 3 */
    {{0,0}, {1,0}, {1,-1}},   /* L shape 4 */
};

static long long backtrack(u64 lo, u64 hi) {
    long long cached = ht_get(lo, hi);
    if (cached >= 0) return cached;

    /* Find first empty cell */
    int pos = -1;
    for (int i = 0; i < WIDTH * HEIGHT; i++) {
        if (!get_bit(lo, hi, i)) {
            pos = i;
            break;
        }
    }

    if (pos < 0) {
        /* All cells filled */
        ht_set(lo, hi, 1);
        return 1;
    }

    int r = pos / WIDTH;
    int c = pos % WIDTH;

    long long count = 0;

    for (int s = 0; s < 6; s++) {
        /* Check if shape fits */
        int ok = 1;
        int positions[3];
        for (int k = 0; k < 3; k++) {
            int nr = r + shapes[s][k][0];
            int nc = c + shapes[s][k][1];
            if (nr < 0 || nr >= HEIGHT || nc < 0 || nc >= WIDTH) {
                ok = 0;
                break;
            }
            int p = nr * WIDTH + nc;
            if (get_bit(lo, hi, p)) {
                ok = 0;
                break;
            }
            positions[k] = p;
        }
        if (!ok) continue;

        /* Place shape */
        u64 nlo = lo, nhi = hi;
        for (int k = 0; k < 3; k++)
            set_bit(&nlo, &nhi, positions[k]);

        count += backtrack(nlo, nhi);
    }

    ht_set(lo, hi, count);
    return count;
}

int main(void) {
    memset(htable, 0, sizeof(htable));
    long long result = backtrack(0, 0);
    printf("%lld\n", result);
    return 0;
}
