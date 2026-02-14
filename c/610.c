/*
 * Project Euler Problem 610: Roman Numerals II
 *
 * Expected value of random Roman numeral string.
 * E = 1000*R/(1-R) + E_0 where E_0 is expected value without M's.
 * Recursive computation over valid Roman numeral prefixes.
 */
#include <stdio.h>
#include <string.h>

#define MAX_ROMAN 1000

/* Roman numeral conversion tables */
static int vals[] = {1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1};
static const char *syms[] = {"M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"};
static int nsyms = 13;

static char roman_strs[MAX_ROMAN][20];  /* roman numeral strings for 0..999 */
static int roman_vals[MAX_ROMAN]; /* not really needed, just 0..999 */

/* Map from roman string to value (-1 if not valid) */
/* We'll use a hash approach: store all valid roman numerals and their values */

#define HASH_SIZE 4096
#define HASH_MASK (HASH_SIZE - 1)

typedef struct Entry {
    char key[20];
    int value;
    struct Entry *next;
} Entry;

static Entry *hash_table[HASH_SIZE];
static Entry entries_pool[MAX_ROMAN];
static int pool_idx = 0;

static unsigned hash_str(const char *s) {
    unsigned h = 0;
    while (*s) { h = h * 31 + (unsigned char)*s; s++; }
    return h & HASH_MASK;
}

static void hash_put(const char *key, int value) {
    unsigned h = hash_str(key);
    Entry *e = &entries_pool[pool_idx++];
    strcpy(e->key, key);
    e->value = value;
    e->next = hash_table[h];
    hash_table[h] = e;
}

static int hash_get(const char *key) {
    unsigned h = hash_str(key);
    for (Entry *e = hash_table[h]; e; e = e->next) {
        if (strcmp(e->key, key) == 0) return e->value;
    }
    return -1;
}

static void to_roman(int n, char *buf) {
    buf[0] = '\0';
    for (int i = 0; i < nsyms; i++) {
        while (n >= vals[i]) {
            strcat(buf, syms[i]);
            n -= vals[i];
        }
    }
}

/* Letters for generating (excluding M for E_0 computation) */
static const char letters[] = {'I', 'V', 'X', 'L', 'C', 'D'};
static int nletters = 6;
static const char all_letters[] = {'I', 'V', 'X', 'L', 'C', 'D', 'M'};
static int nall = 7;

static double R = 0.14;
static double RE;

/* Cache for expected value computation */
#define CACHE_SIZE 4096
typedef struct CEntry {
    char key[20];
    double value;
    int valid;
    struct CEntry *next;
} CEntry;

static CEntry *cache_table[CACHE_SIZE];
static CEntry cache_pool[2000];
static int cache_pool_idx = 0;

static double get_cached(const char *key, int *found) {
    unsigned h = hash_str(key) % CACHE_SIZE;
    for (CEntry *e = cache_table[h]; e; e = e->next) {
        if (strcmp(e->key, key) == 0) { *found = 1; return e->value; }
    }
    *found = 0;
    return 0.0;
}

static void put_cached(const char *key, double value) {
    unsigned h = hash_str(key) % CACHE_SIZE;
    CEntry *e = &cache_pool[cache_pool_idx++];
    strcpy(e->key, key);
    e->value = value;
    e->valid = 1;
    e->next = cache_table[h];
    cache_table[h] = e;
}

static double get_expected(const char *letters_str) {
    int found;
    double cached = get_cached(letters_str, &found);
    if (found) return cached;

    double sum_prob = RE;
    int v = hash_get(letters_str);
    double expected = RE * (v >= 0 ? v : 0);

    /* Try appending each non-M letter */
    for (int i = 0; i < nletters; i++) {
        char new_str[20];
        int len = strlen(letters_str);
        memcpy(new_str, letters_str, len);
        new_str[len] = letters[i];
        new_str[len + 1] = '\0';

        if (hash_get(new_str) >= 0) {
            sum_prob += R;
            expected += R * get_expected(new_str);
        }
    }

    double result = sum_prob > 0.0 ? expected / sum_prob : 0.0;
    put_cached(letters_str, result);
    return result;
}

int main(void) {
    RE = 1.0 - 7.0 * R;

    memset(hash_table, 0, sizeof(hash_table));
    memset(cache_table, 0, sizeof(cache_table));

    /* Build all valid roman numerals for 0..999 */
    /* Note: empty string maps to 0 */
    hash_put("", 0);
    for (int i = 1; i < MAX_ROMAN; i++) {
        char buf[20];
        to_roman(i, buf);
        /* Only add if it doesn't contain M (since we handle M separately) */
        if (strchr(buf, 'M') == NULL) {
            hash_put(buf, i);
        }
    }

    double E0 = get_expected("");
    double ans = 1000.0 * R / (1.0 - R) + E0;

    printf("%.8f\n", ans);
    return 0;
}
