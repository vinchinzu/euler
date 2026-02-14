/*
 * Project Euler Problem 244: Sliders
 *
 * BFS on 4x4 board states (2-color sliding puzzle) to find checksum
 * of shortest path from initial to target configuration.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define GRID_SIZE 4
#define C 243
#define M 100000007

/*
 * Board state encoding: hole position (4 bits: row*4+col) + 16 bits for grid
 * Hash = hole_i*4 + hole_j, then for each cell: h = 2*h + val
 * Total hash fits in an int (20 bits used at most)
 */

typedef struct {
    int hole_i, hole_j;
    unsigned short grid;  /* 16 bits, one per cell, row-major */
} Board;

static inline int get_cell(unsigned short grid, int r, int c) {
    return (grid >> (r * 4 + c)) & 1;
}

static inline unsigned short set_cell(unsigned short grid, int r, int c, int v) {
    int bit = r * 4 + c;
    grid &= ~(1 << bit);
    grid |= (v << bit);
    return grid;
}

static int board_hash(Board *b) {
    int h = b->hole_i * 4 + b->hole_j;
    for (int r = 0; r < 4; r++)
        for (int c = 0; c < 4; c++)
            h = 2 * h + get_cell(b->grid, r, c);
    return h;
}

/* Hash table for visited states: max hash value ~2^20 = 1M */
#define HTSIZE (1 << 21)
static int visited_hash[HTSIZE];  /* -1 = empty, else stores checksum */
static int visited_key[HTSIZE];

static void ht_init(void) {
    memset(visited_hash, -1, sizeof(visited_hash));
}

static int ht_contains(int key) {
    unsigned idx = (unsigned)key & (HTSIZE - 1);
    while (visited_hash[idx] != -1) {
        if (visited_key[idx] == key) return 1;
        idx = (idx + 1) & (HTSIZE - 1);
    }
    return 0;
}

static void ht_insert(int key, int val) {
    unsigned idx = (unsigned)key & (HTSIZE - 1);
    while (visited_hash[idx] != -1) {
        if (visited_key[idx] == key) return;
        idx = (idx + 1) & (HTSIZE - 1);
    }
    visited_key[idx] = key;
    visited_hash[idx] = val;
}

typedef struct {
    Board board;
    int checksum;
} QueueEntry;

#define QSIZE 2000000
static QueueEntry queue[QSIZE];

int main(void) {
    /* Initial board:
     * 0 1 0 0
     * 1 1 0 0
     * 1 1 0 0
     * 1 1 0 0
     */
    int init_grid_vals[4][4] = {
        {0, 1, 0, 0},
        {1, 1, 0, 0},
        {1, 1, 0, 0},
        {1, 1, 0, 0}
    };

    /* Target board:
     * 0 0 1 0
     * 0 1 0 1
     * 1 0 1 0
     * 0 1 0 1
     */
    int target_grid_vals[4][4] = {
        {0, 0, 1, 0},
        {0, 1, 0, 1},
        {1, 0, 1, 0},
        {0, 1, 0, 1}
    };

    Board start;
    start.hole_i = 0;
    start.hole_j = 0;
    start.grid = 0;
    for (int r = 0; r < 4; r++)
        for (int c = 0; c < 4; c++)
            if (init_grid_vals[r][c])
                start.grid = set_cell(start.grid, r, c, 1);

    Board target;
    target.hole_i = 0;
    target.hole_j = 0;
    target.grid = 0;
    for (int r = 0; r < 4; r++)
        for (int c = 0; c < 4; c++)
            if (target_grid_vals[r][c])
                target.grid = set_cell(target.grid, r, c, 1);

    int target_hash = board_hash(&target);

    int dx[] = {-1, 1, 0, 0};
    int dy[] = {0, 0, -1, 1};
    int keys[] = {'U', 'D', 'L', 'R'};

    ht_init();

    int front = 0, back = 0;
    queue[back].board = start;
    queue[back].checksum = 0;
    back++;

    while (front < back) {
        QueueEntry cur = queue[front++];
        int h = board_hash(&cur.board);

        if (h == target_hash) {
            printf("%d\n", cur.checksum);
            return 0;
        }

        if (ht_contains(h)) continue;
        ht_insert(h, cur.checksum);

        for (int d = 0; d < 4; d++) {
            /* Move piece from (new_i, new_j) into the hole */
            int new_i = cur.board.hole_i - dx[d];
            int new_j = cur.board.hole_j - dy[d];
            if (new_i < 0 || new_i >= 4 || new_j < 0 || new_j >= 4) continue;

            Board nb;
            nb.grid = cur.board.grid;
            int val = get_cell(nb.grid, new_i, new_j);
            nb.grid = set_cell(nb.grid, cur.board.hole_i, cur.board.hole_j, val);
            nb.grid = set_cell(nb.grid, new_i, new_j, 0);
            nb.hole_i = new_i;
            nb.hole_j = new_j;

            int nh = board_hash(&nb);
            if (!ht_contains(nh)) {
                int nc = ((long long)cur.checksum * C + keys[d]) % M;
                queue[back].board = nb;
                queue[back].checksum = nc;
                back++;
            }
        }
    }

    printf("0\n");
    return 0;
}
