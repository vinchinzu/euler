/*
 * Project Euler 858 - G(N) computation
 *
 * Computes G(800) using smooth numbers, prime weight recursion, and bitmask DP.
 * Smooth numbers (only small prime factors) with up to 322 entries require
 * multi-word bitmask handling.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MOD 1000000007LL
#define N 800

/* Bitmask with 6 * 64 = 384 bits, enough for 322 */
#define BM_WORDS 6
typedef struct { unsigned long long w[BM_WORDS]; } bitmask;

static bitmask bm_zero(void) {
    bitmask b; memset(&b, 0, sizeof(b)); return b;
}

static bitmask bm_full(int nbits) {
    bitmask b = bm_zero();
    for (int i = 0; i < nbits; i++)
        b.w[i / 64] |= 1ULL << (i % 64);
    return b;
}

static void bm_set(bitmask *b, int i) { b->w[i/64] |= 1ULL << (i%64); }
static int bm_test(const bitmask *b, int i) { return (b->w[i/64] >> (i%64)) & 1; }

static bitmask bm_and(bitmask a, bitmask b) {
    bitmask r;
    for (int i = 0; i < BM_WORDS; i++) r.w[i] = a.w[i] & b.w[i];
    return r;
}

static int bm_popcount(bitmask b) {
    int c = 0;
    for (int i = 0; i < BM_WORDS; i++)
        c += __builtin_popcountll(b.w[i]);
    return c;
}

/* Extract popcount of first 'lim' bits */
static int bm_popcount_first(bitmask b, int lim) {
    int c = 0;
    for (int i = 0; i < lim; i++)
        if (bm_test(&b, i)) c++;
    return c;
}

static long long mod_pow(long long base, long long exp, long long mod) {
    long long r = 1;
    base %= mod; if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) r = r * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return r;
}

static long long modinv(long long a) { return mod_pow(a, MOD - 2, MOD); }

/* Primes */
static int primes[200], nprimes;
static int small_primes[20], nsmall;
static int large_primes[200], nlarge;
static int is_prime_arr[N + 1];

/* Smooth numbers */
static int smooths[400];
static int num_smooths;

/* W_p values */
static long long W_vals[N + 1]; /* W_vals[p] for prime p */
static long long C_val;

/* Masks for small primes */
static bitmask sp_masks[20][20]; /* sp_masks[sp_idx][k] */
static int sp_max_e[20];

/* Options for small primes */
typedef struct { int k; long long weight; } option;
static option sp_options[20][20]; /* sp_options[sp_idx][opt_idx] */
static int sp_nopts[20];

/* Group lookups for large primes */
typedef struct {
    int limit; /* bisect_right(smooths, K) */
    long long table[400]; /* table[cnt] for cnt = 0..limit */
} group_lookup;
static group_lookup grp_lookups[200];
static int ngrp_lookups;

/* Powers of 2 */
static long long pow2_all[400];

/* Recursion */
static long long total_sum;

static void recurse(int idx, bitmask current_mask, long long current_weight) {
    if (idx == nsmall) {
        long long term = current_weight;

        /* Factor from smooth numbers (2^M) */
        int M = bm_popcount(current_mask);
        term = term * pow2_all[M] % MOD;

        /* Factors from large groups */
        for (int g = 0; g < ngrp_lookups; g++) {
            int lim = grp_lookups[g].limit;
            int cnt = bm_popcount_first(current_mask, lim);
            term = term * grp_lookups[g].table[cnt] % MOD;
        }

        total_sum = (total_sum + term) % MOD;
        return;
    }

    for (int oi = 0; oi < sp_nopts[idx]; oi++) {
        int k = sp_options[idx][oi].k;
        long long w = sp_options[idx][oi].weight;
        bitmask new_mask = bm_and(current_mask, sp_masks[idx][k]);
        long long new_weight = current_weight * w % MOD;
        recurse(idx + 1, new_mask, new_weight);
    }
}

static void generate_smooth(int idx, long long current) {
    if (idx == nsmall) {
        if (current <= N)
            smooths[num_smooths++] = (int)current;
        return;
    }
    int p = small_primes[idx];
    long long pe = 1;
    while (current * pe <= N) {
        generate_smooth(idx + 1, current * pe);
        pe *= p;
    }
}

static int bisect_right_smooths(int val) {
    int lo = 0, hi = num_smooths;
    while (lo < hi) {
        int mid = (lo + hi) / 2;
        if (smooths[mid] <= val) lo = mid + 1; else hi = mid;
    }
    return lo;
}

static int cmp_int(const void *a, const void *b) {
    return *(const int *)a - *(const int *)b;
}

int main(void) {
    /* Sieve */
    memset(is_prime_arr, 1, sizeof(is_prime_arr));
    is_prime_arr[0] = is_prime_arr[1] = 0;
    for (int p = 2; p * p <= N; p++)
        if (is_prime_arr[p])
            for (int i = p * p; i <= N; i += p)
                is_prime_arr[i] = 0;

    nprimes = nsmall = nlarge = 0;
    int sqrt_N = 28; /* floor(sqrt(800)) = 28 */
    for (int p = 2; p <= N; p++) {
        if (!is_prime_arr[p]) continue;
        primes[nprimes++] = p;
        if (p <= sqrt_N) small_primes[nsmall++] = p;
        else large_primes[nlarge++] = p;
    }

    /* Generate smooth numbers */
    num_smooths = 0;
    generate_smooth(0, 1);
    qsort(smooths, num_smooths, sizeof(int), cmp_int);

    /* W_vals */
    for (int i = 0; i < nprimes; i++) {
        int p = primes[i];
        long long pe = 1;
        while (pe * p <= N) pe *= p;
        W_vals[p] = pe;
    }

    /* C = product W_p mod MOD */
    C_val = 1;
    for (int i = 0; i < nprimes; i++)
        C_val = C_val * W_vals[primes[i]] % MOD;

    /* Weight function */
    /* get_weight(p, k): k==0 -> 1; else -(p^k - p^(k-1)) / W_p mod MOD */

    /* Prepare small prime options and masks */
    for (int si = 0; si < nsmall; si++) {
        int p = small_primes[si];

        /* max exponent */
        int max_e = 0;
        long long pe = 1;
        while (pe <= N) { pe *= p; max_e++; }
        max_e--;
        sp_max_e[si] = max_e;

        /* Masks */
        /* k=0: all bits set */
        sp_masks[si][0] = bm_full(num_smooths);

        for (int k = 1; k <= max_e; k++) {
            bitmask m = bm_zero();
            for (int i = 0; i < num_smooths; i++) {
                int y = smooths[i];
                int v = 0;
                int temp = y;
                while (temp > 0 && temp % p == 0) { v++; temp /= p; }
                if (v < k) bm_set(&m, i);
            }
            sp_masks[si][k] = m;
        }

        /* Options */
        sp_nopts[si] = 0;
        /* k=0 */
        sp_options[si][sp_nopts[si]].k = 0;
        sp_options[si][sp_nopts[si]].weight = 1;
        sp_nopts[si]++;

        pe = p;
        for (int k = 1; k <= max_e; k++) {
            long long phi = (mod_pow(p, k, MOD) - mod_pow(p, k - 1, MOD) + MOD) % MOD;
            long long num = (MOD - phi) % MOD;
            long long w = num * modinv(W_vals[p]) % MOD;
            sp_options[si][sp_nopts[si]].k = k;
            sp_options[si][sp_nopts[si]].weight = w;
            sp_nopts[si]++;
            pe *= p;
        }
    }

    /* Group lookups for large primes */
    /* Group by K = N // P */
    typedef struct { int K; int P_list[200]; int nP; } group_t;
    group_t groups[200];
    int ngroups = 0;

    /* Collect groups */
    int group_map[N + 1];
    memset(group_map, -1, sizeof(group_map));

    for (int i = 0; i < nlarge; i++) {
        int P = large_primes[i];
        int K = N / P;
        if (group_map[K] == -1) {
            group_map[K] = ngroups;
            groups[ngroups].K = K;
            groups[ngroups].nP = 0;
            ngroups++;
        }
        int gi = group_map[K];
        groups[gi].P_list[groups[gi].nP++] = P;
    }

    ngrp_lookups = ngroups;
    for (int gi = 0; gi < ngroups; gi++) {
        int K = groups[gi].K;
        int limit_idx = bisect_right_smooths(K);
        grp_lookups[gi].limit = limit_idx;

        /* Compute weights for each P in group */
        for (int cnt = 0; cnt <= limit_idx; cnt++) {
            long long val = 1;
            long long p2 = mod_pow(2, cnt, MOD);
            for (int pi = 0; pi < groups[gi].nP; pi++) {
                int P = groups[gi].P_list[pi];
                /* w(P, 1) = -(P-1) / W_P mod MOD */
                /* W_P = P for large primes */
                long long phi = (P - 1) % MOD;
                long long w = (MOD - phi) % MOD * modinv(W_vals[P]) % MOD;
                long long term = (p2 + w) % MOD;
                val = val * term % MOD;
            }
            grp_lookups[gi].table[cnt] = val;
        }
    }

    /* Powers of 2 */
    for (int i = 0; i <= num_smooths; i++)
        pow2_all[i] = mod_pow(2, i, MOD);

    /* Recurse */
    total_sum = 0;
    bitmask initial_mask = bm_full(num_smooths);
    recurse(0, initial_mask, 1);

    long long ans = total_sum * C_val % MOD;
    printf("%lld\n", ans);
    return 0;
}
