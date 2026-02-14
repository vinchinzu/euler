/* Project Euler 494 - Collatz prefix families
 * Translated from python/494.py + python/494_helper.c
 *
 * Self-contained: finds special Collatz sequences and counts paths.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef unsigned long long ull;

#define NSTEPS 90
#define L 100000

/* Compute M = 3^39 */
static ull M3_39;

void compute_M(void) {
    M3_39 = 1;
    for (int i = 0; i < 39; i++)
        M3_39 *= 3;
}

/* Fibonacci: F_1=1, F_2=1, F_3=2, ... */
ull fibonacci(int n) {
    if (n <= 0) return 0;
    ull a = 1, b = 1;
    for (int i = 2; i < n; i++) {
        ull c = a + b;
        a = b;
        b = c;
    }
    return (n == 1) ? a : b;
}

int is_power_of_2(ull n) {
    return n > 0 && (n & (n - 1)) == 0;
}

ull helper_func(ull n, int numSteps, int nEven) {
    if (numSteps == 0) return 1;
    ull result = helper_func((n * 2) % M3_39, numSteps - 1, 1);
    if (nEven && n % 3 == 1)
        result += helper_func((n - 1) / 3, numSteps - 1, 0);
    return result;
}

/* Store special sequences */
typedef struct {
    ull start;
    ull collatz[NSTEPS + 1];
    int len;
} SeqEntry;

int main() {
    compute_M();

    ull ans = fibonacci(NSTEPS);

    /* Find special sequences */
    SeqEntry *specials = malloc(L * sizeof(SeqEntry));
    int num_specials = 0;

    /* We also need a set of special start values for redundancy check */
    /* Use a simple hash set */
    #define HASH_SIZE 200003
    ull *hash_table = calloc(HASH_SIZE, sizeof(ull));
    char *hash_used = calloc(HASH_SIZE, 1);

    for (ull start = 1; start < L; start++) {
        ull collatz[NSTEPS + 1];
        int len = 0;
        ull n = start;
        double r = 1.0;
        int is_special = 0;

        for (int i = 0; i < NSTEPS; i++) {
            collatz[len++] = n;
            if (n % 2 == 0) {
                n /= 2;
                r /= 2;
            } else {
                n = 3 * n + 1;
                r *= 3;
            }
            if (is_power_of_2(n)) break;
            if (n > start && r < 1.0) is_special = 1;
        }

        if (is_special) {
            specials[num_specials].start = start;
            memcpy(specials[num_specials].collatz, collatz, len * sizeof(ull));
            specials[num_specials].len = len;
            num_specials++;

            /* Add to hash set */
            ull idx = start % HASH_SIZE;
            while (hash_used[idx]) {
                if (hash_table[idx] == start) break;
                idx = (idx + 1) % HASH_SIZE;
            }
            hash_table[idx] = start;
            hash_used[idx] = 1;
        }
    }

    /* Check if a value is in the special set */
    #define in_special(v) ({ \
        int _found = 0; \
        ull _idx = (v) % HASH_SIZE; \
        while (hash_used[_idx]) { \
            if (hash_table[_idx] == (v)) { _found = 1; break; } \
            _idx = (_idx + 1) % HASH_SIZE; \
        } \
        _found; \
    })

    /* Remove redundant sequences */
    for (int si = 0; si < num_specials; si++) {
        int is_redundant = 0;
        for (int i = 1; i < specials[si].len; i++) {
            if (in_special(specials[si].collatz[i])) {
                is_redundant = 1;
                break;
            }
        }
        if (!is_redundant) {
            int num_steps = NSTEPS - specials[si].len;
            ull paths = helper_func(specials[si].start, num_steps, 0);
            ans += paths;
        }
    }

    printf("%llu\n", ans);

    free(specials);
    free(hash_table);
    free(hash_used);
    return 0;
}
