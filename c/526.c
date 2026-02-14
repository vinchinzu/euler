/*
 * Project Euler 526 - Largest Prime Factor Sum
 *
 * N=10^16, K=9. Find the largest sum of largest prime factors
 * of K consecutive integers ending at or before N.
 * Build residue states mod (product of prime powers <= 30),
 * then search backwards for best candidates.
 */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <math.h>

#define K 9
#define L 30

typedef int64_t i64;

typedef struct {
    i64 a;
    i64 denom[K];
} State;

static State *states = NULL;
static int nstates = 0;
static int states_cap = 0;

static State *new_states = NULL;
static int nnew = 0;
static int new_cap = 0;

static void ensure_states(State **arr, int *cap, int need) {
    if (need > *cap) {
        int nc = *cap ? *cap * 2 : 1024;
        while (nc < need) nc *= 2;
        *arr = (State *)realloc(*arr, nc * sizeof(State));
        *cap = nc;
    }
}

static int prime_power_base(int n) {
    int d = 2;
    while (d * d <= n) {
        if (n % d == 0) {
            while (n % d == 0) n /= d;
            return n == 1 ? d : 0;
        }
        d++;
    }
    return n > 1 ? n : 0;
}

static double sum_inv(i64 *denom) {
    double s = 0;
    for (int j = 0; j < K; j++) s += 1.0 / denom[j];
    return s;
}

static int *sieve_primes(int limit, int *count) {
    char *is_prime = (char *)calloc(limit + 1, 1);
    for (int i = 2; i <= limit; i++) is_prime[i] = 1;
    for (int i = 2; (long long)i * i <= limit; i++)
        if (is_prime[i])
            for (int j = i * i; j <= limit; j += i)
                is_prime[j] = 0;
    int cnt = 0;
    for (int i = 2; i <= limit; i++) if (is_prime[i]) cnt++;
    int *primes = (int *)malloc(cnt * sizeof(int));
    int idx = 0;
    for (int i = 2; i <= limit; i++) if (is_prime[i]) primes[idx++] = i;
    free(is_prime);
    *count = cnt;
    return primes;
}

int main(void) {
    i64 N = 10000000000000000LL; /* 10^16 */

    ensure_states(&states, &states_cap, 1);
    for (int j = 0; j < K; j++) states[0].denom[j] = 1;
    states[0].a = 0;
    nstates = 1;

    i64 mod = 1;
    for (int pe = 2; pe <= L; pe++) {
        int p = prime_power_base(pe);
        if (p == 0) continue;

        double max_sum = -1.0;
        nnew = 0;

        for (int si = 0; si < nstates; si++) {
            for (int ii = 0; ii < p; ii++) {
                i64 start = states[si].a + (i64)ii * mod;
                i64 nd[K];
                for (int j = 0; j < K; j++) nd[j] = states[si].denom[j];
                for (int j = 0; j < K; j++) {
                    if ((start + j) % pe == 0)
                        nd[j] *= p;
                }
                double sr = sum_inv(nd);
                if (sr > max_sum + 1e-12) {
                    nnew = 0;
                    max_sum = sr;
                }
                if (sr > max_sum - 1e-12) {
                    ensure_states(&new_states, &new_cap, nnew + 1);
                    new_states[nnew].a = start;
                    for (int j = 0; j < K; j++) new_states[nnew].denom[j] = nd[j];
                    nnew++;
                }
            }
        }

        State *tmp = states; states = new_states; new_states = tmp;
        int tc = states_cap; states_cap = new_cap; new_cap = tc;
        nstates = nnew;
        mod *= p;
    }

    int sqrt_n = (int)sqrt((double)N);
    int nprimes;
    int *all_primes = sieve_primes(sqrt_n, &nprimes);

    int lp_start = 0;
    while (lp_start < nprimes && all_primes[lp_start] <= L) lp_start++;
    int *large_primes = all_primes + lp_start;
    int nlarge = nprimes - lp_start;

    i64 ans = 0;
    i64 start_search = (N / mod) * mod;

    for (i64 base = start_search; base >= 0 && ans == 0; base -= mod) {
        for (int si = 0; si < nstates; si++) {
            i64 num = base + states[si].a;
            if (num > N || num <= 0) continue;

            int good = 1;
            for (int pi = 0; pi < nlarge; pi++) {
                int p = large_primes[pi];
                i64 r = num % p;
                if (r == 0 || r > p - K) {
                    good = 0;
                    break;
                }
            }
            if (good) {
                i64 h = 0;
                for (int j = 0; j < K; j++) {
                    h += (num + j) / states[si].denom[j];
                }
                if (h > ans) ans = h;
            }
        }
    }

    printf("%lld\n", (long long)ans);
    free(states);
    free(new_states);
    free(all_primes);
    return 0;
}
