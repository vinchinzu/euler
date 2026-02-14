/*
 * Project Euler Problem 167: Investigating Ulam sequences
 *
 * For v = 5, 7, 9, ..., 21, compute the 10^11-th term of U(2, v).
 * Uses the periodicity of the odd membership pattern.
 *
 * Optimized: uses hash set for Ulam terms and sums to speed up next_ulam.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long i64;
typedef unsigned char u8;

/* ---- Bitarray for odd_flags ---- */
static u8 *bits;
static int bits_cap;

static void bits_init(int initial_bytes) {
    bits_cap = initial_bytes;
    bits = calloc(bits_cap, 1);
}

static void bits_free(void) { free(bits); bits = NULL; bits_cap = 0; }

static void bits_ensure(int idx) {
    int needed = (idx >> 3) + 1;
    if (needed > bits_cap) {
        int new_cap = bits_cap;
        while (new_cap < needed) new_cap *= 2;
        bits = realloc(bits, new_cap);
        memset(bits + bits_cap, 0, new_cap - bits_cap);
        bits_cap = new_cap;
    }
}

static inline int bits_get(int idx) {
    if (idx < 0) return 0;
    int byte = idx >> 3;
    if (byte >= bits_cap) return 0;
    return (bits[byte] >> (idx & 7)) & 1;
}

static inline void bits_set1(int idx) {
    bits_ensure(idx);
    bits[idx >> 3] |= (1 << (idx & 7));
}

/* ---- Hash set for counting sums ---- */
/* We use an array indexed by value to count how many ways each
 * value can be represented as sum of two distinct Ulam terms. */
static int *sum_counts;  /* sum_counts[s] = number of pairs */
static int sum_cap;

static void sc_init(int cap) {
    sum_cap = cap;
    sum_counts = calloc(cap, sizeof(int));
}

static void sc_free(void) { free(sum_counts); sum_counts = NULL; }

static void sc_ensure(int val) {
    if (val >= sum_cap) {
        int new_cap = sum_cap;
        while (new_cap <= val) new_cap *= 2;
        sum_counts = realloc(sum_counts, new_cap * sizeof(int));
        memset(sum_counts + sum_cap, 0, (new_cap - sum_cap) * sizeof(int));
        sum_cap = new_cap;
    }
}

/* When we add a new term `val` to the Ulam sequence, we need to add val+prev_term
 * to sum_counts for all prev terms. */

static int *terms;
static int terms_count, terms_cap_t;

static void terms_init(void) {
    terms_count = 0;
    terms_cap_t = 8192;
    terms = malloc(terms_cap_t * sizeof(int));
}

static void terms_push_only(int val) {
    if (terms_count >= terms_cap_t) {
        terms_cap_t *= 2;
        terms = realloc(terms, terms_cap_t * sizeof(int));
    }
    terms[terms_count++] = val;
}

/* Add a new Ulam term: update sum_counts for all existing terms */
static void add_ulam_term(int val) {
    for (int i = 0; i < terms_count; i++) {
        int s = terms[i] + val;
        sc_ensure(s);
        /* Cap at 2 to save space/time */
        if (sum_counts[s] < 2) sum_counts[s]++;
    }
    terms_push_only(val);
}

/* Find next Ulam number: smallest integer > last term with exactly 1 representation */
static int next_ulam(void) {
    int candidate = terms[terms_count - 1] + 1;
    while (1) {
        if (candidate < sum_cap && sum_counts[candidate] == 1) {
            add_ulam_term(candidate);
            return candidate;
        }
        candidate++;
    }
}

/* ---- State hash for period detection ---- */
#define STATE_HASH_SIZE (1 << 22)
#define STATE_HASH_MASK (STATE_HASH_SIZE - 1)

typedef struct { i64 key; int value; int used; } StateEntry;
static StateEntry *state_table;

static void state_init(void) {
    state_table = calloc(STATE_HASH_SIZE, sizeof(StateEntry));
}

static void state_free(void) { free(state_table); state_table = NULL; }

static inline unsigned int state_hash_fn(i64 key) {
    unsigned long long h = (unsigned long long)key;
    h ^= h >> 17;
    h *= 0xFF51AFD7ED558CCDULL;
    h ^= h >> 33;
    return (unsigned int)(h & STATE_HASH_MASK);
}

static int state_get(i64 key) {
    unsigned int idx = state_hash_fn(key);
    while (1) {
        if (!state_table[idx].used) return -1;
        if (state_table[idx].key == key) return state_table[idx].value;
        idx = (idx + 1) & STATE_HASH_MASK;
    }
}

static void state_set(i64 key, int value) {
    unsigned int idx = state_hash_fn(key);
    while (1) {
        if (!state_table[idx].used) {
            state_table[idx].key = key;
            state_table[idx].value = value;
            state_table[idx].used = 1;
            return;
        }
        if (state_table[idx].key == key) {
            state_table[idx].value = value;
            return;
        }
        idx = (idx + 1) & STATE_HASH_MASK;
    }
}

static int cmp_int(const void *a, const void *b) {
    return *(const int *)a - *(const int *)b;
}

static i64 get_ulam_k(int v, i64 k) {
    terms_init();
    sc_init(65536);

    /* Initialize with first two terms */
    add_ulam_term(2);
    add_ulam_term(v);

    bits_init(65536);
    bits_set1((v - 1) / 2);

    int even_terms[3];
    int even_count = 1;
    even_terms[0] = 2;

    while (even_count < 2) {
        int value = next_ulam();
        if (value & 1)
            bits_set1((value - 1) / 2);
        else
            even_terms[even_count++] = value;
    }

    int e2 = even_terms[1];
    int t = e2 / 2;

    int max_index = (terms[terms_count - 1] - 1) / 2;

    /* Build initial state from last t bits */
    i64 state_mask = (t < 64) ? ((1LL << t) - 1) : -1LL;
    int i = max_index + 1;
    i64 state = 0;
    for (int offset = 0; offset < t && offset < 64; offset++) {
        int idx2 = i - t + offset;
        int bit = (idx2 >= 0) ? bits_get(idx2) : 0;
        state = (state << 1) | bit;
    }

    state_init();
    state_set(state, i);

    int period_start = 0, period_length = 0;

    while (1) {
        int prev_flag = bits_get(i - 1);
        int shifted_flag = bits_get(i - t);
        int new_flag = prev_flag ^ shifted_flag;
        if (new_flag) bits_set1(i);

        state = ((state << 1) & state_mask) | new_flag;
        i++;

        int found = state_get(state);
        if (found >= 0) {
            period_start = found;
            period_length = i - found;
            break;
        }
        state_set(state, i);
    }

    /* Collect prefix odds */
    int *prefix_terms = NULL;
    int prefix_count = 0, prefix_cap = 0;

    for (int idx2 = 0; idx2 < period_start; idx2++) {
        if (bits_get(idx2)) {
            if (prefix_count >= prefix_cap) {
                prefix_cap = prefix_cap ? prefix_cap * 2 : 256;
                prefix_terms = realloc(prefix_terms, prefix_cap * sizeof(int));
            }
            prefix_terms[prefix_count++] = 2 * idx2 + 1;
        }
    }

    for (int j = 0; j < even_count; j++) {
        if (prefix_count >= prefix_cap) {
            prefix_cap = prefix_cap ? prefix_cap * 2 : 256;
            prefix_terms = realloc(prefix_terms, prefix_cap * sizeof(int));
        }
        prefix_terms[prefix_count++] = even_terms[j];
    }

    qsort(prefix_terms, prefix_count, sizeof(int), cmp_int);

    i64 result;
    if (k <= prefix_count) {
        result = prefix_terms[k - 1];
    } else {
        i64 remaining = k - prefix_count;

        int *period_indices = NULL;
        int pi_count = 0, pi_cap = 0;
        for (int offset = 0; offset < period_length; offset++) {
            if (bits_get(period_start + offset)) {
                if (pi_count >= pi_cap) {
                    pi_cap = pi_cap ? pi_cap * 2 : 256;
                    period_indices = realloc(period_indices, pi_cap * sizeof(int));
                }
                period_indices[pi_count++] = offset;
            }
        }

        i64 full_periods = (remaining - 1) / pi_count;
        i64 rem = (remaining - 1) % pi_count;

        i64 base_index = (i64)period_start + full_periods * period_length;
        i64 chosen_offset = period_indices[rem];
        i64 odd_index = base_index + chosen_offset;

        result = 2 * odd_index + 1;

        free(period_indices);
    }

    free(prefix_terms);
    free(terms);
    bits_free();
    sc_free();
    state_free();
    return result;
}

int main(void) {
    i64 total = 0;
    for (int n = 2; n <= 10; n++) {
        int v = 2 * n + 1;
        i64 val = get_ulam_k(v, 100000000000LL);
        total += val;
    }
    printf("%lld\n", total);
    return 0;
}
