/*
 * Project Euler Problem 582: Integer sided triangles with 120 degree angle.
 *
 * Find the number of integer sided triangles with a 120 degree angle and sides
 * a <= b <= c satisfying b - a <= K=100, and c <= N=10^100.
 *
 * By the Law of Cosines: a^2 + a*b + b^2 = c^2.
 * Letting d = b - a, this gives (2c)^2 - 3(2a+d)^2 = d^2.
 * This is a generalized Pell equation x^2 - 3*y^2 = d^2.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

typedef __int128 i128;
typedef long long ll;

/*
 * We need to work with very large numbers (up to 10^100).
 * We'll use a simple big-number representation for checking bounds,
 * but actually the Python solution checks c <= 10^100 which is enormous.
 * The Pell equation solutions grow exponentially, so there are only
 * a bounded number of solutions per d.
 *
 * The fundamental solution to x^2 - 3*y^2 = 1 is (2, 1).
 * For each d, find particular solution to x^2 - 3*y^2 = d^2,
 * then generate more using (2,1).
 *
 * Actually, for d <= 100, x=2d, y=d works: (2d)^2 - 3*d^2 = d^2. Check.
 * Then generate more: x_{n+1} = 2*x_n + 3*y_n, y_{n+1} = x_n + 2*y_n.
 *
 * Filter: x must be even (for c = x/2 to be integer) and y > d (for a > 0).
 * Since N = 10^100, we keep generating until c > 10^100.
 * With __int128 we can handle up to about 10^38.
 * But 10^100 is much bigger. We need arbitrary precision.
 *
 * However, solutions grow as (2+sqrt(3))^k ~ 3.73^k.
 * So to reach 10^100, we need k ~ 100/log10(3.73) ~ 175 solutions.
 * We can use GMP or simply count without bound checking (since all
 * solutions with positive a contribute).
 *
 * Actually wait - the answer is 19903, which is small. The constraint
 * c <= 10^100 is essentially "no bound" for practical purposes.
 * We just need to count all solutions with a >= 1 (i.e., y > d)
 * that we can generate. Since solutions grow exponentially, there
 * are finitely many with x, y fitting in some large representation.
 *
 * Let me count more carefully. For each d, the particular solution
 * is (x, y) = (2d, d). Then generate using the recurrence.
 * But (2d, d) gives y = d, so 2a + d = d means a = 0 (invalid).
 * We need the NEXT solution.
 *
 * Actually let me also look for (x, y) with x = d, y = 0:
 * d^2 - 3*0 = d^2. Yes! But y=0 means 2a+d=0, so a<0. Skip.
 *
 * Particular solutions: we need x^2 - 3*y^2 = d^2.
 * (x, y) = (d, 0) and (2d, d) are both solutions.
 * From (d, 0): next = (2*d + 0, d + 0) = (2d, d). Already known.
 * From (2d, d): next = (2*2d + 3*d, 2d + 2*d) = (7d, 4d).
 *   Check: (7d)^2 - 3*(4d)^2 = 49d^2 - 48d^2 = d^2. Yes!
 *   c = 7d/2 -> need d even for c integer. y = 4d > d. a = (4d-d)/2 = 3d/2 -> need d even.
 *   Wait, x = 2c so c = x/2. For c integer, x must be even.
 *   x = 7d: even iff d even.
 *
 * But we also need to consider negative y solutions.
 * x^2 - 3*y^2 = d^2 with y negative: same as positive.
 *
 * Let me reconsider: the fundamental solution (2, 1) generates all solutions
 * from each class. For x^2 - 3*y^2 = d^2, we have classes based on y mod...
 *
 * Actually let me just follow the Python solution exactly:
 * For each d from 1 to 100:
 *   Find particular solution (x0, y0) to x^2 - 3*y^2 = d^2
 *   Generate solutions using fundamental (u, v) = (2, 1):
 *     x_{n+1} = u*x_n + d_val*v*y_n = 2*x_n + 3*y_n
 *     y_{n+1} = u*y_n + v*x_n = 2*y_n + x_n
 *   For each solution, check x even and y > d.
 *
 * Since 10^100 is unreachable with finite precision, I'll use a max iteration count.
 * With __int128 we can go up to ~10^38, but solutions double ~every step.
 * After ~240 steps we'd reach 10^100. But __int128 only handles ~39 digits.
 *
 * Let me use a simple 256-byte big integer for x and y.
 * Or better: since we're just counting, and solutions grow as 3.73^k,
 * after 175 iterations x > 10^100, and we need at most ~175 solutions per d.
 * I'll use GMP-style arrays.
 *
 * Actually, simplest approach: use Python-like big integers with arrays.
 * Or: since the growth is known, just count iterations until log(x) > 100*log(10).
 * Use double to track log(x).
 */

/* Big integer: array of uint64_t, base 10^18 */
#define BIGBASE 1000000000000000000ULL
#define BIGLEN 8  /* enough for ~144 digits */

typedef struct {
    uint64_t d[BIGLEN];
    int len;
} BigInt;

void big_from_ll(BigInt *a, ll val) {
    memset(a, 0, sizeof(*a));
    if (val == 0) { a->len = 1; return; }
    a->len = 0;
    uint64_t v = (uint64_t)val;
    while (v > 0) {
        a->d[a->len++] = v % BIGBASE;
        v /= BIGBASE;
    }
}

/* a = b * scalar */
void big_mul_scalar(BigInt *a, const BigInt *b, ll scalar) {
    __int128 carry = 0;
    a->len = b->len;
    for (int i = 0; i < b->len; i++) {
        __int128 prod = (__int128)b->d[i] * scalar + carry;
        a->d[i] = (uint64_t)(prod % BIGBASE);
        carry = prod / BIGBASE;
    }
    while (carry > 0) {
        a->d[a->len++] = (uint64_t)(carry % BIGBASE);
        carry /= BIGBASE;
    }
}

/* a = b + c */
void big_add(BigInt *a, const BigInt *b, const BigInt *c) {
    int maxlen = b->len > c->len ? b->len : c->len;
    uint64_t carry = 0;
    a->len = maxlen;
    for (int i = 0; i < maxlen; i++) {
        __int128 sum = carry;
        if (i < b->len) sum += b->d[i];
        if (i < c->len) sum += c->d[i];
        a->d[i] = (uint64_t)(sum % BIGBASE);
        carry = (uint64_t)(sum / BIGBASE);
    }
    if (carry) a->d[a->len++] = carry;
}

/* Compare: return -1, 0, 1 */
int big_cmp(const BigInt *a, const BigInt *b) {
    if (a->len != b->len) return a->len < b->len ? -1 : 1;
    for (int i = a->len - 1; i >= 0; i--) {
        if (a->d[i] != b->d[i]) return a->d[i] < b->d[i] ? -1 : 1;
    }
    return 0;
}

/* Check if x is even: x mod 2 == 0 */
int big_is_even(const BigInt *a) {
    return (a->d[0] % 2) == 0;
}

int main() {
    /* N = 10^100 as BigInt */
    BigInt N_limit;
    memset(&N_limit, 0, sizeof(N_limit));
    /* 10^100 = 10^(18*5 + 10) = (10^18)^5 * 10^10 */
    /* d[0] = 0, d[1] = 0, ..., d[5] = 10^10 */
    /* Actually: 10^100 / (10^18)^k for each limb */
    /* 10^100 = 10^18 * 10^82 = ... */
    /* Simpler: construct 10^100 by repeated multiplication */
    big_from_ll(&N_limit, 1);
    for (int i = 0; i < 100; i++) {
        BigInt tmp;
        big_mul_scalar(&tmp, &N_limit, 10);
        N_limit = tmp;
    }

    /* 2 * N_limit for comparing x <= 2*N (since x = 2c, c <= N means x <= 2N) */
    BigInt two_N;
    big_mul_scalar(&two_N, &N_limit, 2);

    int K = 100;
    int ans = 0;

    for (int d = 1; d <= K; d++) {
        /* Solve x^2 - 3*y^2 = d^2 */
        /* Particular solution: (x0, y0) where x0^2 - 3*y0^2 = d^2 */
        /* Try y=0..999 */
        /* Actually, (d, 0) is always a solution. */
        /* Fundamental solution for x^2 - 3*y^2 = 1 is (2, 1). */
        /* Recurrence: x' = 2*x + 3*y, y' = x + 2*y */

        BigInt x, y;
        big_from_ll(&x, d);
        big_from_ll(&y, 0);

        /* Generate solutions until x > 2*N */
        while (big_cmp(&x, &two_N) <= 0) {
            /* Check if x is even */
            if (big_is_even(&x)) {
                /* c = x/2 (we know c <= N since x <= 2N) */
                /* Check y > d (which means a > 0) */
                BigInt d_big;
                big_from_ll(&d_big, d);
                if (big_cmp(&y, &d_big) > 0) {
                    ans++;
                }
            }

            /* Next solution: x' = 2x + 3y, y' = x + 2y */
            BigInt x2, x3y, y2, new_x, new_y;
            big_mul_scalar(&x2, &x, 2);
            big_mul_scalar(&x3y, &y, 3);
            big_add(&new_x, &x2, &x3y);

            big_mul_scalar(&y2, &y, 2);
            big_add(&new_y, &x, &y2);

            x = new_x;
            y = new_y;
        }
    }

    printf("%d\n", ans);
    return 0;
}
