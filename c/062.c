/* Project Euler 062 - Cubic permutations */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_ENTRIES 100000

typedef struct {
    char signature[21]; /* sorted digits of cube */
    long long smallest;
    int count;
} Entry;

static Entry table[MAX_ENTRIES];
static int table_size = 0;

/* Sort characters of a string in place */
static void sort_chars(char *s) {
    int len = (int)strlen(s);
    for (int i = 0; i < len - 1; i++) {
        for (int j = i + 1; j < len; j++) {
            if (s[i] > s[j]) {
                char tmp = s[i];
                s[i] = s[j];
                s[j] = tmp;
            }
        }
    }
}

/* Simple hash table lookup/insert */
static int find_or_insert(const char *sig, long long cube) {
    /* Linear probing hash */
    unsigned long hash = 0;
    for (const char *p = sig; *p; p++) {
        hash = hash * 31 + (unsigned long)*p;
    }
    int idx = (int)(hash % MAX_ENTRIES);
    while (1) {
        if (table[idx].count == 0) {
            /* New entry */
            strcpy(table[idx].signature, sig);
            table[idx].smallest = cube;
            table[idx].count = 1;
            table_size++;
            return 1;
        }
        if (strcmp(table[idx].signature, sig) == 0) {
            table[idx].count++;
            return table[idx].count;
        }
        idx = (idx + 1) % MAX_ENTRIES;
    }
}

static long long find_smallest(const char *sig) {
    unsigned long hash = 0;
    for (const char *p = sig; *p; p++) {
        hash = hash * 31 + (unsigned long)*p;
    }
    int idx = (int)(hash % MAX_ENTRIES);
    while (1) {
        if (strcmp(table[idx].signature, sig) == 0) {
            return table[idx].smallest;
        }
        idx = (idx + 1) % MAX_ENTRIES;
    }
}

int main(void) {
    memset(table, 0, sizeof(table));

    for (long long n = 1; ; n++) {
        long long cube = n * n * n;
        char s[21];
        sprintf(s, "%lld", cube);
        char sig[21];
        strcpy(sig, s);
        sort_chars(sig);

        int cnt = find_or_insert(sig, cube);
        if (cnt == 5) {
            printf("%lld\n", find_smallest(sig));
            return 0;
        }
    }

    return 0;
}
