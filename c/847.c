/*
 * Project Euler 847
 *
 * Uses digit DP with bit-level processing.
 * Counting triples (A,B,C) with various carry/check constraints.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MOD 1000000007LL

/*
 * State: (j, R, carries, c_states)
 * j: current bit position (processing MSB to LSB), -1 = done
 * R: excess value, range [-3, 2] -> offset by 3, so 0..5
 * carries: (cA, cB, cC), 3 bits -> 0..7
 * c_states: for each check, a frozenset of (psA, psB, psC) triples
 *           Each ps is 0 or 1, so triple is 3 bits -> 0..7.
 *           A frozenset is a bitmask of 8 possible triples -> 0..255.
 *           For up to 3 checks, c_states is 3 bytes packed into 24 bits.
 *
 * We use a hash map for memoization.
 */

/* Global parameters for current DP call */
static int num_bits_g;
static int L_bits_g[100]; /* binary digits of limit, LSB first */
static int K_g;
static int num_checks_g;
static int checks_g[3][3]; /* checks_g[i] = {flagA, flagB, flagC} */

/* Hash map for memoization */
#define HASH_SIZE (1 << 22) /* 4M entries */
#define HASH_MASK (HASH_SIZE - 1)

typedef struct {
    unsigned long long key;
    long long val;
    int used;
} hash_entry;

static hash_entry htable[HASH_SIZE];

static void hash_clear(void) {
    memset(htable, 0, sizeof(htable));
}

static unsigned long long pack_state(int j, int R_off, int carries, int c0, int c1, int c2) {
    /* j: 0..100, R_off: 0..5, carries: 0..7, c0,c1,c2: 0..255 */
    unsigned long long k = 0;
    k = (unsigned long long)(j + 1); /* 0..101 */
    k = k * 6 + R_off;
    k = k * 8 + carries;
    k = k * 256 + c0;
    k = k * 256 + c1;
    k = k * 256 + c2;
    return k;
}

static long long hash_get(unsigned long long key, int *found) {
    unsigned int h = (unsigned int)(key ^ (key >> 22)) & HASH_MASK;
    for (int i = 0; i < 16; i++) {
        int idx = (h + i) & HASH_MASK;
        if (!htable[idx].used) { *found = 0; return 0; }
        if (htable[idx].key == key) { *found = 1; return htable[idx].val; }
    }
    *found = 0;
    return 0;
}

static void hash_set(unsigned long long key, long long val) {
    unsigned int h = (unsigned int)(key ^ (key >> 22)) & HASH_MASK;
    for (int i = 0; i < 16; i++) {
        int idx = (h + i) & HASH_MASK;
        if (!htable[idx].used || htable[idx].key == key) {
            htable[idx].key = key;
            htable[idx].val = val;
            htable[idx].used = 1;
            return;
        }
    }
    /* Evict first slot */
    int idx = h & HASH_MASK;
    htable[idx].key = key;
    htable[idx].val = val;
    htable[idx].used = 1;
}

static long long dp(int j, int R, int cA, int cB, int cC, int cs0, int cs1, int cs2) {
    if (j == -1) {
        if (cA && cB && cC && R <= 0)
            return 1;
        return 0;
    }

    int R_off = R + 3; /* offset: R in [-3,2] -> [0,5] */
    int carries = (cA << 2) | (cB << 1) | cC;
    int c0 = (num_checks_g > 0) ? cs0 : 0;
    int c1 = (num_checks_g > 1) ? cs1 : 0;
    int c2 = (num_checks_g > 2) ? cs2 : 0;

    unsigned long long key = pack_state(j, R_off, carries, c0, c1, c2);
    int found;
    long long cached = hash_get(key, &found);
    if (found) return cached;

    long long res = 0;
    int limit_bit = (j < num_bits_g) ? L_bits_g[j] : 0;

    for (int a = 0; a < 2; a++) {
        for (int b = 0; b < 2; b++) {
            for (int c = 0; c < 2; c++) {
                int sum_val = a + b + c;
                int new_R = 2 * R + sum_val - limit_bit;
                if (new_R >= 2) continue;
                if (new_R <= -3) new_R = -3;

                /* Update carries for A */
                int valid_ncA[2], ncA_count = 0;
                if (cA) {
                    if (a == 1) { valid_ncA[ncA_count++] = 1; }
                    else continue;
                } else {
                    if (a == 0) { valid_ncA[ncA_count++] = 0; valid_ncA[ncA_count++] = 1; }
                    else { valid_ncA[ncA_count++] = 0; }
                }

                /* Update carries for B */
                int valid_ncB[2], ncB_count = 0;
                if (cB) {
                    if (b == 1) { valid_ncB[ncB_count++] = 1; }
                    else continue;
                } else {
                    if (b == 0) { valid_ncB[ncB_count++] = 0; valid_ncB[ncB_count++] = 1; }
                    else { valid_ncB[ncB_count++] = 0; }
                }

                /* Update carries for C */
                int valid_ncC[2], ncC_count = 0;
                if (cC) {
                    if (c == 1) { valid_ncC[ncC_count++] = 1; }
                    else continue;
                } else {
                    if (c == 0) { valid_ncC[ncC_count++] = 0; valid_ncC[ncC_count++] = 1; }
                    else { valid_ncC[ncC_count++] = 0; }
                }

                for (int iA = 0; iA < ncA_count; iA++) {
                    int ncA = valid_ncA[iA];
                    int bitA = a;
                    int bitA1 = a + ncA - 2 * cA;
                    for (int iB = 0; iB < ncB_count; iB++) {
                        int ncB = valid_ncB[iB];
                        int bitB = b;
                        int bitB1 = b + ncB - 2 * cB;
                        for (int iC = 0; iC < ncC_count; iC++) {
                            int ncC = valid_ncC[iC];
                            int bitC = c;
                            int bitC1 = c + ncC - 2 * cC;

                            int has_source = (j < K_g);
                            int possible = 1;
                            int new_cs[3] = {0, 0, 0};

                            for (int idx = 0; idx < num_checks_g; idx++) {
                                int bA = checks_g[idx][0] ? bitA1 : bitA;
                                int bB = checks_g[idx][1] ? bitB1 : bitB;
                                int bC = checks_g[idx][2] ? bitC1 : bitC;

                                int prev_states;
                                if (idx == 0) prev_states = cs0;
                                else if (idx == 1) prev_states = cs1;
                                else prev_states = cs2;

                                int current_possible = 0;

                                for (int ps = 0; ps < 8; ps++) {
                                    if (!(prev_states & (1 << ps))) continue;
                                    int psA = (ps >> 2) & 1;
                                    int psB = (ps >> 1) & 1;
                                    int psC = ps & 1;

                                    int max_owner = has_source ? 3 : 1;
                                    for (int oi = 0; oi < max_owner; oi++) {
                                        int owner = has_source ? oi : -1;
                                        int valid_owner = 1;
                                        int nsA = psA, nsB = psB, nsC = psC;

                                        /* Check A */
                                        if (owner == 0) {
                                            if (psA == 0 && bA == 0) nsA = 1;
                                        } else {
                                            if (psA == 0 && bA == 1) valid_owner = 0;
                                        }
                                        if (!valid_owner) continue;

                                        /* Check B */
                                        if (owner == 1) {
                                            if (psB == 0 && bB == 0) nsB = 1;
                                        } else {
                                            if (psB == 0 && bB == 1) valid_owner = 0;
                                        }
                                        if (!valid_owner) continue;

                                        /* Check C */
                                        if (owner == 2) {
                                            if (psC == 0 && bC == 0) nsC = 1;
                                        } else {
                                            if (psC == 0 && bC == 1) valid_owner = 0;
                                        }
                                        if (!valid_owner) continue;

                                        int ns = (nsA << 2) | (nsB << 1) | nsC;
                                        current_possible |= (1 << ns);
                                    }
                                }

                                if (!current_possible) { possible = 0; break; }
                                new_cs[idx] = current_possible;
                            }

                            if (possible) {
                                res += dp(j - 1, new_R, ncA, ncB, ncC,
                                          new_cs[0], new_cs[1], new_cs[2]);
                            }
                        }
                    }
                }
            }
        }
    }

    hash_set(key, res);
    return res;
}

static long long solve_dp(long long limit, int k, int checks[][3], int nchk) {
    num_bits_g = 0;
    {
        long long tmp = limit;
        while (tmp > 0) {
            L_bits_g[num_bits_g++] = (int)(tmp & 1);
            tmp >>= 1;
        }
    }
    int nb = num_bits_g;
    if (k > nb) nb = k;
    num_bits_g = nb;
    /* Pad L_bits with 0s */

    K_g = k;
    num_checks_g = nchk;
    for (int i = 0; i < nchk; i++) {
        checks_g[i][0] = checks[i][0];
        checks_g[i][1] = checks[i][1];
        checks_g[i][2] = checks[i][2];
    }

    hash_clear();

    int init_cs = 1; /* frozenset({(0,0,0)}) = bit 0 set */
    return dp(nb - 1, 0, 0, 0, 0, init_cs, init_cs, init_cs);
}

static long long C_val(long long n) {
    if (n < 0) return 0;
    return (n + 3) * (n + 2) * (n + 1) / 6 - 1;
}

int main(void) {
    /* N = 1111111111111111111 (19 ones) */
    long long N = 0;
    {
        long long p = 1;
        for (int i = 0; i < 19; i++) {
            N += p;
            p *= 10;
        }
    }

    long long total_H = 0;
    int k = 0;

    while (1) {
        int chk1[1][3] = {{0, 0, 0}};
        long long t1 = solve_dp(N - 1, k, chk1, 1);

        int chk2[2][3] = {{0, 1, 0}, {1, 0, 0}};
        long long t2 = solve_dp(N - 2, k, chk2, 2);

        int chk3[3][3] = {{0, 1, 1}, {1, 0, 1}, {1, 1, 0}};
        long long t3 = solve_dp(N - 3, k, chk3, 3);

        long long size_Sk = 3 * t1 - 3 * t2 + t3;
        long long term = C_val(N) - size_Sk;

        if (term == 0)
            break;

        total_H = (total_H + term % MOD) % MOD;
        k++;
        if (k > 100) break;
    }

    printf("%lld\n", (total_H % MOD + MOD) % MOD);
    return 0;
}
