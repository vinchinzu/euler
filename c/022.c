/*
 * Project Euler 022 - Names Scores
 * Sort names from file and compute total name scores.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_NAMES 6000
#define MAX_NAME_LEN 64

char names[MAX_NAMES][MAX_NAME_LEN];
int name_count = 0;

int name_score(const char *name) {
    int score = 0;
    for (int i = 0; name[i] != '\0'; i++) {
        score += name[i] - 'A' + 1;
    }
    return score;
}

int cmp_strings(const void *a, const void *b) {
    return strcmp((const char *)a, (const char *)b);
}

int main(void) {
    FILE *file = fopen("../data/names.txt", "r");
    if (!file) {
        fprintf(stderr, "Cannot open ../data/names.txt\n");
        return 1;
    }

    /* Read the entire file */
    fseek(file, 0, SEEK_END);
    long fsize = ftell(file);
    fseek(file, 0, SEEK_SET);

    char *buffer = (char *)malloc(fsize + 1);
    fread(buffer, 1, fsize, file);
    buffer[fsize] = '\0';
    fclose(file);

    /* Parse comma-separated quoted names */
    char *p = buffer;
    while (*p && name_count < MAX_NAMES) {
        /* Find opening quote */
        while (*p && *p != '"') p++;
        if (!*p) break;
        p++; /* skip opening quote */

        /* Copy name until closing quote */
        int i = 0;
        while (*p && *p != '"' && i < MAX_NAME_LEN - 1) {
            names[name_count][i++] = *p++;
        }
        names[name_count][i] = '\0';
        name_count++;

        if (*p == '"') p++; /* skip closing quote */
        if (*p == ',') p++; /* skip comma */
    }

    free(buffer);

    /* Sort names */
    qsort(names, name_count, MAX_NAME_LEN, cmp_strings);

    long long total = 0;
    for (int i = 0; i < name_count; i++) {
        total += (long long)name_score(names[i]) * (i + 1);
    }

    printf("%lld\n", total);
    return 0;
}
