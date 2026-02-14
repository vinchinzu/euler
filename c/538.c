/*
 * Project Euler 538 - Maximum Quadrilateral
 *
 * u_n = 2^B(3n) + 3^B(2n) + B(n+1), where B(n) = popcount(n).
 * f(U_n) = perimeter of cyclic quadrilateral with max area using 4
 * consecutive values from sorted set U_n. Uses Brahmagupta's formula.
 *
 * Maintain a sorted set of u values, track the best 4 consecutive.
 */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>

typedef long long ll;

/* We use a sorted array with binary search insertion.
 * N = 3*10^6 elements max. Insertion into sorted array is O(n) per step,
 * which is too slow for 3M elements.
 *
 * Instead, we use a balanced BST via a skip list or a simpler approach:
 * Since u values are bounded, we can use a Fenwick tree approach.
 *
 * Actually, let's think about what values u can take.
 * B(n) <= 20 (for n up to ~10^7), so:
 * 2^B(3n) <= 2^20 = 1048576
 * 3^B(2n) <= 3^20 ~ 3.5 * 10^9 -- too big for simple array
 *
 * But B(2n) = B(n) (since 2n just shifts bits), so 3^B(n).
 * B(n) for n up to 3*10^6 is at most ~22, so 3^22 ~ 3.1*10^10.
 * u can be up to ~3.1*10^10.
 *
 * Alternative approach: maintain a dynamically sorted array and track
 * only the "frontier" of best 4 consecutive values.
 *
 * Key insight from the Python code: when a new u is added, only check
 * candidates near u in the sorted order. The best quad uses 4 consecutive
 * sorted values (since Brahmagupta's formula is maximized when sides are
 * as close as possible).
 *
 * We can use a sorted array with binary search for position finding,
 * but we need efficient insertion. Use a skip list or just accept O(n)
 * per insertion since the constant is small for memmove.
 *
 * Actually 3M * 3M = 9*10^12 operations for naive -- too slow.
 * Let's use a balanced BST via a treap or red-black tree.
 *
 * Simpler: use an array + binary search. The memmove is actually fast
 * for this problem because the array fits in cache and memmove is
 * hardware-optimized. 3M * avg_shift ~ 3M * 1.5M = 4.5*10^12 bytes
 * moved -- that's too much.
 *
 * Better approach: use a linked list with skip pointers, or bucket sort.
 *
 * Actually let's look at the Python more carefully. It uses SortedList
 * which is O(sqrt(n)) per operation. In C, we can implement a simple
 * sqrt-decomposition sorted list.
 *
 * But even simpler: the key observation is we only need to track 4
 * consecutive values in sorted order. We can maintain a sorted array
 * and do binary search + local check. The insertion can use a gap
 * buffer or just realloc approach.
 *
 * Let me try a different approach: since we need the answer = sum of
 * best perimeters, and the best quad uses 4 consecutive values in
 * sorted order, we can maintain a "window" of the top few values.
 *
 * Actually, re-reading the Python: it specifically looks at candidates
 * near the newly inserted value. The best 4 consecutive values maximize
 * Brahmagupta's formula. For a cyclic quad, the area is maximized when
 * the 4 sides are as equal as possible.
 *
 * The top 4 values (largest) would give the largest perimeter, and if
 * they're close together, also the largest area. Let me check: for 4
 * consecutive sorted values a <= b <= c <= d, the Brahmagupta area^2
 * = (a+b+c-d)(a+b-c+d)(a-b+c+d)(-a+b+c+d)/16. This is maximized
 * when d < a+b+c (triangle inequality for cyclic quad). The perimeter
 * is always a+b+c+d. But the QUESTION asks for the perimeter of the
 * quad with LARGEST AREA.
 *
 * So the approach is: maintain sorted values, and among all groups of
 * 4 consecutive values, find the one maximizing Brahmagupta's formula,
 * and report its perimeter.
 *
 * Since we can only improve the best by adding a new value near the
 * current best, we check locally.
 *
 * Let me implement with a simple sorted array + memmove, but optimize
 * by noting we only need to check near the insertion point.
 * For 3M elements, each insertion moves on average 1.5M longs = 12MB.
 * 3M * 12MB = 36TB -- way too slow.
 *
 * So we need an O(log n) sorted set. Let me implement a red-black tree
 * or use a simpler structure.
 *
 * Actually, let's use a sorted array with block decomposition (sqrt
 * decomposition). Block size = sqrt(N) ~ 1732. Each block is a sorted
 * array. Insertion: find block, insert into block. If block too large,
 * split. Finding k-th neighbor: traverse blocks.
 */

#define MAXN 3000001
#define BLOCK_SIZE 2000
#define MAX_BLOCKS 3000

static ll blocks[MAX_BLOCKS][BLOCK_SIZE * 2]; /* each block sorted */
static int block_len[MAX_BLOCKS];
static int nblocks;
static int total_elements;

static void init_blocks(void) {
    nblocks = 1;
    block_len[0] = 0;
    total_elements = 0;
}

/* Binary search in a sorted block for insertion position */
static int bsearch_block(ll *arr, int len, ll val) {
    int lo = 0, hi = len;
    while (lo < hi) {
        int mid = (lo + hi) / 2;
        if (arr[mid] < val) lo = mid + 1;
        else hi = mid;
    }
    return lo;
}

/* Insert val into the sorted structure. Returns (block_idx, pos_in_block) */
static void sorted_insert(ll val, int *ret_block, int *ret_pos) {
    /* Find which block to insert into */
    int bi = 0;
    for (int b = 0; b < nblocks; b++) {
        if (block_len[b] > 0 && blocks[b][block_len[b] - 1] < val) {
            bi = b + 1;
        } else {
            bi = b;
            break;
        }
    }
    if (bi >= nblocks) bi = nblocks - 1;

    int pos = bsearch_block(blocks[bi], block_len[bi], val);

    /* Insert into block */
    memmove(&blocks[bi][pos + 1], &blocks[bi][pos],
            (block_len[bi] - pos) * sizeof(ll));
    blocks[bi][pos] = val;
    block_len[bi]++;
    total_elements++;

    *ret_block = bi;
    *ret_pos = pos;

    /* Split if too large */
    if (block_len[bi] > BLOCK_SIZE * 3 / 2) {
        /* Insert new block after bi */
        if (nblocks >= MAX_BLOCKS) return; /* shouldn't happen */
        memmove(&block_len[bi + 2], &block_len[bi + 1],
                (nblocks - bi - 1) * sizeof(int));
        /* Move blocks */
        for (int b = nblocks; b > bi + 1; b--) {
            memcpy(blocks[b], blocks[b - 1], block_len[b - 1] * sizeof(ll));
            block_len[b] = block_len[b - 1];
        }
        nblocks++;

        int half = block_len[bi] / 2;
        memcpy(blocks[bi + 1], &blocks[bi][half],
               (block_len[bi] - half) * sizeof(ll));
        block_len[bi + 1] = block_len[bi] - half;
        block_len[bi] = half;
    }
}

/*
 * Get the global index of element at (block, pos).
 * And get elements at global indices around a given position.
 */
static int global_index(int block, int pos) {
    int idx = pos;
    for (int b = 0; b < block; b++) idx += block_len[b];
    return idx;
}

/* Get element at global index idx */
static ll get_at(int idx) {
    for (int b = 0; b < nblocks; b++) {
        if (idx < block_len[b]) return blocks[b][idx];
        idx -= block_len[b];
    }
    return -1; /* shouldn't happen */
}

static int popcount_ll(ll n) {
    return __builtin_popcountll(n);
}

static ll ipow(ll base, int exp) {
    ll result = 1;
    for (int i = 0; i < exp; i++) result *= base;
    return result;
}

int main(void) {
    int N = 3000000;
    int K = 4;

    init_blocks();

    double max_area2 = 0.0;
    ll best_perim = 0;
    ll best_min_side = 0;
    ll ans = 0;

    for (int n = 1; n <= N; n++) {
        int b3n = popcount_ll((ll)3 * n);
        int b2n = popcount_ll((ll)2 * n);
        int bn1 = popcount_ll((ll)(n + 1));
        ll u = ipow(2, b3n) + ipow(3, b2n) + bn1;

        int ins_block, ins_pos;
        sorted_insert(u, &ins_block, &ins_pos);

        if (total_elements < K) {
            ans += best_perim;
            continue;
        }

        if (u >= best_min_side) {
            /* Get global index of inserted element */
            int gidx = global_index(ins_block, ins_pos);

            /* Check windows of K consecutive around gidx */
            int start = gidx - (K - 1);
            if (start < 0) start = 0;
            int end = gidx;
            if (end + K - 1 >= total_elements) end = total_elements - K;
            if (end < start) end = start;

            for (int i = start; i <= end; i++) {
                if (i + K - 1 >= total_elements) break;
                ll sides[4];
                for (int j = 0; j < K; j++) sides[j] = get_at(i + j);
                ll perim = sides[0] + sides[1] + sides[2] + sides[3];

                double area2 = 1.0;
                for (int j = 0; j < K; j++)
                    area2 *= (double)(perim - 2 * sides[j]);

                if (area2 >= max_area2) {
                    max_area2 = area2;
                    best_perim = perim;
                    best_min_side = sides[0];
                }
            }
        }

        ans += best_perim;
    }

    printf("%lld\n", ans);
    return 0;
}
