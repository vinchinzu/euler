/*
 * Project Euler 626: Counting Binary Matrices
 *
 * Count equivalence classes of binary NxN matrices under row/column swaps
 * and row/column flips, using Burnside's Lemma.
 *
 * N=20, M=1001001011.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef __int128 lll;

#define N 20
#define M 1001001011LL

ll powmod(ll base, ll exp, ll mod) {
    ll result = 1;
    base = ((base % mod) + mod) % mod;
    while (exp > 0) {
        if (exp & 1) result = (lll)result * base % mod;
        base = (lll)base * base % mod;
        exp >>= 1;
    }
    return result;
}

ll modinv(ll a, ll mod) {
    return powmod(a, mod - 2, mod);
}

ll gcd(ll a, ll b) {
    while (b) { ll t = b; b = a % b; a = t; }
    return a;
}

/* Partition representation: parts[0..nparts-1] are the part sizes */
/* We enumerate all partitions of N */

#define MAX_PARTS 20
#define MAX_PARTITIONS 1000

typedef struct {
    int parts[MAX_PARTS];
    int counts[MAX_PARTS]; /* count of each distinct part */
    int ndistinct;
    int nparts;
} Partition;

Partition partitions[MAX_PARTITIONS];
int num_partitions = 0;

void gen_partitions(int remaining, int max_val, int parts[], int nparts) {
    if (remaining == 0) {
        Partition *p = &partitions[num_partitions];
        p->nparts = nparts;
        /* Compute counts of each distinct part */
        p->ndistinct = 0;
        for (int i = 0; i < nparts; i++) {
            p->parts[i] = parts[i];
        }
        /* Count distinct parts */
        int sorted[MAX_PARTS];
        memcpy(sorted, parts, nparts * sizeof(int));
        /* parts are already in non-increasing order from generation */
        p->ndistinct = 0;
        for (int i = 0; i < nparts; ) {
            int val = sorted[i];
            int cnt = 0;
            while (i < nparts && sorted[i] == val) { cnt++; i++; }
            p->counts[p->ndistinct] = cnt;
            p->parts[p->ndistinct] = val; /* store distinct part values */
            p->ndistinct++;
        }
        num_partitions++;
        return;
    }
    for (int i = (max_val < remaining) ? max_val : remaining; i >= 1; i--) {
        parts[nparts] = i;
        gen_partitions(remaining - i, i, parts, nparts + 1);
    }
}

ll factorial_mod(int n) {
    ll r = 1;
    for (int i = 2; i <= n; i++) r = (lll)r * i % M;
    return r;
}

/* Number of permutations with given cycle type */
ll num_arrangements(Partition *p) {
    ll result = factorial_mod(N);
    for (int i = 0; i < p->ndistinct; i++) {
        int size = p->parts[i];
        int count = p->counts[i];
        ll inv_size = modinv(size, M);
        for (int j = 0; j < count; j++)
            result = (lll)result * inv_size % M;
        result = (lll)result * modinv(factorial_mod(count), M) % M;
    }
    return result;
}

/* Count restricted rows */
int num_restricted_rows(Partition *perm, Partition *other) {
    int count = 0;
    for (int i = 0; i < perm->ndistinct; i++) {
        int size = perm->parts[i];
        int found = 0;
        for (int j = 0; j < other->ndistinct; j++) {
            int other_size = other->parts[j];
            if ((other_size / gcd(size, other_size)) % 2 == 1) {
                found = 1;
                break;
            }
        }
        if (found) count += perm->counts[i];
    }
    return count;
}

/* Total number of parts in partition */
int total_parts(Partition *p) {
    int s = 0;
    for (int i = 0; i < p->ndistinct; i++) s += p->counts[i];
    return s;
}

int main() {
    int parts_buf[MAX_PARTS];
    gen_partitions(N, N, parts_buf, 0);

    ll ans = 0;

    for (int i = 0; i < num_partitions; i++) {
        for (int j = 0; j < num_partitions; j++) {
            Partition *p1 = &partitions[i];
            Partition *p2 = &partitions[j];

            /* Compute num_grid_cycles */
            ll num_grid_cycles = 0;
            for (int a = 0; a < p1->ndistinct; a++) {
                for (int b = 0; b < p2->ndistinct; b++) {
                    num_grid_cycles += gcd(p1->parts[a], p2->parts[b])
                        * (ll)p1->counts[a] * p2->counts[b];
                }
            }

            int nr1 = num_restricted_rows(p1, p2);
            int nr2 = num_restricted_rows(p2, p1);
            int tp1 = total_parts(p1);
            int tp2 = total_parts(p2);
            int all_restricted = (nr1 == tp1 && nr2 == tp2);

            ll term = (lll)num_arrangements(p1) * num_arrangements(p2) % M;
            term = (lll)term * powmod(2, num_grid_cycles, M) % M;
            int exp2 = 2 * N - nr1 - nr2 - (all_restricted ? 0 : 1);
            term = (lll)term * powmod(2, exp2, M) % M;

            ans = (ans + term) % M;
        }
    }

    ll inv_fact_n = modinv(factorial_mod(N), M);
    ll inv_2_pow = modinv(powmod(2, 2 * N - 1, M), M);
    ans = (lll)ans * inv_fact_n % M;
    ans = (lll)ans * inv_fact_n % M;
    ans = (lll)ans * inv_2_pow % M;

    printf("%lld\n", ans);
    return 0;
}
