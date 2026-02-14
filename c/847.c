/*
 * Project Euler 847
 *
 * Uses iterative layer-by-layer digit DP with bit-level processing.
 * For each bit level j (from MSB down to LSB), maintain a hash map
 * of (R, carries, c_states) -> count mod MOD.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MOD 1000000007LL

/* Hash map: key = packed state, value = count mod MOD */
#define HM_CAP_BITS 20
#define HM_CAP (1 << HM_CAP_BITS)
#define HM_MASK (HM_CAP - 1)

typedef struct {
    unsigned long long key;
    long long val;
} hm_slot;

typedef struct {
    hm_slot *slots;
    int *used; /* 0 = empty, 1 = occupied */
    int count;
} hashmap;

static void hm_init(hashmap *m) {
    m->slots = calloc(HM_CAP, sizeof(hm_slot));
    m->used = calloc(HM_CAP, sizeof(int));
    m->count = 0;
}

static void hm_clear(hashmap *m) {
    if (m->count > 0) {
        memset(m->used, 0, HM_CAP * sizeof(int));
        m->count = 0;
    }
}

static void hm_free(hashmap *m) {
    free(m->slots);
    free(m->used);
}

static inline unsigned int hm_hash(unsigned long long key) {
    return (unsigned int)(key * 2654435761ULL) & HM_MASK;
}

static void hm_add(hashmap *m, unsigned long long key, long long val) {
    unsigned int h = hm_hash(key);
    for (;;) {
        int idx = h & HM_MASK;
        if (!m->used[idx]) {
            m->slots[idx].key = key;
            m->slots[idx].val = val;
            m->used[idx] = 1;
            m->count++;
            return;
        }
        if (m->slots[idx].key == key) {
            m->slots[idx].val = (m->slots[idx].val + val) % MOD;
            return;
        }
        h++;
    }
}

/* Iterate over all entries */
typedef struct {
    unsigned long long key;
    long long val;
} hm_entry;

/* Pack state: R_off (0..5), carries (0..7), c0 (0..255), c1 (0..255), c2 (0..255) */
static inline unsigned long long pack_state(int R_off, int carries, int c0, int c1, int c2) {
    unsigned long long k = (unsigned long long)R_off;
    k = k * 8 + carries;
    k = k * 256 + c0;
    k = k * 256 + c1;
    k = k * 256 + c2;
    return k;
}

static inline void unpack_state(unsigned long long k, int *R_off, int *carries, int *c0, int *c1, int *c2) {
    *c2 = k & 255; k >>= 8;
    *c1 = k & 255; k >>= 8;
    *c0 = k & 255; k >>= 8;
    *carries = k & 7; k >>= 3;
    *R_off = (int)k;
}

static long long solve_dp(long long limit, int kval, int checks[][3], int nchk) {
    int L_bits[100];
    memset(L_bits, 0, sizeof(L_bits));
    int actual_bits = 0;
    {
        long long tmp = limit;
        while (tmp > 0) {
            L_bits[actual_bits++] = (int)(tmp & 1);
            tmp >>= 1;
        }
    }
    int nb = actual_bits;
    if (kval > nb) nb = kval;

    hashmap cur, nxt;
    hm_init(&cur);
    hm_init(&nxt);

    /* Initial state: j = nb-1 (MSB), R=0 (R_off=3), carries=0, c_states = {(0,0,0)} = bitmask 1 */
    int init_cs = 1; /* bit 0 set = (0,0,0) */
    int c0_init = (nchk > 0) ? init_cs : 0;
    int c1_init = (nchk > 1) ? init_cs : 0;
    int c2_init = (nchk > 2) ? init_cs : 0;

    unsigned long long init_key = pack_state(3, 0, c0_init, c1_init, c2_init);
    hm_add(&cur, init_key, 1);

    /* Process levels from j = nb-1 down to j = 0 */
    for (int j = nb - 1; j >= 0; j--) {
        hm_clear(&nxt);
        int limit_bit = (j < actual_bits) ? L_bits[j] : 0;
        int has_source = (j < kval);

        /* Iterate over all current states */
        for (int slot = 0; slot < HM_CAP; slot++) {
            if (!cur.used[slot]) continue;
            unsigned long long skey = cur.slots[slot].key;
            long long sval = cur.slots[slot].val;
            if (sval == 0) continue;

            int R_off, carries_packed, cs0, cs1, cs2;
            unpack_state(skey, &R_off, &carries_packed, &cs0, &cs1, &cs2);
            int R = R_off - 3;
            int cA = (carries_packed >> 2) & 1;
            int cB = (carries_packed >> 1) & 1;
            int cC = carries_packed & 1;

            for (int a = 0; a < 2; a++) {
                for (int b = 0; b < 2; b++) {
                    for (int c = 0; c < 2; c++) {
                        int new_R = 2 * R + (a + b + c) - limit_bit;
                        if (new_R >= 2) continue;
                        if (new_R <= -3) new_R = -3;

                        /* Carry transitions for A */
                        int valid_ncA[2], ncA_count = 0;
                        if (cA) {
                            if (a == 1) { valid_ncA[ncA_count++] = 1; }
                            else continue;
                        } else {
                            if (a == 0) { valid_ncA[ncA_count++] = 0; valid_ncA[ncA_count++] = 1; }
                            else { valid_ncA[ncA_count++] = 0; }
                        }

                        int valid_ncB[2], ncB_count = 0;
                        if (cB) {
                            if (b == 1) { valid_ncB[ncB_count++] = 1; }
                            else continue;
                        } else {
                            if (b == 0) { valid_ncB[ncB_count++] = 0; valid_ncB[ncB_count++] = 1; }
                            else { valid_ncB[ncB_count++] = 0; }
                        }

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

                                    int possible = 1;
                                    int new_cs[3] = {0, 0, 0};

                                    for (int idx = 0; idx < nchk; idx++) {
                                        int bA = checks[idx][0] ? bitA1 : bitA;
                                        int bB = checks[idx][1] ? bitB1 : bitB;
                                        int bC = checks[idx][2] ? bitC1 : bitC;

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

                                                if (owner == 0) {
                                                    if (psA == 0 && bA == 0) nsA = 1;
                                                } else {
                                                    if (psA == 0 && bA == 1) valid_owner = 0;
                                                }
                                                if (!valid_owner) continue;

                                                if (owner == 1) {
                                                    if (psB == 0 && bB == 0) nsB = 1;
                                                } else {
                                                    if (psB == 0 && bB == 1) valid_owner = 0;
                                                }
                                                if (!valid_owner) continue;

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
                                        int new_R_off = new_R + 3;
                                        int new_carries = (ncA << 2) | (ncB << 1) | ncC;
                                        int nc0 = (nchk > 0) ? new_cs[0] : 0;
                                        int nc1 = (nchk > 1) ? new_cs[1] : 0;
                                        int nc2 = (nchk > 2) ? new_cs[2] : 0;

                                        unsigned long long nkey = pack_state(new_R_off, new_carries, nc0, nc1, nc2);
                                        hm_add(&nxt, nkey, sval);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        /* Swap cur and nxt */
        hashmap tmp = cur;
        cur = nxt;
        nxt = tmp;
    }

    /* Sum over final states where all carries are set and R <= 0 */
    long long result = 0;
    for (int slot = 0; slot < HM_CAP; slot++) {
        if (!cur.used[slot]) continue;
        int R_off, carries_packed, cs0, cs1, cs2;
        unpack_state(cur.slots[slot].key, &R_off, &carries_packed, &cs0, &cs1, &cs2);
        int R = R_off - 3;
        int cA = (carries_packed >> 2) & 1;
        int cB = (carries_packed >> 1) & 1;
        int cC = carries_packed & 1;
        if (cA && cB && cC && R <= 0) {
            result = (result + cur.slots[slot].val) % MOD;
        }
    }

    hm_free(&cur);
    hm_free(&nxt);
    return result;
}

static long long mod_pow(long long base, long long exp, long long mod) {
    long long r = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) r = r * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return r;
}

static long long C_val_mod(long long n) {
    if (n < 0) return 0;
    long long a = (n + 3) % MOD;
    long long b = (n + 2) % MOD;
    long long c = (n + 1) % MOD;
    long long inv6 = mod_pow(6, MOD - 2, MOD);
    long long val = a * b % MOD * c % MOD * inv6 % MOD;
    val = (val - 1 + MOD) % MOD;
    return val;
}

int main(void) {
    long long N = 0;
    {
        long long p = 1;
        for (int i = 0; i < 19; i++) {
            N += p;
            p *= 10;
        }
    }

    long long C_N_mod = C_val_mod(N);

    long long total_H = 0;
    int k = 0;

    while (1) {
        int chk1[1][3] = {{0, 0, 0}};
        long long t1 = solve_dp(N - 1, k, chk1, 1);

        int chk2[2][3] = {{0, 1, 0}, {1, 0, 0}};
        long long t2 = solve_dp(N - 2, k, chk2, 2);

        int chk3[3][3] = {{0, 1, 1}, {1, 0, 1}, {1, 1, 0}};
        long long t3 = solve_dp(N - 3, k, chk3, 3);

        long long size_Sk = ((3 * t1 % MOD - 3 * t2 % MOD + t3 % MOD) % MOD + MOD) % MOD;
        long long term = (C_N_mod - size_Sk % MOD + MOD) % MOD;

        if (term == 0)
            break;

        total_H = (total_H + term) % MOD;
        k++;
        if (k > 100) break;
    }

    printf("%lld\n", total_H);
    return 0;
}
