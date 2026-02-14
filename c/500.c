/* Project Euler 500 - Problem 500!!!
 * Translated from python/500.py
 *
 * Find smallest number with 2^500500 divisors, mod 500500507.
 * Priority queue: start with primes, each time pick smallest,
 * multiply into answer, push its square.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef long long ll;
typedef unsigned long long ull;

#define NVAL 500500
#define MOD_VAL 500500507LL
#define SIEVE_LIMIT 7800000

/* Min-heap for unsigned long long */
ull *heap;
int heap_size;

void heap_push(ull val) {
    int i = heap_size++;
    heap[i] = val;
    while (i > 0) {
        int parent = (i - 1) / 2;
        if (heap[parent] > heap[i]) {
            ull t = heap[parent]; heap[parent] = heap[i]; heap[i] = t;
            i = parent;
        } else break;
    }
}

ull heap_pop(void) {
    ull val = heap[0];
    heap[0] = heap[--heap_size];
    int i = 0;
    while (1) {
        int left = 2 * i + 1, right = 2 * i + 2, smallest = i;
        if (left < heap_size && heap[left] < heap[smallest]) smallest = left;
        if (right < heap_size && heap[right] < heap[smallest]) smallest = right;
        if (smallest == i) break;
        ull t = heap[i]; heap[i] = heap[smallest]; heap[smallest] = t;
        i = smallest;
    }
    return val;
}

int main() {
    /* Sieve primes */
    char *is_prime = calloc(SIEVE_LIMIT + 1, 1);
    memset(is_prime, 1, SIEVE_LIMIT + 1);
    is_prime[0] = is_prime[1] = 0;
    int sq = (int)sqrt((double)SIEVE_LIMIT);
    for (int i = 2; i <= sq; i++) {
        if (is_prime[i]) {
            for (int j = i * i; j <= SIEVE_LIMIT; j += i)
                is_prime[j] = 0;
        }
    }

    /* Collect primes into heap */
    heap = malloc((NVAL + NVAL) * sizeof(ull));
    heap_size = 0;

    int count = 0;
    for (int i = 2; i <= SIEVE_LIMIT && count < NVAL; i++) {
        if (is_prime[i]) {
            heap_push((ull)i);
            count++;
        }
    }
    free(is_prime);

    ll ans = 1;
    for (int i = 0; i < NVAL; i++) {
        ull v = heap_pop();
        ans = ans * (ll)(v % MOD_VAL) % MOD_VAL;
        heap_push(v * v);
    }

    printf("%lld\n", ans);
    free(heap);
    return 0;
}
