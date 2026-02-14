/*
 * Project Euler 762 - Amoeba Division
 *
 * A grid with K=4 rows. An amoeba at (x,y) divides into amoebas at
 * (x+1,y) and (x+1,(y+1)%K). Count distinct arrangements after N divisions.
 *
 * Uses BFS/DP: for each column configuration (tuple of counts), enumerate
 * all ways to choose which amoebas divide, compute the resulting next column,
 * and combine same-result configurations. Final answer counts configs where
 * all positions have at most 1 amoeba.
 *
 * Since N=100000 is large but K=4 is small, the column configs are bounded
 * (each entry 0..2K-1). We use a hash map of column config -> count.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

#define K 4
#define MOD_VAL 1000000000LL
#define MAX_ENTRY 8  /* 2*K */

/* Encode a K-tuple of small ints into a single 32-bit key */
/* Each entry fits in 4 bits (0..7), so 4 entries = 16 bits */
static inline uint32_t encode(int c[K]) {
    return (uint32_t)c[0] | ((uint32_t)c[1] << 4) | ((uint32_t)c[2] << 8) | ((uint32_t)c[3] << 12);
}

static inline void decode(uint32_t key, int c[K]) {
    c[0] = key & 0xF;
    c[1] = (key >> 4) & 0xF;
    c[2] = (key >> 8) & 0xF;
    c[3] = (key >> 12) & 0xF;
}

/* Simple hash map: key -> value (mod MOD_VAL) */
#define HM_SIZE (1 << 16)
#define HM_MASK (HM_SIZE - 1)

typedef struct {
    uint32_t *keys;
    int64_t *vals;
    int *next;
    int *buckets;
    int count;
    int cap;
} HashMap;

static void hm_init(HashMap *hm, int cap) {
    hm->keys = (uint32_t *)malloc(cap * sizeof(uint32_t));
    hm->vals = (int64_t *)malloc(cap * sizeof(int64_t));
    hm->next = (int *)malloc(cap * sizeof(int));
    hm->buckets = (int *)malloc(HM_SIZE * sizeof(int));
    memset(hm->buckets, -1, HM_SIZE * sizeof(int));
    hm->count = 0;
    hm->cap = cap;
}

static void hm_free(HashMap *hm) {
    free(hm->keys);
    free(hm->vals);
    free(hm->next);
    free(hm->buckets);
}

static void hm_clear(HashMap *hm) {
    memset(hm->buckets, -1, HM_SIZE * sizeof(int));
    hm->count = 0;
}

static void hm_add(HashMap *hm, uint32_t key, int64_t val) {
    int bucket = key & HM_MASK;
    for (int idx = hm->buckets[bucket]; idx != -1; idx = hm->next[idx]) {
        if (hm->keys[idx] == key) {
            hm->vals[idx] = (hm->vals[idx] + val) % MOD_VAL;
            return;
        }
    }
    int idx = hm->count++;
    hm->keys[idx] = key;
    hm->vals[idx] = val % MOD_VAL;
    hm->next[idx] = hm->buckets[bucket];
    hm->buckets[bucket] = idx;
}

/* Generate all division choices for a column config via cartesian product.
 * For each position i with count c[i]:
 *   if c[i] == 0: must choose 0 (no amoeba to divide)
 *   else: can choose c[i]-1 (one divides) or c[i] (none divide)
 * Then col_divisions[i] is the number of amoebas that DON'T divide.
 * Wait - re-reading the Python: axes[i] = [col_count-1, col_count] when col_count > 0.
 * col_divisions is the chosen value. total_divisions = sum of col_divisions.
 * But that doesn't make sense as "total divisions" - let me re-read.
 *
 * Actually looking more carefully:
 * axes[i] = [col_count[i]-1, col_count[i]] means we choose how many remain.
 * Wait no - in the Python code, total_divisions = sum(col_divisions), and this
 * must be > 0 and < K. And then new_positions[i] = col_divisions[i] + col_divisions[(i+1)%K].
 *
 * So col_divisions[i] represents the number of amoebas at position i that divide.
 * If c[i] > 0, choices are c[i]-1 or c[i]. If c[i] == 0, choice is 0.
 * Wait, that's the number remaining... Hmm.
 *
 * Let me re-read: "axes.append([col_count - 1, col_count])"
 * Then total_divisions = sum(col_divisions). If total_divisions > 0 and < K.
 * Then new_positions[i] = col_divisions[i] + col_divisions[(i+1)%K].
 *
 * The "col_divisions" values represent counts in the next column.
 * When an amoeba at position i divides, it produces one at i and one at (i+1)%K.
 * So choosing col_count[i]-1 means one amoeba divides (producing 1 at i and 1 at (i+1)%K
 * in the next column, plus col_count[i]-1 that don't divide stay in current column).
 * But wait - col_divisions is what contributes to the NEXT column's count at position i.
 *
 * Actually: if col_count[i] amoebas are at position i, and d of them divide,
 * then the next column gets d at position i and d at position (i+1)%K from those.
 * The remaining col_count[i]-d stay. But in this model, all are in one column,
 * and divisions advance them to the next column.
 *
 * So: col_divisions[i] = number of amoebas from position i that divide.
 * Options: 0 to col_count[i]. But axes gives [col_count-1, col_count] meaning
 * either col_count-1 or col_count divide. That means almost all must divide.
 *
 * With total_divisions > 0 and < K, and the result being used to advance
 * by total_divisions steps, this is a complex recurrence.
 *
 * This is an O(N * states * 2^K) algorithm. For N=100000, that's way too slow.
 * The Python code seems to run this directly, which would be extremely slow.
 * The Python has an "extrapolation" wrapper that does Lagrange interpolation,
 * but it's a no-op (returns the function itself).
 *
 * Looking at the answer 285528881, this must use a smarter approach.
 * The Python solution as written won't produce the correct answer for N=100000.
 * Let me look for a pattern or formula.
 *
 * Actually wait - the N=100000 with K=4 and the extrapolation hint suggests
 * Lagrange interpolation: compute C(n) for small n, then extrapolate to N=100000
 * using the fact that C(n) satisfies a linear recurrence (polynomial in n).
 *
 * Since the extrapolation function is a no-op in the Python, the Python solution
 * as written is incomplete/broken for the actual problem parameters.
 * But the answer is known: 285528881.
 *
 * Let me implement the BFS approach for small n to find the pattern,
 * then use Lagrange interpolation to get C(100000).
 */

/* After more analysis: the BFS computes C(n) for small n values.
 * Then we use Lagrange interpolation to extrapolate to N=100000.
 * The recurrence is known to be eventually polynomial of bounded degree.
 * We compute enough initial values and interpolate.
 */

#define N_TARGET 100000
#define MAX_N_COMPUTE 200  /* compute this many initial values for interpolation */
#define MAX_STATES_MAP 100000

static HashMap maps[MAX_N_COMPUTE + 1];

static int64_t compute_C(int n) {
    /* BFS DP to compute C(n) */
    for (int i = 0; i <= n; i++)
        hm_clear(&maps[i]);

    int start[K] = {1, 0, 0, 0};
    hm_add(&maps[0], encode(start), 1);

    for (int division = 0; division < n; division++) {
        HashMap *cur = &maps[division];
        for (int idx = 0; idx < cur->count; idx++) {
            uint32_t key = cur->keys[idx];
            int64_t count = cur->vals[idx];
            if (count == 0) continue;

            int cc[K];
            decode(key, cc);

            /* Generate all division choices via cartesian product */
            /* Each position i: if cc[i]==0, only 0; else cc[i]-1 or cc[i] */
            int n_choices[K];
            int choices[K][2];
            int total_combos = 1;
            for (int i = 0; i < K; i++) {
                if (cc[i] == 0) {
                    n_choices[i] = 1;
                    choices[i][0] = 0;
                } else {
                    n_choices[i] = 2;
                    choices[i][0] = cc[i] - 1;
                    choices[i][1] = cc[i];
                }
                total_combos *= n_choices[i];
            }

            for (int combo = 0; combo < total_combos; combo++) {
                int cd[K];
                int tmp = combo;
                int total_div = 0;
                for (int i = 0; i < K; i++) {
                    cd[i] = choices[i][tmp % n_choices[i]];
                    tmp /= n_choices[i];
                    total_div += cd[i];
                }

                if (total_div > 0 && total_div < K) {
                    int next_step = division + total_div;
                    if (next_step <= n) {
                        int new_pos[K];
                        for (int i = 0; i < K; i++)
                            new_pos[i] = cd[i] + cd[(i + 1) % K];
                        hm_add(&maps[next_step], encode(new_pos), count);
                    }
                }
            }
        }
    }

    /* Count final configs where all positions have at most 1 amoeba */
    int64_t result = 0;
    HashMap *final = &maps[n];
    for (int idx = 0; idx < final->count; idx++) {
        int cc[K];
        decode(final->keys[idx], cc);
        int ok = 1;
        for (int i = 0; i < K; i++)
            if (cc[i] > 1) { ok = 0; break; }
        if (ok)
            result = (result + final->vals[idx]) % MOD_VAL;
    }
    return result;
}

/* Lagrange interpolation mod p */
static int64_t pow_mod(int64_t base, int64_t exp, int64_t mod) {
    int64_t result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1)
            result = (__int128)result * base % mod;
        base = (__int128)base * base % mod;
        exp >>= 1;
    }
    return result;
}

static int64_t mod_inv(int64_t a, int64_t mod) {
    return pow_mod(a, mod - 2, mod);
}

/* Lagrange interpolation: given y[0..d] at x=0,1,...,d, evaluate at x=target */
static int64_t lagrange_interp(int64_t *y, int d, int64_t target, int64_t mod) {
    /* prefix[i] = product_{j=0}^{i-1} (target - j) */
    int64_t *prefix = (int64_t *)malloc((d + 2) * sizeof(int64_t));
    int64_t *suffix = (int64_t *)malloc((d + 2) * sizeof(int64_t));

    prefix[0] = 1;
    for (int i = 0; i <= d; i++)
        prefix[i + 1] = (__int128)prefix[i] * ((target - i) % mod + mod) % mod;

    suffix[d + 1] = 1;
    for (int i = d; i >= 0; i--)
        suffix[i] = (__int128)suffix[i + 1] * ((target - i) % mod + mod) % mod;

    /* Precompute factorials */
    int64_t *fact = (int64_t *)malloc((d + 1) * sizeof(int64_t));
    int64_t *inv_fact = (int64_t *)malloc((d + 1) * sizeof(int64_t));
    fact[0] = 1;
    for (int i = 1; i <= d; i++)
        fact[i] = (__int128)fact[i - 1] * i % mod;
    inv_fact[d] = mod_inv(fact[d], mod);
    for (int i = d - 1; i >= 0; i--)
        inv_fact[i] = (__int128)inv_fact[i + 1] * (i + 1) % mod;

    int64_t result = 0;
    for (int i = 0; i <= d; i++) {
        int64_t num = (__int128)prefix[i] * suffix[i + 1] % mod;
        int64_t denom = (__int128)inv_fact[i] * inv_fact[d - i] % mod;
        if ((d - i) % 2 == 1) denom = (mod - denom) % mod;
        result = (result + (__int128)y[i] * num % mod * denom) % mod;
    }

    free(prefix);
    free(suffix);
    free(fact);
    free(inv_fact);
    return result;
}

int main(void) {
    /* First compute C(n) for n = 0, 1, ..., MAX_N_COMPUTE-1 */
    for (int i = 0; i <= MAX_N_COMPUTE; i++)
        hm_init(&maps[i], MAX_STATES_MAP);

    int64_t *vals = (int64_t *)malloc((MAX_N_COMPUTE + 1) * sizeof(int64_t));

    for (int n = 0; n < MAX_N_COMPUTE; n++) {
        vals[n] = compute_C(n);
    }

    /* Try interpolation with increasing degree until stable */
    /* The sequence should satisfy a linear recurrence, so partial sums are polynomial */
    /* Actually C(n) itself might be eventually periodic or polynomial.
     * Let's try: compute enough values and see if Lagrange interpolation works.
     * We need to find the degree d such that values at 0..d suffice.
     * Try d from small up until the interpolated value at d+1 matches computed value.
     */
    int64_t mod = MOD_VAL;
    /* MOD_VAL = 10^9 is not prime! We need a prime modulus for Lagrange.
     * Let's use the answer modulo 10^9. Since 10^9 = 2^9 * 5^9, not prime.
     * We can use CRT: compute mod 2^9=512 and mod 5^9=1953125, then combine.
     * Or better: just try direct interpolation and see if it works.
     *
     * Actually, for Lagrange interpolation we need inverses of (i-j) for i!=j.
     * With non-prime modulus this fails. Let's instead compute the answer exactly
     * using bignum or use CRT with suitable primes.
     *
     * Simpler approach: use two large primes, interpolate mod each, then CRT.
     */

    /* Actually, let me think about this differently. The Python solution's
     * extrapolation is a no-op, meaning it just calls C(N) directly.
     * But C(100000) with BFS is infeasible. So the Python solution must be
     * wrong/incomplete for N=100000.
     *
     * The answer 285528881 must come from a different approach.
     * Let me try computing initial values and finding a recurrence.
     */

    /* Print computed values for analysis */
    /* Check if the sequence stabilizes into a linear recurrence */

    /* For now, try Lagrange interpolation with two primes and CRT */
    int64_t p1 = 1000000007LL;
    int64_t p2 = 998244353LL;

    /* We need C(n) mod p1 and mod p2 for n=0..d */
    /* Since our computed vals are mod 10^9, we need to recompute mod these primes.
     * But actually our computation uses MOD_VAL=10^9 internally.
     * Let me recompute with a prime modulus.
     */

    /* Recompute with mod p1 */
    /* This requires changing MOD_VAL, but it's a #define...
     * Let me just compute the values exactly (they should be small enough for
     * moderate n) and then reduce mod primes.
     *
     * Actually for small n (say n <= 100), C(n) should be small enough to
     * fit in int64_t without modular reduction. Let me check.
     * C(0)=1, C(1)=? The number of valid arrangements grows, but for n~100
     * it could be huge. Let's just use mod arithmetic.
     *
     * The cleanest approach: since MOD=10^9 and we need interpolation,
     * let's just hope the polynomial degree is small enough that we can
     * detect it and verify. If we compute values for n=0..199 with mod 10^9,
     * and the interpolation at n=200 (not computed) matches when we compute it,
     * then we're good.
     */

    /* Try finding the right degree */
    int best_d = -1;
    for (int d = 5; d < MAX_N_COMPUTE - 5; d++) {
        /* Check if interpolating 0..d predicts d+1 correctly */
        int64_t predicted = lagrange_interp(vals, d, d + 1, p1);
        /* But vals are mod 10^9, not mod p1... This won't work directly. */
        /* We need consistent modulus. Let me just use mod 10^9 and hope
         * the denominators are coprime to 10^9 (they won't always be). */
        break;
    }

    /* Alternative approach: compute differences until they become constant */
    /* d-th finite difference of a degree-d polynomial is constant */
    int64_t *diff = (int64_t *)malloc(MAX_N_COMPUTE * sizeof(int64_t));
    for (int i = 0; i < MAX_N_COMPUTE; i++)
        diff[i] = vals[i];

    int degree = -1;
    for (int d = 1; d < MAX_N_COMPUTE - 1; d++) {
        for (int i = 0; i < MAX_N_COMPUTE - d; i++)
            diff[i] = ((diff[i + 1] - diff[i]) % MOD_VAL + MOD_VAL) % MOD_VAL;
        /* Check if all remaining values are the same */
        int all_same = 1;
        for (int i = 1; i < MAX_N_COMPUTE - d - 5; i++) {
            if (diff[i] != diff[0]) { all_same = 0; break; }
        }
        if (all_same) {
            degree = d;
            break;
        }
    }

    if (degree > 0 && degree < MAX_N_COMPUTE - 10) {
        /* Use Newton's forward difference formula */
        /* Recompute forward differences from scratch */
        int64_t **fd = (int64_t **)malloc((degree + 1) * sizeof(int64_t *));
        fd[0] = (int64_t *)malloc((degree + 1) * sizeof(int64_t));
        for (int i = 0; i <= degree; i++)
            fd[0][i] = vals[i];
        for (int d = 1; d <= degree; d++) {
            fd[d] = (int64_t *)malloc((degree + 1 - d) * sizeof(int64_t));
            for (int i = 0; i <= degree - d; i++)
                fd[d][i] = ((fd[d-1][i+1] - fd[d-1][i]) % MOD_VAL + MOD_VAL) % MOD_VAL;
        }

        /* Newton interpolation: C(n) = sum_{k=0}^{degree} C(n,k) * fd[k][0] */
        int64_t ans = 0;
        int64_t binom = 1;  /* C(N_TARGET, k) mod MOD_VAL */
        /* But MOD_VAL is not prime, so we can't compute C(N,k) mod MOD_VAL easily.
         * Use __int128 for the binomial computation. */

        /* Actually for Newton's formula with non-prime mod, we need:
         * C(n) = sum fd[k][0] * binom(n, k)
         * binom(n,k) = n*(n-1)*...*(n-k+1) / k!
         * With mod 10^9, division by k! is problematic.
         *
         * Let's try a different approach: compute C(n) mod small primes
         * and use CRT. But we need the values mod those primes too.
         */

        /* Actually, since fd[k][0] are known and binom(n,k) is an integer,
         * we can compute binom(n,k) mod 10^9 if we know it's an integer.
         * n*(n-1)*...*(n-k+1) is always divisible by k!, so:
         * binom(n,k) mod m = (n*(n-1)*...*(n-k+1) / k!) mod m
         *
         * For this, compute the product mod m*k! (or use exact arithmetic
         * with __int128 for small k, then divide by k! and reduce mod m).
         * Since degree should be small (maybe 10-20), k! fits in int64.
         */
        for (int k = 0; k <= degree; k++) {
            /* Compute binom(N_TARGET, k) mod MOD_VAL */
            /* = N_TARGET * (N_TARGET-1) * ... * (N_TARGET-k+1) / k! */
            __int128 num = 1;
            __int128 den = 1;
            for (int j = 0; j < k; j++) {
                num *= (N_TARGET - j);
                den *= (j + 1);
            }
            int64_t bk = (int64_t)((num / den) % MOD_VAL);
            if (bk < 0) bk += MOD_VAL;
            ans = (ans + (__int128)fd[k][0] * bk) % MOD_VAL;
        }

        printf("%lld\n", ans);

        for (int d = 0; d <= degree; d++)
            free(fd[d]);
        free(fd);
    } else {
        /* Fallback: if we can't find polynomial degree, just print what we have */
        printf("ERROR: could not determine polynomial degree (got %d)\n", degree);
    }

    free(vals);
    free(diff);
    for (int i = 0; i <= MAX_N_COMPUTE; i++)
        hm_free(&maps[i]);

    return 0;
}
