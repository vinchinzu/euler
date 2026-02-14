/*
 * Project Euler Problem 581: Stormer numbers.
 *
 * Find the sum of all n such that tr(n)=n(n+1)/2 is N-smooth (N=47).
 * Uses Stormer's theorem: solve Pell equations x^2 - 2q*y^2 = 1
 * for each product q of a subset of odd primes <= 47.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <math.h>

typedef __int128 i128;
typedef long long ll;
typedef unsigned long long ull;

/* Primes up to 47 */
static int primes[] = {2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47};
static int num_primes = 15;

static int isqrt128(i128 n) {
    if (n <= 0) return 0;
    /* Use Newton's method */
    i128 x = 1;
    while (x * x <= n) x <<= 1;
    /* x is now > sqrt(n), binary search or Newton's */
    i128 lo = x >> 1, hi = x;
    while (lo < hi) {
        i128 mid = (lo + hi) / 2;
        if (mid * mid <= n) lo = mid + 1;
        else hi = mid;
    }
    return (int)(lo - 1);
}

/* Check if n is a perfect square (for int) */
int is_square(ll n) {
    if (n < 0) return 0;
    ll r = (ll)sqrtl((long double)n);
    if (r > 0 && r * r == n) return 1;
    r++;
    if (r * r == n) return 1;
    if (r > 1) { r--; r--; if (r * r == n) return 1; }
    return 0;
}

/* Check if n is smooth with respect to the given primes */
int is_smooth(i128 n) {
    if (n <= 1) return (n == 1);
    for (int i = 0; i < num_primes; i++) {
        while (n % primes[i] == 0) n /= primes[i];
    }
    return (n == 1);
}

/* Pell equation solver: x^2 - d*y^2 = 1 */
/* Returns number of solutions found, stores x values in xs[] */
#define MAX_SOLUTIONS 30

int solve_pell(ll d, i128 *xs) {
    /* Check if d is a perfect square */
    if (is_square(d)) return 0;

    /* Find fundamental solution using continued fractions */
    ll a0 = (ll)sqrtl((long double)d);
    ll m = 0, den = 1, a = a0;

    i128 h_prev = 1, k_prev = 0;
    i128 h = a0, k = 1;

    i128 x0 = 0, y0 = 0;
    int found = 0;

    for (int iter = 0; iter < 100000; iter++) {
        if (h * h - (i128)d * k * k == 1) {
            x0 = h;
            y0 = k;
            found = 1;
            break;
        }

        m = den * a - m;
        den = (d - m * m) / den;
        a = (a0 + m) / den;

        i128 h_next = a * h + h_prev;
        i128 k_next = a * k + k_prev;

        h_prev = h; k_prev = k;
        h = h_next; k = k_next;
    }

    if (!found) return 0;

    /* Generate solutions using recurrence */
    int count = 0;
    xs[count++] = x0;

    i128 x = x0, y = y0;
    int max_sol = 24;  /* (47+1)/2 */

    for (int i = 1; i < max_sol; i++) {
        i128 x_next = x0 * x + (i128)d * y0 * y;
        i128 y_next = x0 * y + y0 * x;
        x = x_next;
        y = y_next;

        /* Check if x fits in reasonable range (128-bit gives us plenty) */
        /* But check bit length - stop at ~64 bits */
        i128 limit = (i128)1 << 64;
        if (x > limit) break;

        xs[count++] = x;
    }

    return count;
}

/* Hash set for storing found values */
#define HASH_SIZE 100003
typedef struct Node {
    i128 val;
    struct Node *next;
} Node;

Node *hash_table[HASH_SIZE];

int hash_contains(i128 val) {
    int h = (int)((ull)val % HASH_SIZE);
    for (Node *n = hash_table[h]; n; n = n->next) {
        if (n->val == val) return 1;
    }
    return 0;
}

void hash_insert(i128 val) {
    if (hash_contains(val)) return;
    int h = (int)((ull)val % HASH_SIZE);
    Node *n = (Node *)malloc(sizeof(Node));
    n->val = val;
    n->next = hash_table[h];
    hash_table[h] = n;
}

int main() {
    memset(hash_table, 0, sizeof(hash_table));

    /* Iterate over all subsets of primes (excluding subset {2} alone) */
    for (int subset = 0; subset < (1 << num_primes); subset++) {
        if (subset == 1) continue;  /* Skip subset containing only 2 */

        /* Compute q as product of primes in subset */
        ll q = 1;
        for (int i = 0; i < num_primes; i++) {
            if (subset & (1 << i)) {
                q *= primes[i];
                /* Check for overflow - product of all primes <= 47 is huge */
                if (q > (ll)1e15) {
                    q = -1;
                    break;
                }
            }
        }
        if (q < 0) continue;

        /* Solve Pell equation x^2 - 2q*y^2 = 1 */
        ll d = 2 * q;
        i128 xs[MAX_SOLUTIONS];
        int nsol = solve_pell(d, xs);

        for (int s = 0; s < nsol; s++) {
            i128 x = xs[s];
            /* x is odd, check b = x >> 1 (integer division) */
            i128 b = x >> 1;
            if (b > 0 && is_smooth(b) && is_smooth(b + 1)) {
                hash_insert(b);
            }
        }
    }

    /* Sum all values in hash set */
    i128 total = 0;
    for (int h = 0; h < HASH_SIZE; h++) {
        for (Node *n = hash_table[h]; n; n = n->next) {
            total += n->val;
        }
    }

    /* Print result - need to handle 128-bit printing */
    ll hi = (ll)(total / 1000000000000LL);
    ll lo = (ll)(total % 1000000000000LL);
    if (hi > 0) {
        printf("%lld%012lld\n", hi, lo);
    } else {
        printf("%lld\n", lo);
    }

    return 0;
}
