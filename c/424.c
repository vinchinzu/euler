/*
 * Project Euler 424 - Kakuro puzzles
 *
 * Solve 200 cryptic kakuro puzzles. Constraint propagation + backtracking.
 * Extracted from embedded C in python/424.py.
 * Reads data file from ../data/kakuro200.txt (relative to executable).
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#define MAXCELLS 50
#define MAXCLUES 50
#define MAXSIZE 8

typedef struct {
    int sum_letters[3];
    int n_sum_letters;
    int cells[6];
    int n_cells;
} Clue;

typedef struct {
    int size;
    int n_cells;
    int n_clues;
    int cell_letter[MAXCELLS];
    int cell_clues[MAXCELLS][8];
    int n_cell_clues[MAXCELLS];
    Clue clues[MAXCLUES];
    int nonzero_letters;
    int letter_to_cells[10][MAXCELLS];
    int n_letter_cells[10];
} Puzzle;

int letter_vals[10];
int cell_vals[MAXCELLS];
int used_letters;

int compute_target(Puzzle *p, int clue_idx) {
    Clue *c = &p->clues[clue_idx];
    int target = 0;
    for (int i = 0; i < c->n_sum_letters; i++) {
        int v = letter_vals[c->sum_letters[i]];
        if (v < 0) return -1;
        target = target * 10 + v;
    }
    return target;
}

int check_clue(Puzzle *p, int clue_idx) {
    Clue *c = &p->clues[clue_idx];
    int target = compute_target(p, clue_idx);
    if (target < 0) return 1;

    int partial_sum = 0, unassigned = 0;
    int seen = 0;
    for (int i = 0; i < c->n_cells; i++) {
        int v = cell_vals[c->cells[i]];
        if (v > 0) {
            if (seen & (1 << v)) return 0;
            seen |= (1 << v);
            partial_sum += v;
        } else {
            unassigned++;
        }
    }

    if (unassigned == 0)
        return partial_sum == target;

    int remaining = target - partial_sum;
    int avail = 0x3FE & ~seen;
    int count = __builtin_popcount(avail);
    if (count < unassigned) return 0;

    int min_sum = 0, max_sum = 0, cnt = 0;
    for (int v = 1; v <= 9 && cnt < unassigned; v++)
        if (avail & (1 << v)) { min_sum += v; cnt++; }
    cnt = 0;
    for (int v = 9; v >= 1 && cnt < unassigned; v--)
        if (avail & (1 << v)) { max_sum += v; cnt++; }

    return min_sum <= remaining && remaining <= max_sum;
}

int propagate(Puzzle *p) {
    int changed = 1;
    while (changed) {
        changed = 0;
        for (int ci = 0; ci < p->n_cells; ci++) {
            int li = p->cell_letter[ci];
            if (li < 0) continue;
            if (letter_vals[li] >= 0 && cell_vals[ci] <= 0) {
                if (letter_vals[li] < 1 || letter_vals[li] > 9) return 0;
                cell_vals[ci] = letter_vals[li];
                changed = 1;
            } else if (cell_vals[ci] > 0 && letter_vals[li] < 0) {
                int v = cell_vals[ci];
                if (used_letters & (1 << v)) return 0;
                letter_vals[li] = v;
                used_letters |= (1 << v);
                for (int k = 0; k < p->n_letter_cells[li]; k++) {
                    int ci2 = p->letter_to_cells[li][k];
                    if (cell_vals[ci2] <= 0) { cell_vals[ci2] = v; changed = 1; }
                    else if (cell_vals[ci2] != v) return 0;
                }
                changed = 1;
            } else if (letter_vals[li] >= 0 && cell_vals[ci] > 0) {
                if (letter_vals[li] != cell_vals[ci]) return 0;
            }
        }

        for (int ci_idx = 0; ci_idx < p->n_clues; ci_idx++) {
            Clue *c = &p->clues[ci_idx];
            int target = compute_target(p, ci_idx);
            if (target < 0) continue;

            int unassigned_ci = -2, partial_sum = 0, seen = 0;
            for (int i = 0; i < c->n_cells; i++) {
                int v = cell_vals[c->cells[i]];
                if (v > 0) {
                    if (seen & (1 << v)) return 0;
                    seen |= (1 << v);
                    partial_sum += v;
                } else if (unassigned_ci == -2) {
                    unassigned_ci = c->cells[i];
                } else {
                    unassigned_ci = -1;
                }
            }

            if (unassigned_ci >= 0) {
                int needed = target - partial_sum;
                if (needed < 1 || needed > 9 || (seen & (1 << needed))) return 0;
                cell_vals[unassigned_ci] = needed;
                changed = 1;
            }
        }
    }
    return 1;
}

int check_all(Puzzle *p) {
    for (int ci_idx = 0; ci_idx < p->n_clues; ci_idx++)
        if (!check_clue(p, ci_idx)) return 0;
    return 1;
}

int solve_bt(Puzzle *p) {
    int save_l[10], save_c[MAXCELLS], save_used;
    memcpy(save_l, letter_vals, sizeof(letter_vals));
    memcpy(save_c, cell_vals, p->n_cells * sizeof(int));
    save_used = used_letters;

    if (!propagate(p) || !check_all(p)) {
        memcpy(letter_vals, save_l, sizeof(letter_vals));
        memcpy(cell_vals, save_c, p->n_cells * sizeof(int));
        used_letters = save_used;
        return 0;
    }

    int best_li = -1;
    for (int li = 0; li < 10; li++)
        if (letter_vals[li] < 0) { best_li = li; break; }

    if (best_li >= 0) {
        for (int val = 0; val <= 9; val++) {
            if (used_letters & (1 << val)) continue;
            if ((p->nonzero_letters & (1 << best_li)) && val == 0) continue;

            int sl[10], sc[MAXCELLS]; int su;
            memcpy(sl, letter_vals, sizeof(letter_vals));
            memcpy(sc, cell_vals, p->n_cells * sizeof(int));
            su = used_letters;

            letter_vals[best_li] = val;
            used_letters |= (1 << val);

            if (solve_bt(p)) return 1;

            memcpy(letter_vals, sl, sizeof(letter_vals));
            memcpy(cell_vals, sc, p->n_cells * sizeof(int));
            used_letters = su;
        }
        return 0;
    }

    int best_ci = -1;
    for (int ci = 0; ci < p->n_cells; ci++)
        if (cell_vals[ci] <= 0) { best_ci = ci; break; }

    if (best_ci < 0) return 1;

    int used_in_groups = 0;
    for (int k = 0; k < p->n_cell_clues[best_ci]; k++) {
        Clue *c = &p->clues[p->cell_clues[best_ci][k]];
        for (int i = 0; i < c->n_cells; i++)
            if (cell_vals[c->cells[i]] > 0)
                used_in_groups |= (1 << cell_vals[c->cells[i]]);
    }

    for (int val = 1; val <= 9; val++) {
        if (used_in_groups & (1 << val)) continue;

        int sl[10], sc[MAXCELLS]; int su;
        memcpy(sl, letter_vals, sizeof(letter_vals));
        memcpy(sc, cell_vals, p->n_cells * sizeof(int));
        su = used_letters;

        cell_vals[best_ci] = val;

        if (solve_bt(p)) return 1;

        memcpy(letter_vals, sl, sizeof(letter_vals));
        memcpy(cell_vals, sc, p->n_cells * sizeof(int));
        used_letters = su;
    }
    return 0;
}

void parse_puzzle(char *line, Puzzle *p) {
    memset(p, 0, sizeof(Puzzle));
    for (int i = 0; i < MAXCELLS; i++) p->cell_letter[i] = -1;

    char tokens[100][64];
    int n_tokens = 0;
    char *s = line;
    while (*s) {
        while (*s == ',') s++;
        if (!*s) break;
        char *t = tokens[n_tokens];
        if (*s == '(') {
            while (*s && *s != ')') *t++ = *s++;
            if (*s == ')') *t++ = *s++;
        } else {
            while (*s && *s != ',') *t++ = *s++;
        }
        *t = 0;
        n_tokens++;
    }

    p->size = atoi(tokens[0]);
    char grid[MAXSIZE][MAXSIZE][64];
    for (int i = 0; i < p->size; i++)
        for (int j = 0; j < p->size; j++)
            strcpy(grid[i][j], tokens[1 + i * p->size + j]);

    int cell_map[MAXSIZE][MAXSIZE];
    memset(cell_map, -1, sizeof(cell_map));

    for (int i = 0; i < p->size; i++) {
        for (int j = 0; j < p->size; j++) {
            char *c = grid[i][j];
            if (c[0] == 'X' || c[0] == '(') continue;
            int ci = p->n_cells++;
            cell_map[i][j] = ci;
            if (c[0] >= 'A' && c[0] <= 'J') {
                int li = c[0] - 'A';
                p->cell_letter[ci] = li;
                p->letter_to_cells[li][p->n_letter_cells[li]++] = ci;
            }
        }
    }

    for (int i = 0; i < p->size; i++) {
        for (int j = 0; j < p->size; j++) {
            char *c = grid[i][j];
            if (c[0] != '(') continue;

            char inner[64];
            strncpy(inner, c + 1, strlen(c) - 2);
            inner[strlen(c) - 2] = 0;

            char *tok = strtok(inner, ",");
            while (tok) {
                while (*tok == ' ') tok++;
                char dir = tok[0];
                char *letters_str = tok + 1;

                Clue *cl = &p->clues[p->n_clues];
                cl->n_sum_letters = 0;
                cl->n_cells = 0;

                for (char *ch = letters_str; *ch; ch++) {
                    if (*ch >= 'A' && *ch <= 'J')
                        cl->sum_letters[cl->n_sum_letters++] = *ch - 'A';
                }

                if (cl->n_sum_letters >= 2)
                    p->nonzero_letters |= (1 << cl->sum_letters[0]);

                if (dir == 'h') {
                    for (int dj = 1; j + dj < p->size; dj++) {
                        if (cell_map[i][j + dj] < 0) break;
                        cl->cells[cl->n_cells++] = cell_map[i][j + dj];
                    }
                } else if (dir == 'v') {
                    for (int di = 1; i + di < p->size; di++) {
                        if (cell_map[i + di][j] < 0) break;
                        cl->cells[cl->n_cells++] = cell_map[i + di][j];
                    }
                }

                if (cl->n_cells > 0) {
                    for (int k = 0; k < cl->n_cells; k++) {
                        int ci = cl->cells[k];
                        p->cell_clues[ci][p->n_cell_clues[ci]++] = p->n_clues;
                    }
                    p->n_clues++;
                }

                tok = strtok(NULL, ",");
            }
        }
    }
}

int main(int argc, char *argv[]) {
    /* Try to find the data file */
    const char *paths[] = {
        "data/kakuro200.txt",
        "../data/kakuro200.txt",
        "../../data/kakuro200.txt",
        NULL
    };

    FILE *f = NULL;
    if (argc > 1) {
        f = fopen(argv[1], "r");
    }
    if (!f) {
        for (int i = 0; paths[i]; i++) {
            f = fopen(paths[i], "r");
            if (f) break;
        }
    }
    if (!f) { fprintf(stderr, "Cannot open kakuro200.txt\n"); return 1; }

    long long total = 0;
    char line[4096];
    while (fgets(line, sizeof(line), f)) {
        int len = strlen(line);
        while (len > 0 && (line[len-1] == '\n' || line[len-1] == '\r')) line[--len] = 0;
        if (len == 0) continue;

        Puzzle p;
        parse_puzzle(line, &p);

        memset(letter_vals, -1, sizeof(letter_vals));
        memset(cell_vals, 0, sizeof(cell_vals));
        used_letters = 0;

        if (solve_bt(&p)) {
            long long val = 0;
            for (int i = 0; i < 10; i++)
                val = val * 10 + (letter_vals[i] >= 0 ? letter_vals[i] : 0);
            total += val;
        }
    }
    fclose(f);
    printf("%lld\n", total);
    return 0;
}
