/*
 * Project Euler Problem 598: Split Divisibilities.
 *
 * Find the number of pairs (a, b) with a <= b such that a*b = N!
 * and tau(a) = tau(b), where N = 100.
 *
 * Factor N! = prod p_i^{e_i}. For each factorization choice f_i (0 <= f_i <= e_i),
 * tau(a) = prod(f_i + 1), tau(b) = prod(e_i - f_i + 1).
 * We need tau(a) = tau(b), i.e., prod(f_i+1) = prod(e_i-f_i+1).
 *
 * Meet-in-the-middle approach: split primes into two halves.
 * For the "right" half, build a map from ratio (in lowest terms) to count.
 * For the "left" half, enumerate all combos and look up the complement ratio.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

typedef long long ll;

/* Primes up to 100 */
static int primes[] = {2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97};
static int num_primes = 25;

int gcd(int a, int b) {
    while (b) { int t = b; b = a % b; a = t; }
    return a;
}

/* Rational number as (numerator, denominator) in lowest terms */
typedef struct {
    int num;
    int den;
} Ratio;

Ratio make_ratio(int x, int y) {
    int g = gcd(x, y);
    Ratio r = {x / g, y / g};
    return r;
}

/* Hash map for ratios -> count */
#define HASH_SIZE 1000003

typedef struct Entry {
    int num, den;
    ll count;
    struct Entry *next;
} Entry;

Entry *hash_map[HASH_SIZE];

int hash_ratio(int num, int den) {
    unsigned h = (unsigned)(num * 100003 + den);
    return h % HASH_SIZE;
}

void hash_add(int num, int den, ll count) {
    int h = hash_ratio(num, den);
    for (Entry *e = hash_map[h]; e; e = e->next) {
        if (e->num == num && e->den == den) {
            e->count += count;
            return;
        }
    }
    Entry *e = (Entry *)malloc(sizeof(Entry));
    e->num = num;
    e->den = den;
    e->count = count;
    e->next = hash_map[h];
    hash_map[h] = e;
}

ll hash_get(int num, int den) {
    int h = hash_ratio(num, den);
    for (Entry *e = hash_map[h]; e; e = e->next) {
        if (e->num == num && e->den == den) return e->count;
    }
    return 0;
}

int main() {
    int N_fact = 100;

    /* Compute exponents for N! */
    int es[25];
    for (int i = 0; i < num_primes; i++) {
        int p = primes[i];
        int e = 0, n = N_fact;
        while (n > 0) {
            n /= p;
            e += n;
        }
        es[i] = e;
    }

    /* Find the split point (from the back) */
    /* The Python solution processes from the largest prime first,
     * building a map, then switches to brute force when the brute
     * force product < map size. */

    /* Build map from the "right" side (large primes) */
    memset(hash_map, 0, sizeof(hash_map));

    /* Start with ratio 1/1, count 1 */
    hash_add(1, 1, 1);

    int index = num_primes - 1;

    while (index >= 0) {
        int e = es[index];

        /* Check if brute force product for remaining primes is smaller than current map */
        ll prod = 1;
        for (int i = 0; i < index; i++) {
            prod *= (es[i] + 1);
            if (prod > 10000000) break;
        }

        /* Count map entries */
        ll map_size = 0;
        for (int h = 0; h < HASH_SIZE; h++) {
            for (Entry *en = hash_map[h]; en; en = en->next) map_size++;
        }

        if (prod < map_size) break;

        /* Expand map with current prime */
        /* Collect all entries */
        typedef struct { int num, den; ll count; } MapEntry;
        MapEntry *entries = NULL;
        int n_entries = 0, cap = 0;

        for (int h = 0; h < HASH_SIZE; h++) {
            for (Entry *en = hash_map[h]; en; en = en->next) {
                if (n_entries >= cap) {
                    cap = cap ? cap * 2 : 1024;
                    entries = (MapEntry *)realloc(entries, cap * sizeof(MapEntry));
                }
                entries[n_entries].num = en->num;
                entries[n_entries].den = en->den;
                entries[n_entries].count = en->count;
                n_entries++;
            }
        }

        /* Clear map */
        for (int h = 0; h < HASH_SIZE; h++) {
            Entry *en = hash_map[h];
            while (en) {
                Entry *next = en->next;
                free(en);
                en = next;
            }
            hash_map[h] = NULL;
        }

        /* Expand */
        for (int ei = 0; ei < n_entries; ei++) {
            int rnum = entries[ei].num;
            int rden = entries[ei].den;
            ll cnt = entries[ei].count;

            for (int f = 0; f <= e; f++) {
                int new_x = rnum * (f + 1);
                int new_y = rden * (e - f + 1);
                int g = gcd(new_x, new_y);
                hash_add(new_x / g, new_y / g, cnt);
            }
        }

        free(entries);
        index--;
    }

    /* Brute force the "left" side (primes 0..index) */
    int n_left = index + 1;

    /* Build axes for brute force */
    ll total_bf = 1;
    for (int i = 0; i < n_left; i++) total_bf *= (es[i] + 1);

    ll result = 0;

    for (ll combo = 0; combo < total_bf; combo++) {
        int x = 1, y = 1;
        ll tmp = combo;
        for (int i = 0; i < n_left; i++) {
            int f = (int)(tmp % (es[i] + 1));
            tmp /= (es[i] + 1);
            x *= (f + 1);
            y *= (es[i] - f + 1);
        }
        /* We need the ratio to match: x * map_num = y * map_den
         * i.e., map ratio should be y/x (in lowest terms) */
        int g = gcd(x, y);
        int target_num = y / g;
        int target_den = x / g;
        result += hash_get(target_num, target_den);
    }

    result /= 2;

    printf("%lld\n", result);

    /* Cleanup */
    for (int h = 0; h < HASH_SIZE; h++) {
        Entry *en = hash_map[h];
        while (en) {
            Entry *next = en->next;
            free(en);
            en = next;
        }
    }

    return 0;
}
