#include <stdio.h>
#include <stdbool.h>
#include <string.h>

typedef struct {
    int grid[9][9];
} Sudoku;

bool find_empty(Sudoku *s, int *row, int *col) {
    for (*row = 0; *row < 9; (*row)++)
        for (*col = 0; *col < 9; (*col)++)
            if (s->grid[*row][*col] == 0)
                return true;
    return false;
}

bool is_safe(Sudoku *s, int row, int col, int num) {
    /* Check row */
    for (int c = 0; c < 9; c++)
        if (s->grid[row][c] == num) return false;
    /* Check column */
    for (int r = 0; r < 9; r++)
        if (s->grid[r][col] == num) return false;
    /* Check 3x3 box */
    int br = row - row % 3, bc = col - col % 3;
    for (int r = 0; r < 3; r++)
        for (int c = 0; c < 3; c++)
            if (s->grid[br + r][bc + c] == num) return false;
    return true;
}

bool solve(Sudoku *s) {
    int row, col;
    if (!find_empty(s, &row, &col))
        return true;
    for (int num = 1; num <= 9; num++) {
        if (is_safe(s, row, col, num)) {
            s->grid[row][col] = num;
            if (solve(s)) return true;
            s->grid[row][col] = 0;
        }
    }
    return false;
}

int main(void) {
    FILE *f = fopen("../data/p096_sudoku.txt", "r");
    if (!f) { fprintf(stderr, "Cannot open file\n"); return 1; }

    int total_sum = 0;
    char line[256];

    while (fgets(line, sizeof(line), f)) {
        if (strncmp(line, "Grid", 4) == 0) {
            Sudoku s;
            for (int i = 0; i < 9; i++) {
                if (!fgets(line, sizeof(line), f)) break;
                for (int j = 0; j < 9; j++)
                    s.grid[i][j] = line[j] - '0';
            }
            if (solve(&s))
                total_sum += s.grid[0][0] * 100 + s.grid[0][1] * 10 + s.grid[0][2];
        }
    }

    fclose(f);
    printf("%d\n", total_sum);
    return 0;
}
