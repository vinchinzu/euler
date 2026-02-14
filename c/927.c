/* Project Euler Problem 927 - Prime Trees (R(N))
 * S = intersection of S_p over all primes p.
 * A number m is in S if for all prime factors p of phi(m),
 * the map x -> 1 + x^p mod m reaches 0 from 1.
 * R(N) = sum of elements of S not exceeding N.
 *
 * Two-phase: 1) find primes in S using reachability check
 *            2) generate all composites from those primes, checking each
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

typedef long long ll;
typedef unsigned long long ull;

int N_limit;
int *spf;
int *visited;
int current_generation = 0;

void sieve(int n) {
    for (int i = 0; i <= n; i++) spf[i] = i;
    for (int i = 2; (ll)i * i <= n; i++) {
        if (spf[i] == i) {
            for (int j = i * i; j <= n; j += i)
                if (spf[j] == j)
                    spf[j] = i;
        }
    }
}

ull power_mod(ull base, ull exp, ull mod) {
    ull res = 1;
    base %= mod;
    while (exp > 0) {
        if (exp % 2 == 1) res = (unsigned __int128)res * base % mod;
        base = (unsigned __int128)base * base % mod;
        exp /= 2;
    }
    return res;
}

int check_reachability(int m, int exponent) {
    current_generation++;
    ull x = 1;
    while (1) {
        if (visited[x] == current_generation) return 0;
        visited[x] = current_generation;
        if (x == 0) return 1;
        x = (1 + power_mod(x, exponent, m)) % m;
        if (x == 0) return 1;
    }
}

int is_prime_in_S(int q) {
    if (q == 2) return 1;
    int phi = q - 1;
    if (phi % 2 == 0) {
        if (!check_reachability(q, 2)) return 0;
        while (phi % 2 == 0) phi /= 2;
    }
    while (phi > 1) {
        int p = spf[phi];
        if (!check_reachability(q, p)) return 0;
        while (phi % p == 0) phi /= p;
    }
    return 1;
}

/* Get prime factors of n */
int get_prime_factors(int n, int *factors) {
    int cnt = 0;
    int d = 2;
    while ((ll)d * d <= n) {
        if (n % d == 0) {
            factors[cnt++] = d;
            while (n % d == 0) n /= d;
        }
        d++;
    }
    if (n > 1) factors[cnt++] = n;
    return cnt;
}

int check_reachability_gen(int m, int exponent) {
    /* Generic reachability check for composite m */
    current_generation++;
    if (current_generation < 0) {
        memset(visited, 0, (N_limit + 1) * sizeof(int));
        current_generation = 1;
    }
    ull x = 1;
    while (1) {
        if ((int)x < N_limit + 1 && visited[x] == current_generation) return 0;
        if ((int)x < N_limit + 1) visited[x] = current_generation;
        if (x == 0) return 1;
        x = (1 + power_mod(x, exponent, m)) % m;
        if (x == 0) return 1;
    }
}

int is_composite_in_S(int m) {
    /* Get prime factors of m */
    int factors_m[64];
    int nfm = get_prime_factors(m, factors_m);

    /* Get prime factors of phi(m) */
    int phi_factors[128];
    int nphi = 0;

    for (int i = 0; i < nfm; i++) {
        int p = factors_m[i];
        /* Add prime factors of p-1 */
        int pf[64];
        int npf = get_prime_factors(p - 1, pf);
        for (int j = 0; j < npf; j++) {
            /* Check if already in phi_factors */
            int found = 0;
            for (int k = 0; k < nphi; k++)
                if (phi_factors[k] == pf[j]) { found = 1; break; }
            if (!found) phi_factors[nphi++] = pf[j];
        }
        /* Check if p^2 divides m */
        int temp = m;
        int count = 0;
        while (temp % p == 0) { count++; temp /= p; }
        if (count >= 2) {
            int found = 0;
            for (int k = 0; k < nphi; k++)
                if (phi_factors[k] == p) { found = 1; break; }
            if (!found) phi_factors[nphi++] = p;
        }
    }

    for (int i = 0; i < nphi; i++) {
        if (!check_reachability_gen(m, phi_factors[i])) return 0;
    }
    return 1;
}

/* Min-heap for generating numbers in S */
#define HEAP_SIZE (1 << 22)
ll heap[HEAP_SIZE];
int heap_size = 0;

void heap_push(ll val) {
    if (heap_size >= HEAP_SIZE) return;
    int i = heap_size++;
    heap[i] = val;
    while (i > 0) {
        int parent = (i - 1) / 2;
        if (heap[parent] > heap[i]) {
            ll tmp = heap[parent]; heap[parent] = heap[i]; heap[i] = tmp;
            i = parent;
        } else break;
    }
}

ll heap_pop(void) {
    ll val = heap[0];
    heap[0] = heap[--heap_size];
    int i = 0;
    while (1) {
        int left = 2 * i + 1, right = 2 * i + 2, smallest = i;
        if (left < heap_size && heap[left] < heap[smallest]) smallest = left;
        if (right < heap_size && heap[right] < heap[smallest]) smallest = right;
        if (smallest != i) {
            ll tmp = heap[i]; heap[i] = heap[smallest]; heap[smallest] = tmp;
            i = smallest;
        } else break;
    }
    return val;
}

/* Hash set to avoid duplicates in heap */
#define SEEN_SIZE (1 << 24)
#define SEEN_MASK (SEEN_SIZE - 1)
typedef struct SEntry { ll key; struct SEntry *next; } SEntry;
SEntry *seen_table[SEEN_SIZE];
#define SPOOL_SIZE (1 << 23)
SEntry spool[SPOOL_SIZE];
int spool_idx = 0;

int seen_contains(ll key) {
    unsigned int h = (unsigned int)((ull)key * 2654435761ULL) & SEEN_MASK;
    SEntry *e = seen_table[h];
    while (e) { if (e->key == key) return 1; e = e->next; }
    return 0;
}

void seen_add(ll key) {
    unsigned int h = (unsigned int)((ull)key * 2654435761ULL) & SEEN_MASK;
    if (spool_idx >= SPOOL_SIZE) return;
    SEntry *e = &spool[spool_idx++];
    e->key = key;
    e->next = seen_table[h];
    seen_table[h] = e;
}

int main(void) {
    N_limit = 10000000;

    spf = (int *)malloc((N_limit + 1) * sizeof(int));
    visited = (int *)calloc(N_limit + 1, sizeof(int));

    sieve(N_limit);

    /* Phase 1: Find primes in S */
    int *s_primes = (int *)malloc(1000000 * sizeof(int));
    int ns_primes = 0;

    for (int q = 2; q <= N_limit; q++) {
        if (spf[q] == q) { /* q is prime */
            if (is_prime_in_S(q)) {
                s_primes[ns_primes++] = q;
            }
        }
    }

    /* Phase 2: Generate all numbers in S up to N_limit */
    memset(seen_table, 0, sizeof(seen_table));

    heap_push(1);
    seen_add(1);

    ll total_sum = 0;

    while (heap_size > 0) {
        ll curr = heap_pop();

        /* Check if curr is in S */
        int in_s = 0;
        if (curr == 1) {
            in_s = 1;
        } else if (curr <= N_limit && spf[(int)curr] == (int)curr) {
            /* curr is prime, already verified */
            in_s = 1;
        } else {
            /* Composite: check */
            if (curr <= N_limit)
                in_s = is_composite_in_S((int)curr);
        }

        if (in_s) {
            total_sum += curr;
        }

        for (int i = 0; i < ns_primes; i++) {
            ll nxt = curr * s_primes[i];
            if (nxt > N_limit) break;
            if (!seen_contains(nxt)) {
                seen_add(nxt);
                heap_push(nxt);
            }
        }
    }

    printf("%lld\n", total_sum);

    free(spf);
    free(visited);
    free(s_primes);
    return 0;
}
