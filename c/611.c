/*
 * Project Euler Problem 611: Hallway of square steps
 *
 * Count integers up to N expressible as a^2 + b^2 (a < b > 0) in an odd
 * number of ways. Uses Lucy DP sieve for counting primes by residue mod 4.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <math.h>

typedef long long ll;

#define N_VAL 1000000000000LL

int main(void) {
    ll L = (ll)sqrtl((long double)N_VAL);
    /* Correct isqrt */
    while (L * L > N_VAL) L--;
    while ((L + 1) * (L + 1) <= N_VAL) L++;

    /* Sieve primes up to L */
    char *sieve = (char*)calloc(L + 1, 1);
    sieve[0] = sieve[1] = 1;
    for (ll i = 2; i * i <= L; i++) {
        if (!sieve[i]) {
            for (ll j = i * i; j <= L; j += i)
                sieve[j] = 1;
        }
    }

    /* Collect odd primes up to L */
    int sieve_prime_count = 0;
    for (ll i = 3; i <= L; i++)
        if (!sieve[i]) sieve_prime_count++;

    ll *sieve_primes = (ll*)malloc(sieve_prime_count * sizeof(ll));
    int sp_idx = 0;
    for (ll i = 3; i <= L; i++)
        if (!sieve[i]) sieve_primes[sp_idx++] = i;

    /* All primes including 2 for the helper */
    int all_prime_count = sieve_prime_count + 1;
    ll *all_primes = (ll*)malloc(all_prime_count * sizeof(ll));
    all_primes[0] = 2;
    for (int i = 0; i < sieve_prime_count; i++)
        all_primes[i + 1] = sieve_primes[i];

    /* Lucy DP: count primes by residue mod 4 */
    ll big_size = N_VAL / L + 1;

    ll *big0 = (ll*)calloc(big_size, sizeof(ll));
    ll *big1 = (ll*)calloc(big_size, sizeof(ll));
    ll *small0 = (ll*)calloc(L + 1, sizeof(ll));
    ll *small1 = (ll*)calloc(L + 1, sizeof(ll));

    for (ll i = 1; i < big_size; i++) {
        ll v = N_VAL / i;
        big0[i] = (v + 3) / 4;
        big1[i] = (v + 1) / 4;
    }
    for (ll i = 1; i <= L; i++) {
        small0[i] = (i + 3) / 4;
        small1[i] = (i + 1) / 4;
    }

    for (int pi = 0; pi < sieve_prime_count; pi++) {
        ll p = sieve_primes[pi];
        ll p2 = p * p;
        ll sp0 = small0[p - 1];
        ll sp1 = small1[p - 1];
        int mod1 = (p % 4 == 1);

        /* Big array updates */
        for (ll i = 1; i < big_size; i++) {
            ll v = N_VAL / i;
            if (v < p2) break;
            ll ip = i * p;
            ll v0, v1;
            if (ip < big_size) {
                v0 = big0[ip] - sp0;
                v1 = big1[ip] - sp1;
            } else {
                v0 = small0[N_VAL / ip] - sp0;
                v1 = small1[N_VAL / ip] - sp1;
            }
            if (mod1) {
                big0[i] -= v0;
                big1[i] -= v1;
            } else {
                big0[i] -= v1;
                big1[i] -= v0;
            }
        }

        /* Small array updates */
        if (p2 <= L) {
            for (ll i = L; i >= p2; i--) {
                ll v0 = small0[i / p] - sp0;
                ll v1 = small1[i / p] - sp1;
                if (mod1) {
                    small0[i] -= v0;
                    small1[i] -= v1;
                } else {
                    small0[i] -= v1;
                    small1[i] -= v0;
                }
            }
        }
    }

    /* Remove count of 1 (which is 1 mod 4 but not prime) */
    for (ll i = 1; i < big_size; i++) big0[i] -= 1;
    for (ll i = 1; i <= L; i++) small0[i] -= 1;

    /* Quotient value lookup: primes equiv 1 mod 4 */
    #define QV(v) ((v) <= L ? small0[v] : big0[N_VAL / (v)])

    /* Iterate over products of prime powers (stack-based DFS) */
    typedef struct { int min_idx; ll n; ll P; int skip; } Frame;
    int stack_cap = 1 << 20;
    Frame *stack = (Frame*)malloc(stack_cap * sizeof(Frame));
    int stack_top = 0;

    stack[stack_top++] = (Frame){0, 1, 1, 1};
    ll ans = 0;

    while (stack_top > 0) {
        Frame f = stack[--stack_top];
        int min_idx = f.min_idx;
        ll n = f.n;
        ll P = f.P;
        int skip = f.skip;

        ll p0 = all_primes[min_idx];

        if (!skip && ((P + 1) / 2 - P) % 2 != 0) {
            ans += 1;
        }

        if (N_VAL / n >= p0 && P % 2 != 0) {
            ans += QV(N_VAL / n) - small0[p0] + (p0 % 4 == 1 ? 1 : 0);
        }

        for (int idx = min_idx; idx < all_prime_count - 1; idx++) {
            ll p = all_primes[idx];
            if (n * p * p > N_VAL) break;
            int step = (p % 4 == 3) ? 2 : 1;
            int e = step;
            ll pe = 1;
            for (int s = 0; s < step; s++) pe *= p;
            while (n * pe <= N_VAL) {
                ll new_P = P * ((p % 4 == 1) ? (e + 1) : 1);
                int new_skip = (p % 4 == 1 && e == 1);
                if (stack_top >= stack_cap) {
                    stack_cap *= 2;
                    stack = (Frame*)realloc(stack, stack_cap * sizeof(Frame));
                }
                stack[stack_top++] = (Frame){idx + 1, n * pe, new_P, new_skip};
                e += step;
                ll mul = 1;
                for (int s = 0; s < step; s++) mul *= p;
                pe *= mul;
            }
        }
    }

    printf("%lld\n", ans);

    free(sieve);
    free(sieve_primes);
    free(all_primes);
    free(big0); free(big1);
    free(small0); free(small1);
    free(stack);
    return 0;
}
