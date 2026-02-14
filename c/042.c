#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>
#include <math.h>

int word_value(const char *word) {
    int sum = 0;
    for (int i = 0; word[i]; i++) {
        if (word[i] >= 'A' && word[i] <= 'Z') {
            sum += word[i] - 'A' + 1;
        }
    }
    return sum;
}

bool is_triangle(int n) {
    if (n < 0) return false;
    long long disc = 1LL + 8LL * n;
    long long m = (long long)sqrt((double)disc);
    /* Adjust for floating point imprecision */
    while (m * m < disc) m++;
    while (m * m > disc) m--;
    return m * m == disc && (m + 1) % 2 == 0;
}

int main(void) {
    FILE *f = fopen("../data/words.txt", "r");
    if (!f) {
        fprintf(stderr, "Error: cannot open ../data/words.txt\n");
        return 1;
    }

    /* Read entire file */
    fseek(f, 0, SEEK_END);
    long fsize = ftell(f);
    fseek(f, 0, SEEK_SET);

    char *buf = (char *)malloc(fsize + 1);
    fread(buf, 1, fsize, f);
    buf[fsize] = '\0';
    fclose(f);

    int count = 0;
    char word[64];
    int wi = 0;
    bool in_quotes = false;

    for (long i = 0; i <= fsize; i++) {
        char c = buf[i];
        if (c == '"') {
            in_quotes = !in_quotes;
        } else if ((c == ',' || c == '\0') && !in_quotes) {
            word[wi] = '\0';
            if (wi > 0 && is_triangle(word_value(word))) {
                count++;
            }
            wi = 0;
        } else {
            word[wi++] = c;
        }
    }

    free(buf);
    printf("%d\n", count);
    return 0;
}
