#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define SIZE 80

int matrix[SIZE][SIZE];
int dp[SIZE][SIZE];

int min(int a, int b) { return a < b ? a : b; }

void load_matrix(const char *filename) {
    FILE *f = fopen(filename, "r");
    if (!f) { fprintf(stderr, "Cannot open %s\n", filename); exit(1); }
    char line[2048];
    int row = 0;
    while (row < SIZE && fgets(line, sizeof(line), f)) {
        int col = 0;
        char *tok = strtok(line, ",\n\r");
        while (tok && col < SIZE) {
            matrix[row][col] = atoi(tok);
            tok = strtok(NULL, ",\n\r");
            col++;
        }
        row++;
    }
    fclose(f);
}

int main(void) {
    load_matrix("../data/matrix.txt");

    /* Initialize first column */
    for (int i = 0; i < SIZE; i++)
        dp[i][0] = matrix[i][0];

    for (int j = 1; j < SIZE; j++) {
        /* Pass 1: from left */
        for (int i = 0; i < SIZE; i++)
            dp[i][j] = dp[i][j-1] + matrix[i][j];

        /* Pass 2: from above */
        for (int i = 1; i < SIZE; i++)
            dp[i][j] = min(dp[i][j], dp[i-1][j] + matrix[i][j]);

        /* Pass 3: from below */
        for (int i = SIZE - 2; i >= 0; i--)
            dp[i][j] = min(dp[i][j], dp[i+1][j] + matrix[i][j]);
    }

    int result = dp[0][SIZE-1];
    for (int i = 1; i < SIZE; i++)
        if (dp[i][SIZE-1] < result)
            result = dp[i][SIZE-1];

    printf("%d\n", result);
    return 0;
}
