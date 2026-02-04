"""Project Euler Problem 766: Sliding Block Puzzle.

Uses C extension compiled on-the-fly for performance.
DFS exploring all reachable configurations of a sliding block puzzle.
Pieces with the same shape (up to translation) are indistinguishable.
"""

import os
import subprocess
import sys
import tempfile


C_CODE = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define H 5
#define W 6
#define NCELLS 30
#define MAX_PIECES 14
#define HASH_SIZE (1 << 23)
#define HASH_MASK (HASH_SIZE - 1)
#define MAX_STATES 3000000
#define MAX_STACK 3000000

static const char* GRID_STR[] = {
    ".AABCC",
    ".ABBCD",
    "EFGGHD",
    "IJGGHK",
    "LMNNKK",
};

/* piece data */
static int npieces;
static int piece_ncells[MAX_PIECES];
static int piece_cells[MAX_PIECES][4]; /* flattened cell indices (y*W+x) */
static int piece_type[MAX_PIECES];     /* shape type id (1-based) */

/* grid state: grid[y*W+x] = piece_id (1-based) or 0 for empty */
static unsigned char pgrid[NCELLS];
/* type grid: tgrid[y*W+x] = piece_type or 0 for empty */
static unsigned char tgrid[NCELLS];
/* piece offsets from original position */
static int poff_y[MAX_PIECES];
static int poff_x[MAX_PIECES];

/* hash table for visited states (stores full tgrid for exact comparison) */
static unsigned char (*hash_keys)[NCELLS];
static int *hash_next;
static int *hash_buckets;
static int hash_count = 0;

static unsigned int compute_hash(void) {
    unsigned int h = 0;
    for (int i = 0; i < NCELLS; i++)
        h = h * 31u + tgrid[i];
    return h;
}

static int hash_insert(void) {
    unsigned int h = compute_hash();
    int bucket = h & HASH_MASK;
    for (int idx = hash_buckets[bucket]; idx != -1; idx = hash_next[idx])
        if (memcmp(hash_keys[idx], tgrid, NCELLS) == 0)
            return 0; /* already exists */
    int idx = hash_count++;
    if (idx >= MAX_STATES) { fprintf(stderr, "MAX_STATES exceeded\n"); exit(1); }
    memcpy(hash_keys[idx], tgrid, NCELLS);
    hash_next[idx] = hash_buckets[bucket];
    hash_buckets[bucket] = idx;
    return 1; /* newly inserted */
}

static const int DY[] = {-1, 1, 0, 0};
static const int DX[] = {0, 0, -1, 1};

/* Stack frame for iterative DFS */
typedef struct {
    int ti;      /* piece index being tried */
    int di;      /* direction index being tried */
    int moved;   /* 1 if we made a move that needs undoing */
} Frame;

static Frame stack[MAX_STACK];
static int stack_top;

/* Remove piece ti from grids */
static void remove_piece(int ti) {
    int nc = piece_ncells[ti];
    int oy = poff_y[ti], ox = poff_x[ti];
    for (int j = 0; j < nc; j++) {
        int orig = piece_cells[ti][j];
        int r = orig / W + oy, c = orig % W + ox;
        pgrid[r * W + c] = 0;
        tgrid[r * W + c] = 0;
    }
}

/* Place piece ti on grids */
static void place_piece(int ti) {
    int nc = piece_ncells[ti];
    int oy = poff_y[ti], ox = poff_x[ti];
    int pid = ti + 1;
    int tid = piece_type[ti];
    for (int j = 0; j < nc; j++) {
        int orig = piece_cells[ti][j];
        int r = orig / W + oy, c = orig % W + ox;
        pgrid[r * W + c] = pid;
        tgrid[r * W + c] = tid;
    }
}

/* Check if piece ti can move in direction di */
static int can_move(int ti, int di) {
    int nc = piece_ncells[ti];
    int oy = poff_y[ti], ox = poff_x[ti];
    int dy = DY[di], dx = DX[di];
    int pid = ti + 1;
    for (int j = 0; j < nc; j++) {
        int orig = piece_cells[ti][j];
        int nr = orig / W + oy + dy;
        int nc2 = orig % W + ox + dx;
        if (nr < 0 || nr >= H || nc2 < 0 || nc2 >= W) return 0;
        int v = pgrid[nr * W + nc2];
        if (v != 0 && v != pid) return 0;
    }
    return 1;
}

int main(void) {
    hash_keys = calloc(MAX_STATES, NCELLS);
    hash_next = malloc(MAX_STATES * sizeof(int));
    hash_buckets = malloc(HASH_SIZE * sizeof(int));
    if (!hash_keys || !hash_next || !hash_buckets) { fprintf(stderr, "alloc failed\n"); return 1; }
    memset(hash_buckets, -1, HASH_SIZE * sizeof(int));

    /* Parse grid */
    int cell_count[26] = {0};
    int cell_pos[26][4]; /* flattened positions */
    for (int y = 0; y < H; y++)
        for (int x = 0; x < W; x++) {
            char ch = GRID_STR[y][x];
            if (ch != '.') {
                int ci = ch - 'A';
                cell_pos[ci][cell_count[ci]++] = y * W + x;
            }
        }

    /* Identify pieces and their shape types */
    npieces = 0;
    int shape_count = 0;
    int shape_n[MAX_PIECES];
    int shape_dy[MAX_PIECES][4];
    int shape_dx[MAX_PIECES][4];

    for (int ci = 0; ci < 26; ci++) {
        if (cell_count[ci] == 0) continue;
        int pi = npieces++;
        int nc = cell_count[ci];
        piece_ncells[pi] = nc;
        for (int j = 0; j < nc; j++)
            piece_cells[pi][j] = cell_pos[ci][j];

        /* Compute relative shape (normalize to top-left corner) */
        int min_y = 99, min_x = 99;
        for (int j = 0; j < nc; j++) {
            int r = cell_pos[ci][j] / W;
            int c = cell_pos[ci][j] % W;
            if (r < min_y || (r == min_y && c < min_x)) {
                min_y = r; min_x = c;
            }
        }
        int rdy[4], rdx[4];
        for (int j = 0; j < nc; j++) {
            rdy[j] = cell_pos[ci][j] / W - min_y;
            rdx[j] = cell_pos[ci][j] % W - min_x;
        }
        /* Sort by (dy, dx) */
        for (int a = 0; a < nc - 1; a++)
            for (int b = a + 1; b < nc; b++)
                if (rdy[a] > rdy[b] || (rdy[a] == rdy[b] && rdx[a] > rdx[b])) {
                    int t;
                    t = rdy[a]; rdy[a] = rdy[b]; rdy[b] = t;
                    t = rdx[a]; rdx[a] = rdx[b]; rdx[b] = t;
                }

        /* Find matching shape or create new */
        int found = -1;
        for (int s = 0; s < shape_count; s++) {
            if (shape_n[s] != nc) continue;
            int match = 1;
            for (int j = 0; j < nc; j++)
                if (shape_dy[s][j] != rdy[j] || shape_dx[s][j] != rdx[j]) { match = 0; break; }
            if (match) { found = s; break; }
        }
        if (found == -1) {
            found = shape_count;
            shape_n[shape_count] = nc;
            for (int j = 0; j < nc; j++) {
                shape_dy[shape_count][j] = rdy[j];
                shape_dx[shape_count][j] = rdx[j];
            }
            shape_count++;
        }
        piece_type[pi] = found + 1; /* 1-based type id */
    }

    /* Initialize grids */
    memset(pgrid, 0, sizeof(pgrid));
    memset(tgrid, 0, sizeof(tgrid));
    memset(poff_y, 0, sizeof(poff_y));
    memset(poff_x, 0, sizeof(poff_x));
    for (int i = 0; i < npieces; i++)
        place_piece(i);

    /* Iterative DFS */
    stack_top = 0;
    stack[stack_top].ti = 0;
    stack[stack_top].di = 0;
    stack[stack_top].moved = 0;
    stack_top++;

    while (stack_top > 0) {
        Frame *f = &stack[stack_top - 1];

        if (f->moved) {
            /* Undo the previous move */
            int ti = f->ti, di = f->di;
            remove_piece(ti);
            poff_y[ti] -= DY[di];
            poff_x[ti] -= DX[di];
            place_piece(ti);
            f->moved = 0;
            /* Advance to next (ti, di) */
            f->di++;
            if (f->di >= 4) {
                f->di = 0;
                f->ti++;
            }
        }

        /* Try remaining (ti, di) pairs */
        int found = 0;
        while (f->ti < npieces) {
            while (f->di < 4) {
                if (can_move(f->ti, f->di)) {
                    /* Make the move */
                    remove_piece(f->ti);
                    poff_y[f->ti] += DY[f->di];
                    poff_x[f->ti] += DX[f->di];
                    place_piece(f->ti);
                    f->moved = 1;

                    if (hash_insert()) {
                        /* New state - push new frame to explore from it */
                        stack[stack_top].ti = 0;
                        stack[stack_top].di = 0;
                        stack[stack_top].moved = 0;
                        stack_top++;
                    }
                    found = 1;
                    break;
                }
                f->di++;
            }
            if (found) break;
            f->di = 0;
            f->ti++;
        }

        if (!found) {
            /* No more moves to try from this frame - backtrack */
            stack_top--;
        }
    }

    printf("%d\n", hash_count);
    free(hash_keys); free(hash_next); free(hash_buckets);
    return 0;
}
"""


def solve():
    tmpdir = tempfile.mkdtemp()
    c_file = os.path.join(tmpdir, "p766.c")
    exe_file = os.path.join(tmpdir, "p766")

    with open(c_file, "w") as f:
        f.write(C_CODE)

    result = subprocess.run(
        ["gcc", "-O2", "-o", exe_file, c_file],
        capture_output=True, text=True
    )
    if result.returncode != 0:
        print(f"Compile error: {result.stderr}", file=sys.stderr)
        sys.exit(1)

    result = subprocess.run(
        [exe_file],
        capture_output=True, text=True, timeout=120
    )

    os.unlink(c_file)
    os.unlink(exe_file)
    os.rmdir(tmpdir)

    return result.stdout.strip()


if __name__ == "__main__":
    print(solve())
