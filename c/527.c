/*
 * Project Euler 527 - Randomized Binary Search
 *
 * R(n) = 2*(n+1)/n * H(n) - 3, where H(n) is the n-th harmonic number.
 * B(n) is computed via recursion (O(log n) depth since binary search halves).
 * H(n) for large n uses the Euler-Maclaurin asymptotic expansion.
 * Answer: R(n) - B(n) for n = 10^10.
 */
#include <stdio.h>
#include <math.h>
#include <stdint.h>

/*
 * Bernoulli numbers B_{2k} for k=1..20 (precomputed exactly as doubles)
 * B_2  = 1/6, B_4 = -1/30, B_6 = 1/42, B_8 = -1/30, B_10 = 5/66,
 * B_12 = -691/2730, B_14 = 7/6, B_16 = -3617/510, B_18 = 43867/798,
 * B_20 = -174611/330
 */
static const double bernoulli2k[] = {
    1.0/6, -1.0/30, 1.0/42, -1.0/30, 5.0/66,
    -691.0/2730, 7.0/6, -3617.0/510, 43867.0/798, -174611.0/330,
    854513.0/138, -236364091.0/2730, 8553103.0/6, -23749461029.0/870,
    8615841276005.0/14322, -7709321041217.0/510, 2577687858367.0/6,
    -26315271553053477373.0/1919190, 2929993913841559.0/6,
    -261082718496449122051.0/13530
};

static long double harmonic_large(int64_t n) {
    long double nd = (long double)n;
    long double gamma = 0.5772156649015328606065120900824024310421L;
    long double Hn = logl(nd) + gamma + 1.0L / (2.0L * nd);

    long double n_pow = nd * nd; /* n^(2k) */
    for (int k = 1; k <= 15; k++) {
        Hn -= (long double)bernoulli2k[k-1] / (2 * k * n_pow);
        n_pow *= nd * nd;
    }
    return Hn;
}

/* B(n): expected guesses for standard binary search on range [1..n] */
/* Recursive with memoization via simple cache for powers of 2 related values */
/* Since the recursion halves n each time, depth is O(log n) ~ 34 */
typedef struct { int64_t key; long double val; } CacheEntry;
#define CACHE_SIZE 65536
#define CACHE_MASK (CACHE_SIZE - 1)
static CacheEntry cache[CACHE_SIZE];
static int cache_used = 0;

static void cache_init(void) {
    for (int i = 0; i < CACHE_SIZE; i++) cache[i].key = -1;
}

static int cache_get(int64_t key, long double *val) {
    int h = (int)(key % CACHE_SIZE);
    if (h < 0) h += CACHE_SIZE;
    /* Linear probe */
    for (int i = 0; i < 32; i++) {
        int idx = (h + i) & CACHE_MASK;
        if (cache[idx].key == key) { *val = cache[idx].val; return 1; }
        if (cache[idx].key == -1) return 0;
    }
    return 0;
}

static void cache_put(int64_t key, long double val) {
    int h = (int)(key % CACHE_SIZE);
    if (h < 0) h += CACHE_SIZE;
    for (int i = 0; i < 32; i++) {
        int idx = (h + i) & CACHE_MASK;
        if (cache[idx].key == -1 || cache[idx].key == key) {
            cache[idx].key = key;
            cache[idx].val = val;
            return;
        }
    }
    /* Fallback: overwrite first slot */
    cache[h].key = key;
    cache[h].val = val;
}

static long double B(int64_t n) {
    if (n <= 1) return 1.0L;
    long double v;
    if (cache_get(n, &v)) return v;

    int64_t mid = (n + 1) / 2;
    int64_t left = mid - 1;
    int64_t right = n - mid;
    long double res = 1.0L + ((long double)left * B(left) + (long double)right * B(right)) / (long double)n;
    cache_put(n, res);
    return res;
}

int main(void) {
    cache_init();

    int64_t N = 10000000000LL; /* 10^10 */
    long double Hn = harmonic_large(N);
    long double R_val = 2.0L * (long double)(N + 1) / (long double)N * Hn - 3.0L;
    long double B_val = B(N);
    long double ans = R_val - B_val;

    printf("%.8Lf\n", ans);
    return 0;
}
