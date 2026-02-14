#include <stdio.h>
#include <stdlib.h>

/*
 * Project Euler 876
 *
 * For k=1..18, a=6^k, b=10^k.
 * Generate divisor pairs (y,z) with y|a, z|b, gcd(y,z)=1.
 * Compute c = (y+z)*(a/y + b/z), numSteps via Euclidean algo.
 * Track min numSteps per c. Sum contributions.
 */

typedef long long ll;

ll gcd(ll a, ll b) {
    while (b) { ll t = b; b = a % b; a = t; }
    return a;
}

/* Hash map for c -> min numSteps */
#define HASH_SIZE (1 << 20)
#define HASH_MASK (HASH_SIZE - 1)

typedef struct Entry {
    ll key;
    int val;
    struct Entry *next;
} Entry;

Entry *table[HASH_SIZE];
Entry pool[5000000];
int pool_idx = 0;

void hash_clear(void) {
    for (int i = 0; i < HASH_SIZE; i++) table[i] = NULL;
    pool_idx = 0;
}

unsigned int hash_key(ll k) {
    unsigned long long h = (unsigned long long)k;
    h ^= h >> 33;
    h *= 0xff51afd7ed558ccdULL;
    h ^= h >> 33;
    return (unsigned int)(h & HASH_MASK);
}

void hash_insert(ll key, int val) {
    unsigned int h = hash_key(key);
    for (Entry *e = table[h]; e; e = e->next) {
        if (e->key == key) {
            if (val < e->val) e->val = val;
            return;
        }
    }
    Entry *e = &pool[pool_idx++];
    e->key = key;
    e->val = val;
    e->next = table[h];
    table[h] = e;
}

int main(void) {
    ll ans = 0;

    for (int k = 1; k <= 18; k++) {
        ll a = 1, b = 1;
        for (int i = 0; i < k; i++) { a *= 6; b *= 10; }

        /* Generate divisors of a = 2^k * 3^k */
        int na = (k + 1) * (k + 1);
        ll *a_divs = malloc(na * sizeof(ll));
        int a_cnt = 0;
        {
            ll pw2 = 1;
            for (int i = 0; i <= k; i++) {
                ll pw3 = 1;
                for (int j = 0; j <= k; j++) {
                    a_divs[a_cnt++] = pw2 * pw3;
                    pw3 *= 3;
                }
                pw2 *= 2;
            }
        }

        /* Generate divisors of b = 2^k * 5^k */
        int nb = (k + 1) * (k + 1);
        ll *b_divs = malloc(nb * sizeof(ll));
        int b_cnt = 0;
        {
            ll pw2 = 1;
            for (int i = 0; i <= k; i++) {
                ll pw5 = 1;
                for (int j = 0; j <= k; j++) {
                    b_divs[b_cnt++] = pw2 * pw5;
                    pw5 *= 5;
                }
                pw2 *= 2;
            }
        }

        hash_clear();

        for (int yi = 0; yi < a_cnt; yi++) {
            ll y = a_divs[yi];
            for (int zi = 0; zi < b_cnt; zi++) {
                ll z = b_divs[zi];
                if (gcd(y, z) != 1) continue;

                ll c = (y + z) * (a / y + b / z);

                /* Compute numSteps */
                ll ly = y, lz = z;
                int numSteps = 0;
                int side = 0; /* 0 = process ly, 1 = process lz */
                while (1) {
                    if (side == 0) {
                        if (ly == 0) break;
                        numSteps += (int)(lz / ly);
                        lz %= ly;
                        side = 1;
                    } else {
                        if (lz == 0) break;
                        numSteps += (int)(ly / lz);
                        ly %= lz;
                        side = 0;
                    }
                }

                hash_insert(c, numSteps);
            }
        }

        /* Sum contributions */
        for (int i = 0; i < pool_idx; i++) {
            ll c = pool[i].key;
            int numSteps = pool[i].val;
            ans += numSteps;
            if (c < 2 * (a + b)) {
                ans += numSteps - 1;
            }
        }

        free(a_divs);
        free(b_divs);
    }

    printf("%lld\n", ans);
    return 0;
}
