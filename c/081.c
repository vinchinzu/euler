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

    dp[0][0] = matrix[0][0];
    for (int j = 1; j < SIZE; j++)
        dp[0][j] = matrix[0][j] + dp[0][j-1];
    for (int i = 1; i < SIZE; i++)
        dp[i][0] = matrix[i][0] + dp[i-1][0];
    for (int i = 1; i < SIZE; i++)
        for (int j = 1; j < SIZE; j++)
            dp[i][j] = matrix[i][j] + min(dp[i-1][j], dp[i][j-1]);

    printf("%d\n", dp[SIZE-1][SIZE-1]);
    return 0;
}
