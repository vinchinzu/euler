/*
 * Project Euler 775 - Paper Wrapping
 *
 * Compute sum_{n=1}^N g(n) where g(n) = paper for n separate cubes minus
 * paper for n cubes bundled as a poly-cube.
 *
 * The algorithm simulates face additions on the growing rectangular prism,
 * tracking when new paper is needed.
 */
#include <stdio.h>
#include <stdint.h>
#include <math.h>

typedef __int128 i128;

#define MOD 1000000007LL

static int64_t pow_mod(int64_t base, int64_t exp, int64_t mod) {
    int64_t result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1)
            result = (i128)result * base % mod;
        base = (i128)base * base % mod;
        exp >>= 1;
    }
    return result;
}

static int64_t tr(int64_t n, int64_t mod) {
    /* n*(n+1)/2 mod mod */
    int64_t a = n % mod;
    int64_t b = (n + 1) % mod;
    return (i128)a * b % mod * pow_mod(2, mod - 2, mod) % mod;
}

static int64_t nCr(int64_t n, int k, int64_t mod) {
    if (k < 0 || k > n) return 0;
    if (k == 0) return 1;
    int64_t result = 1;
    for (int i = 0; i < k; i++) {
        result = (i128)result * ((n - i) % mod + mod) % mod;
        result = (i128)result * pow_mod(i + 1, mod - 2, mod) % mod;
    }
    return result;
}

/* Sum of squares: 1^2 + 2^2 + ... + d^2 mod mod */
static int64_t sum_sq(int64_t d, int64_t mod) {
    if (d <= 0) return 0;
    int64_t a = d % mod;
    int64_t b = (d + 1) % mod;
    int64_t c = (2 * d + 1) % mod;
    int64_t inv6 = pow_mod(6, mod - 2, mod);
    return (i128)a * b % mod * c % mod * inv6 % mod;
}

static int64_t isqrt64(int64_t n) {
    if (n <= 0) return 0;
    int64_t s = (int64_t)sqrt((double)n);
    while (s * s > n) s--;
    while ((s + 1) * (s + 1) <= n) s++;
    return s;
}

int main(void) {
    int64_t N = 10000000000000000LL; /* 10^16 */
    int64_t M = MOD;

    int64_t sides[3] = {1, 1, 1};
    int64_t index = 1;
    int64_t N_mod = N % M;

    /* ans = 6 * (tr(N) - N) mod M */
    int64_t trN = tr(N, M);
    int64_t ans = (6 * ((trN - N_mod % M + M) % M)) % M;

    while (1) {
        int64_t side1 = sides[1];
        int64_t side2 = sides[2];
        int64_t d1_lim = isqrt64(N - index - 1);
        int64_t d1 = d1_lim < side2 - 1 ? d1_lim : side2 - 1;
        int64_t d2_lim = (isqrt64(4 * (N - index)) - 1) / 2;
        int64_t d2 = d2_lim < side1 - 1 ? d2_lim : side1 - 1;

        int64_t ni_mod = (N - index) % M;
        if (ni_mod < 0) ni_mod += M;

        /* ans -= 4 * (N - index) */
        ans = (ans - 4 * ni_mod % M + M) % M;

        /* ans -= 2 * ((N - index) * d1 - sum_sq(d1)) */
        {
            int64_t d1_mod = d1 % M;
            int64_t term = ((i128)ni_mod * d1_mod % M - sum_sq(d1, M) + M) % M;
            ans = (ans - 2 * term % M + M) % M;
        }

        /* ans -= 2 * ((N - index) * d2 - 2 * C(d2+2, 3)) */
        {
            int64_t d2_mod = d2 % M;
            int64_t c3 = nCr(d2 + 2, 3, M);
            int64_t term = ((i128)ni_mod * d2_mod % M - 2 * c3 % M + M) % M;
            ans = (ans - 2 * term % M + M) % M;
        }

        index += side1 * side2;
        if (index >= N)
            break;

        /* Rotate sides: pop front, increment, push back */
        int64_t front = sides[0] + 1;
        sides[0] = sides[1];
        sides[1] = sides[2];
        sides[2] = front;
    }

    ans = ans % M;
    printf("%lld\n", ans);
    return 0;
}
