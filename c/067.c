/* Project Euler 067 - Maximum path sum II
 * Reads triangle from ../data/0067_triangle.txt */
#include <stdio.h>

#define MAXROWS 100

static int triangle[MAXROWS][MAXROWS];
static int nrows;

int main(void) {
    FILE *f = fopen("../data/0067_triangle.txt", "r");
    if (!f) {
        f = fopen("data/0067_triangle.txt", "r");
    }
    if (!f) {
        fprintf(stderr, "Cannot open triangle file\n");
        return 1;
    }

    nrows = 0;
    int val;
    int col = 0;
    int expected = 1;

    while (fscanf(f, "%d", &val) == 1) {
        triangle[nrows][col] = val;
        col++;
        if (col == expected) {
            nrows++;
            expected++;
            col = 0;
        }
    }
    fclose(f);

    /* Dynamic programming: work from bottom to top */
    for (int i = nrows - 2; i >= 0; i--) {
        for (int j = 0; j <= i; j++) {
            int left = triangle[i + 1][j];
            int right = triangle[i + 1][j + 1];
            triangle[i][j] += (left > right) ? left : right;
        }
    }

    printf("%d\n", triangle[0][0]);
    return 0;
}
