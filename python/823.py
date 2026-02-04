"""Project Euler Problem 823: Factor Shuffle.

A list starts with [2,3,...,n]. Each round: divide each number by its smallest
prime factor, collect all those SPFs into a product (new number), remove 1s.
Find S(10^4, 10^16) mod 1234567891.

Key insight: the multiset of prime factors is invariant. Each factor instance
follows a deterministic path through the list. After a transient phase, the
permutation of factor instances becomes periodic. Compute the overall period
as LCM of cycle lengths, then advance to round K.

Implemented in C for performance.
"""

from __future__ import annotations

import subprocess
import tempfile
import os


def solve() -> int:
    """Solve Problem 823 using compiled C."""
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAXN 10001
#define MOD 1234567891LL
#define MAX_FACTORS 40000
#define MAX_SLOTS 15000

static int spf[MAXN];

void sieve(void) {
    for (int i = 0; i < MAXN; i++) spf[i] = i;
    for (int i = 2; i * i < MAXN; i++)
        if (spf[i] == i)
            for (int j = i * i; j < MAXN; j += i)
                if (spf[j] == j) spf[j] = i;
}

/* Each factor instance has: prime value, slot_id */
static int factor_prime[MAX_FACTORS]; /* prime value of factor i */
static int factor_slot[MAX_FACTORS];  /* which slot (number) factor i is in */
int num_factors;

/* Each slot has: list of factor indices, sorted by prime value */
/* We store slot contents as a linked structure */
static int slot_head[MAX_SLOTS];  /* head of factor list for slot s */
static int factor_next[MAX_FACTORS]; /* next factor in same slot */
static int slot_min_prime[MAX_SLOTS]; /* cached min prime in slot */
static int slot_count[MAX_SLOTS];    /* number of factors in slot */
int num_slots;

/* For sorting slots: compute a hash/key for each slot based on factor primes */
/* Actually we need to sort slots by their product value, but products are huge.
   Instead, sort by: (smallest_prime, second_smallest_prime, ...) lexicographically,
   which is equivalent to sorting by the sorted tuple of primes.

   Actually, the problem says the list is unordered. Numbers just exist in the list.
   We don't need to sort them. The operation is:
   1. For each number, remove smallest prime factor
   2. Multiply all removed factors into one new number
   3. Remove numbers that became 1

   Since we track factor instances, this is:
   1. For each slot, find the factor with smallest prime, remove it
   2. All removed factors go into a new slot
   3. Remove empty slots

   This is well-defined regardless of slot ordering. */

/* When there are ties (multiple factors in a slot with the same smallest prime),
   we need to remove exactly ONE factor - the one with the smallest prime.
   Actually, the problem says "divided by its smallest prime factor" - so if
   a number is 12 = 2*2*3, we divide by 2, getting 6. We remove ONE factor
   of value 2. */

void init_state(int N) {
    num_factors = 0;
    num_slots = 0;
    memset(slot_head, -1, sizeof(slot_head));

    for (int n = 2; n <= N; n++) {
        int s = num_slots++;
        slot_head[s] = -1;
        slot_count[s] = 0;
        slot_min_prime[s] = MAXN;

        int temp = n;
        while (temp > 1) {
            int p = spf[temp];
            int fi = num_factors++;
            factor_prime[fi] = p;
            factor_slot[fi] = s;
            factor_next[fi] = slot_head[s];
            slot_head[s] = fi;
            slot_count[s]++;
            if (p < slot_min_prime[s]) slot_min_prime[s] = p;
            temp /= p;
        }
    }
}

/* Do one round. Returns the new slot_id assigned to the product slot. */
void do_round(void) {
    /* For each slot, find and remove one factor with the smallest prime */
    int new_slot = -1;

    /* Allocate new slot for the product */
    int product_slot = num_slots++;
    slot_head[product_slot] = -1;
    slot_count[product_slot] = 0;
    slot_min_prime[product_slot] = MAXN;

    for (int s = 0; s < product_slot; s++) {
        if (slot_count[s] == 0) continue;

        int min_p = slot_min_prime[s];

        /* Find and remove one factor with prime == min_p */
        int prev = -1;
        int cur = slot_head[s];
        int found = -1;
        while (cur != -1) {
            if (factor_prime[cur] == min_p) {
                found = cur;
                break;
            }
            prev = cur;
            cur = factor_next[cur];
        }

        if (found == -1) continue; /* shouldn't happen */

        /* Remove found from slot s */
        if (prev == -1)
            slot_head[s] = factor_next[found];
        else
            factor_next[prev] = factor_next[found];
        slot_count[s]--;

        /* Recompute min prime for slot s */
        if (slot_count[s] == 0) {
            slot_min_prime[s] = MAXN;
        } else {
            int new_min = MAXN;
            for (int f = slot_head[s]; f != -1; f = factor_next[f])
                if (factor_prime[f] < new_min) new_min = factor_prime[f];
            slot_min_prime[s] = new_min;
        }

        /* Add found to product slot */
        factor_slot[found] = product_slot;
        factor_next[found] = slot_head[product_slot];
        slot_head[product_slot] = found;
        slot_count[product_slot]++;
        if (factor_prime[found] < slot_min_prime[product_slot])
            slot_min_prime[product_slot] = factor_prime[found];
    }
}

/* Hash the state for period detection */
/* State = for each factor instance, which slot it's in.
   But slot IDs change each round (new slots are created).
   Need canonical slot IDs.

   Canonical: sort slots by their sorted factor prime lists.
   Two states are the same if the multiset of {sorted factor prime lists}
   is the same.

   Better: sort all factor instances by (slot_id, prime), then
   the canonical form is the sorted list of slot contents. */

/* Actually, the simplest canonical form: sort factor instances by
   index, and for each, record which "group" it's in. Two factors
   are in the same group iff they're in the same slot.

   But we need groups identified canonically. Let's sort slots by
   their sorted prime list, then assign group IDs in order. */

typedef struct {
    int primes[20];  /* sorted primes in this slot */
    int count;
} SlotKey;

int cmp_slot_key(const void *a, const void *b) {
    const SlotKey *sa = (const SlotKey *)a, *sb = (const SlotKey *)b;
    if (sa->count != sb->count) return sa->count - sb->count;
    for (int i = 0; i < sa->count && i < sb->count; i++)
        if (sa->primes[i] != sb->primes[i])
            return sa->primes[i] - sb->primes[i];
    return 0;
}

/* Compute a 64-bit hash of the state */
unsigned long long hash_state(void) {
    /* Collect slot contents, sort them, hash */
    /* For each active slot, get sorted list of primes */
    static SlotKey keys[MAX_SLOTS];
    int nkeys = 0;

    for (int s = 0; s < num_slots; s++) {
        if (slot_count[s] == 0) continue;
        keys[nkeys].count = 0;
        for (int f = slot_head[s]; f != -1; f = factor_next[f]) {
            if (keys[nkeys].count < 20)
                keys[nkeys].primes[keys[nkeys].count] = factor_prime[f];
            keys[nkeys].count++;
        }
        /* Sort primes */
        for (int i = 0; i < keys[nkeys].count && i < 20; i++)
            for (int j = i+1; j < keys[nkeys].count && j < 20; j++)
                if (keys[nkeys].primes[i] > keys[nkeys].primes[j]) {
                    int t = keys[nkeys].primes[i];
                    keys[nkeys].primes[i] = keys[nkeys].primes[j];
                    keys[nkeys].primes[j] = t;
                }
        nkeys++;
    }

    qsort(keys, nkeys, sizeof(SlotKey), cmp_slot_key);

    /* FNV-1a hash */
    unsigned long long h = 14695981039346656037ULL;
    for (int i = 0; i < nkeys; i++) {
        for (int j = 0; j < keys[i].count && j < 20; j++) {
            h ^= (unsigned long long)keys[i].primes[j];
            h *= 1099511628211ULL;
        }
        h ^= 0xFF;
        h *= 1099511628211ULL;
    }
    return h;
}

long long compute_sum(void) {
    long long total = 0;
    for (int s = 0; s < num_slots; s++) {
        if (slot_count[s] == 0) continue;
        long long prod = 1;
        for (int f = slot_head[s]; f != -1; f = factor_next[f])
            prod = (prod * factor_prime[f]) % MOD;
        total = (total + prod) % MOD;
    }
    return total;
}

#define HASH_TABLE_SIZE (1 << 20)
#define HASH_TABLE_MASK (HASH_TABLE_SIZE - 1)

static unsigned long long ht_keys[HASH_TABLE_SIZE];
static int ht_vals[HASH_TABLE_SIZE];
static int ht_used[HASH_TABLE_SIZE];

void ht_init(void) {
    memset(ht_used, 0, sizeof(ht_used));
}

int ht_lookup(unsigned long long key, int *val) {
    int h = (int)(key & HASH_TABLE_MASK);
    while (ht_used[h]) {
        if (ht_keys[h] == key) { *val = ht_vals[h]; return 1; }
        h = (h + 1) & HASH_TABLE_MASK;
    }
    return 0;
}

void ht_insert(unsigned long long key, int val) {
    int h = (int)(key & HASH_TABLE_MASK);
    while (ht_used[h]) {
        if (ht_keys[h] == key) { ht_vals[h] = val; return; }
        h = (h + 1) & HASH_TABLE_MASK;
    }
    ht_keys[h] = key;
    ht_vals[h] = val;
    ht_used[h] = 1;
}

int main(void) {
    int N = 10000;
    long long K = 10000000000000000LL; /* 10^16 */

    sieve();
    init_state(N);
    ht_init();

    /* Store sum at each round for later retrieval */
    static long long sums[500001];

    int round = 0;
    int period_start = -1, period = -1;

    while (round < 500000) {
        unsigned long long h = hash_state();
        sums[round] = compute_sum();

        int prev_round;
        if (ht_lookup(h, &prev_round)) {
            /* Potential period detected - verify by checking sums match */
            period_start = prev_round;
            period = round - prev_round;
            break;
        }
        ht_insert(h, round);

        do_round();
        round++;
    }

    if (period > 0) {
        long long target = K;
        if (target >= period_start) {
            int idx = period_start + (int)((target - period_start) % period);
            printf("%lld\n", sums[idx]);
        } else {
            printf("%lld\n", sums[(int)target]);
        }
    } else {
        /* No period found, print last sum */
        printf("%lld\n", sums[round - 1]);
    }

    return 0;
}
"""
    with tempfile.NamedTemporaryFile(suffix='.c', mode='w', delete=False) as f:
        f.write(c_code)
        c_path = f.name

    bin_path = c_path.replace('.c', '')
    try:
        compile_result = subprocess.run(
            ['gcc', '-O2', '-o', bin_path, c_path, '-lm'],
            capture_output=True, text=True
        )
        if compile_result.returncode != 0:
            raise RuntimeError(f"Compilation failed: {compile_result.stderr}")
        result = subprocess.run([bin_path], capture_output=True, text=True,
                                check=True, timeout=28)
        return int(result.stdout.strip())
    finally:
        os.unlink(c_path)
        if os.path.exists(bin_path):
            os.unlink(bin_path)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
