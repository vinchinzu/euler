/* Project Euler Problem 118: Pandigital Prime Sets */
#include <stdio.h>
#include <stdbool.h>
#include <string.h>
#include <stdlib.h>

#define ALL_MASK 511  /* (1 << 9) - 1 */

typedef unsigned long long ull;
typedef __int128 i128;

static ull mulmod(ull a, ull b, ull m) {
    return (i128)a * b % m;
}

static ull powmod(ull base, ull exp, ull mod) {
    ull result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) result = mulmod(result, base, mod);
        base = mulmod(base, base, mod);
        exp >>= 1;
    }
    return result;
}

static bool is_prime(ull n) {
    if (n < 2) return false;
    if (n < 4) return true;
    if (n % 2 == 0 || n % 3 == 0) return false;
    if (n < 25) return true;

    ull d = n - 1;
    int r = 0;
    while (d % 2 == 0) { d /= 2; r++; }

    ull witnesses[] = {2, 3, 5, 7, 11, 13};
    int nw = 6;
    for (int w = 0; w < nw; w++) {
        ull a = witnesses[w];
        if (a >= n) continue;
        ull x = powmod(a, d, n);
        if (x == 1 || x == n - 1) continue;
        bool composite = true;
        for (int i = 0; i < r - 1; i++) {
            x = mulmod(x, x, n);
            if (x == n - 1) { composite = false; break; }
        }
        if (composite) return false;
    }
    return true;
}

/* For each bitmask, store the primes that can be formed from those digits */
/* Digits: bit i corresponds to digit (i+1), i.e., bit 0 = digit 1, ..., bit 8 = digit 9 */

/* Maximum primes per mask - we need dynamic arrays */
typedef struct {
    ull *primes;
    int count;
    int capacity;
} PrimeList;

static PrimeList primes_by_mask[512];

static void add_prime(int mask, ull p) {
    PrimeList *pl = &primes_by_mask[mask];
    if (pl->count >= pl->capacity) {
        pl->capacity = pl->capacity ? pl->capacity * 2 : 16;
        pl->primes = realloc(pl->primes, pl->capacity * sizeof(ull));
    }
    pl->primes[pl->count++] = p;
}

static int digits_from_mask[512][9];
static int digit_count[512];

static void precompute_digits(void) {
    for (int mask = 1; mask <= ALL_MASK; mask++) {
        int cnt = 0;
        for (int i = 0; i < 9; i++)
            if (mask & (1 << i))
                digits_from_mask[mask][cnt++] = i + 1;
        digit_count[mask] = cnt;
    }
}

/* Generate all permutations of digits and check primality */
static void gen_perms(int *digits, int n, int depth, ull current, int mask) {
    if (depth == n) {
        if (is_prime(current))
            add_prime(mask, current);
        return;
    }
    for (int i = depth; i < n; i++) {
        /* Swap */
        int tmp = digits[depth]; digits[depth] = digits[i]; digits[i] = tmp;

        /* Skip multi-digit numbers ending in even or 5 */
        if (depth == n - 1 && n > 1) {
            int last = digits[depth];
            if (last % 2 == 0 || last == 5) {
                /* Swap back */
                digits[i] = digits[depth]; digits[depth] = tmp;
                continue;
            }
        }

        gen_perms(digits, n, depth + 1, current * 10 + digits[depth], mask);

        /* Swap back */
        digits[i] = digits[depth]; digits[depth] = tmp;
    }
}

static int cmp_ull(const void *a, const void *b) {
    ull va = *(const ull*)a, vb = *(const ull*)b;
    if (va < vb) return -1;
    if (va > vb) return 1;
    return 0;
}

/* Remove duplicates from sorted array */
static int unique(ull *arr, int n) {
    if (n == 0) return 0;
    int j = 0;
    for (int i = 1; i < n; i++)
        if (arr[i] != arr[j])
            arr[++j] = arr[i];
    return j + 1;
}

/* DFS with memoization */
typedef struct {
    int mask;
    ull last;
    long long result;
} MemoEntry;

#define MEMO_SIZE (1 << 20)
static MemoEntry memo_table[MEMO_SIZE];
static bool memo_used[MEMO_SIZE];

static unsigned int memo_hash(int mask, ull last) {
    unsigned int h = (unsigned int)mask * 2654435761u ^ (unsigned int)(last >> 16) ^ (unsigned int)last;
    return h & (MEMO_SIZE - 1);
}

static long long dfs(int mask, ull last) {
    if (mask == 0) return 1;

    unsigned int h = memo_hash(mask, last);
    /* Linear probing */
    for (int probe = 0; probe < 32; probe++) {
        unsigned int idx = (h + probe) & (MEMO_SIZE - 1);
        if (!memo_used[idx]) break;
        if (memo_table[idx].mask == mask && memo_table[idx].last == last)
            return memo_table[idx].result;
    }

    long long total = 0;
    int sub = mask;
    while (sub > 0) {
        PrimeList *pl = &primes_by_mask[sub];
        for (int i = 0; i < pl->count; i++) {
            if (pl->primes[i] > last)
                total += dfs(mask ^ sub, pl->primes[i]);
        }
        sub = (sub - 1) & mask;
    }

    /* Store in memo */
    h = memo_hash(mask, last);
    for (int probe = 0; probe < 32; probe++) {
        unsigned int idx = (h + probe) & (MEMO_SIZE - 1);
        if (!memo_used[idx]) {
            memo_used[idx] = true;
            memo_table[idx].mask = mask;
            memo_table[idx].last = last;
            memo_table[idx].result = total;
            break;
        }
    }

    return total;
}

int main(void) {
    memset(primes_by_mask, 0, sizeof(primes_by_mask));
    memset(memo_used, 0, sizeof(memo_used));
    precompute_digits();

    for (int mask = 1; mask <= ALL_MASK; mask++) {
        int digits[9];
        int n = digit_count[mask];
        for (int i = 0; i < n; i++)
            digits[i] = digits_from_mask[mask][i];

        gen_perms(digits, n, 0, 0, mask);

        /* Sort and deduplicate */
        if (primes_by_mask[mask].count > 0) {
            qsort(primes_by_mask[mask].primes, primes_by_mask[mask].count, sizeof(ull), cmp_ull);
            primes_by_mask[mask].count = unique(primes_by_mask[mask].primes, primes_by_mask[mask].count);
        }
    }

    printf("%lld\n", dfs(ALL_MASK, 0));

    /* Free */
    for (int mask = 0; mask <= ALL_MASK; mask++)
        free(primes_by_mask[mask].primes);

    return 0;
}
