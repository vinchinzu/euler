/* Project Euler Problem 918 - Recursive Sequence
 * a_1=1, a_(2n)=2*a_n, a_(2n+1)=a_n - 3*a_(n+1)
 * S(N) = sum a_n for n=1..N
 * For N>=10, S(N) = 4 - a(N/2)
 * a_k computed recursively with memoization using hash table.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;

/* Simple hash map for memoization: key=ll, value=ll */
#define HASH_SIZE (1 << 22) /* 4M buckets */
#define HASH_MASK (HASH_SIZE - 1)

typedef struct Entry {
    ll key;
    ll val;
    struct Entry *next;
} Entry;

static Entry *table[HASH_SIZE];

/* Pool allocator for entries */
#define POOL_SIZE (1 << 22)
static Entry pool[POOL_SIZE];
static int pool_idx = 0;

static Entry *alloc_entry(void) {
    if (pool_idx >= POOL_SIZE) {
        fprintf(stderr, "Pool exhausted\n");
        exit(1);
    }
    return &pool[pool_idx++];
}

static ll *lookup(ll key) {
    unsigned int h = (unsigned int)((unsigned long long)key * 2654435761ULL) & HASH_MASK;
    Entry *e = table[h];
    while (e) {
        if (e->key == key) return &e->val;
        e = e->next;
    }
    return NULL;
}

static void insert(ll key, ll val) {
    unsigned int h = (unsigned int)((unsigned long long)key * 2654435761ULL) & HASH_MASK;
    Entry *e = alloc_entry();
    e->key = key;
    e->val = val;
    e->next = table[h];
    table[h] = e;
}

/* Iterative computation of a_k using explicit stack to avoid stack overflow */
/* Stack-based approach: resolve dependencies iteratively */
static ll get_a(ll k) {
    /* Check memo first */
    ll *p = lookup(k);
    if (p) return *p;

    /* Use explicit stack */
    #define STACK_SIZE (1 << 20)
    static ll stack[STACK_SIZE];
    int sp = 0;
    stack[sp++] = k;

    while (sp > 0) {
        ll cur = stack[sp - 1];
        ll *cp = lookup(cur);
        if (cp) {
            sp--;
            continue;
        }
        if (cur == 1) {
            insert(1, 1);
            sp--;
            continue;
        }
        if (cur % 2 == 0) {
            ll half = cur / 2;
            ll *hp = lookup(half);
            if (!hp) {
                stack[sp++] = half;
                continue;
            }
            insert(cur, 2 * (*hp));
            sp--;
        } else {
            ll m = (cur - 1) / 2;
            ll *mp = lookup(m);
            ll *mp1 = lookup(m + 1);
            if (!mp) { stack[sp++] = m; }
            if (!mp1) { stack[sp++] = m + 1; }
            if (mp && mp1) {
                insert(cur, (*mp) - 3 * (*mp1));
                sp--;
            }
        }
    }

    p = lookup(k);
    return *p;
}

int main(void) {
    ll N = 1000000000000LL; /* 10^12 */

    memset(table, 0, sizeof(table));

    ll a_val = get_a(N / 2);
    ll result = 4 - a_val;
    printf("%lld\n", result);
    return 0;
}
