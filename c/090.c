#include <stdio.h>
#include <stdbool.h>

/* Required squares: 01, 04, 09, 16, 25, 36, 49, 64, 81 */
static int sq_d1[] = {0, 0, 0, 1, 2, 3, 4, 6, 8};
static int sq_d2[] = {1, 4, 9, 6, 5, 6, 9, 4, 1};
static int nsquares = 9;

/* C(10,6) combinations stored as arrays of 6 digits */
int combos[210][6];
int ncombos = 0;

void gen_combos(int *cur, int depth, int start) {
    if (depth == 6) {
        for (int i = 0; i < 6; i++)
            combos[ncombos][i] = cur[i];
        ncombos++;
        return;
    }
    for (int i = start; i <= 10 - (6 - depth); i++) {
        cur[depth] = i;
        gen_combos(cur, depth + 1, i + 1);
    }
}

bool cube_has(int *cube, int d) {
    for (int i = 0; i < 6; i++) {
        if (cube[i] == d) return true;
        /* 6 and 9 are interchangeable */
        if ((d == 6 || d == 9) && (cube[i] == 6 || cube[i] == 9)) return true;
    }
    return false;
}

bool can_display_all(int *c1, int *c2) {
    for (int s = 0; s < nsquares; s++) {
        int d1 = sq_d1[s], d2 = sq_d2[s];
        bool ok = (cube_has(c1, d1) && cube_has(c2, d2)) ||
                  (cube_has(c1, d2) && cube_has(c2, d1));
        if (!ok) return false;
    }
    return true;
}

int main(void) {
    int cur[6];
    gen_combos(cur, 0, 0);

    int count = 0;
    for (int i = 0; i < ncombos; i++)
        for (int j = i; j < ncombos; j++)
            if (can_display_all(combos[i], combos[j]))
                count++;

    printf("%d\n", count);
    return 0;
}
