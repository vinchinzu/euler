/*
 * Project Euler 141 - Progressive perfect squares below 10^12
 *
 * For n = q*d + r with d > r, the sequence r, d, n/d forms a geometric
 * progression. Parameterize with coprime p > q: d = a*p*q, r = a*q^2,
 * n/d = a*p^2, so n = a^2*p^3*q + a*q^2.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <stdbool.h>

typedef long long ll;
typedef unsigned long long ull;

static ll gcd(ll a, ll b) {
    while (b) { ll t = b; b = a % b; a = t; }
    return a;
}

static bool is_square(ll v) {
    if (v < 0) return false;
    ll r = (ll)sqrt((double)v);
    /* adjust for floating point error */
    while (r * r > v) r--;
    while ((r+1)*(r+1) <= v) r++;
    return r * r == v;
}

int main(void) {
    const ll LIMIT = 1000000000000LL;

    /* Use a simple hash set for progressive squares */
    #define HASH_SIZE (1 << 20)
    #define HASH_MASK (HASH_SIZE - 1)
    typedef struct node { ll val; struct node *next; } node;
    static node *table[HASH_SIZE];
    memset(table, 0, sizeof(table));

    int max_q = (int)pow((double)LIMIT, 0.25) + 2;

    for (int q = 1; q <= max_q; q++) {
        int max_p = (int)pow((double)LIMIT / q, 1.0/3.0) + 2;
        for (int p = q + 1; p <= max_p; p++) {
            if (gcd(p, q) != 1) continue;

            ll coeff = (ll)p * p * p * q;
            ll linear = (ll)q * q;

            /* n = coeff * a^2 + linear * a < LIMIT */
            /* Solve coeff*a^2 + linear*a < LIMIT for max a */
            double disc = (double)linear * linear + 4.0 * coeff * (LIMIT - 1);
            int max_a = (int)((-linear + sqrt(disc)) / (2.0 * coeff));
            if (max_a < 1) continue;

            for (int a = 1; a <= max_a; a++) {
                ll n = coeff * (ll)a * a + linear * a;
                if (n >= LIMIT) break;
                if (is_square(n)) {
                    /* Insert into hash set */
                    unsigned int h = (unsigned int)((ull)n * 2654435761ULL) & HASH_MASK;
                    bool found = false;
                    for (node *nd = table[h]; nd; nd = nd->next) {
                        if (nd->val == n) { found = true; break; }
                    }
                    if (!found) {
                        node *nd = malloc(sizeof(node));
                        nd->val = n;
                        nd->next = table[h];
                        table[h] = nd;
                    }
                }
            }
        }
    }

    ll total = 0;
    for (int i = 0; i < HASH_SIZE; i++) {
        for (node *nd = table[i]; nd; nd = nd->next) {
            total += nd->val;
        }
    }
    printf("%lld\n", total);
    return 0;
}
