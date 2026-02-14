/*
 * Project Euler Problem 390: Triangles with Non Rational Sides and Integral Area
 *
 * Extracted from embedded C in Python solution.
 */
#include <stdio.h>
#include <math.h>
#include <stdint.h>

typedef unsigned __int128 uint128;

static inline uint64_t isqrt128(uint128 n) {
    if (n == 0) return 0;
    uint64_t x = (uint64_t)sqrt((double)n);
    for (int i = 0; i < 5; i++) {
        if (x == 0) break;
        uint128 x128 = (uint128)x;
        uint128 next = (x128 + n / x128) / 2;
        if (next >= x128 && next - x128 <= 1) break;
        if (x128 >= next && x128 - next <= 1) break;
        x = (uint64_t)next;
    }
    while ((uint128)x * x > n) x--;
    while ((uint128)(x + 1) * (x + 1) <= n) x++;
    return x;
}

int main(void) {
    const uint64_t N = 20000000000ULL;  /* 2 * 10^10 */
    uint128 ans = 0;

    for (uint64_t a = 2; (uint128)a * a + 1 <= N; a += 2) {
        uint64_t a2 = a * a;
        uint64_t upper_bound = N / (a2 + 1);
        for (uint64_t t = 2; t <= upper_bound; t += 2) {
            uint128 s = (uint128)a2 * t * t - a2 + (uint128)t * t;
            uint64_t v = isqrt128(s);
            if ((uint128)v * v != s) continue;
            uint128 b = (uint128)a * t + v;
            uint128 n_val = (uint128)a * b + t;
            if (n_val > N) break;
            ans += n_val / 2;
        }
    }

    uint64_t hi = (uint64_t)(ans / 1000000000000ULL);
    uint64_t lo = (uint64_t)(ans % 1000000000000ULL);
    if (hi > 0) printf("%lu%012lu\n", (unsigned long)hi, (unsigned long)lo);
    else printf("%lu\n", (unsigned long)lo);
    return 0;
}
