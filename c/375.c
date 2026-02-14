/*
 * Project Euler Problem 375: Minimum of subsequences
 *
 * BBS RNG: S_0 = 290797, S_n = S_{n-1}^2 mod 50515093
 * A(i,j) = min(S_i, ..., S_j)
 * Find M(N) = sum_{1<=i<=j<=N} A(i,j) for N = 2*10^9.
 *
 * Algorithm:
 * 1. Find period P of BBS sequence
 * 2. Compute M for 3 small multiples of P + (N mod P)
 * 3. Lagrange interpolation to get M(N)
 */

#include <stdio.h>
#include <stdlib.h>

typedef long long ll;

#define BBS_MOD 50515093LL

int main(void) {
    ll N_val = 2000000000LL;

    /* Find period of BBS */
    ll s = 290797;
    s = (s * s) % BBS_MOD;
    ll first = s;
    ll period = 1;
    s = (s * s) % BBS_MOD;
    while (s != first) {
        s = (s * s) % BBS_MOD;
        period++;
    }

    ll start = N_val % period;

    /* Compute M(n) using stack algorithm */
    /* We need to do this 3 times for n = start+period, start+2*period, start+3*period */

    ll points[3];

    for (int mult = 1; mult <= 3; mult++) {
        ll n = start + (ll)mult * period;

        /* Stack: pairs of (position, value) */
        /* Use arrays for stack */
        int stack_cap = 1024;
        ll *stack_pos = (ll *)malloc(stack_cap * sizeof(ll));
        ll *stack_val = (ll *)malloc(stack_cap * sizeof(ll));
        int stack_size = 0;

        /* Sentinel */
        stack_pos[stack_size] = 0;
        stack_val[stack_size] = -1;
        stack_size++;

        ll M = 0;
        ll sv = 290797;

        for (ll pos = 1; pos <= n; pos++) {
            sv = (sv * sv) % BBS_MOD;

            while (stack_val[stack_size - 1] > sv) {
                stack_size--;
                ll v = stack_val[stack_size];
                ll p = stack_pos[stack_size];
                ll prev_p = stack_pos[stack_size - 1];
                M += v * (p - prev_p) * (pos - p);
            }

            if (stack_size >= stack_cap) {
                stack_cap *= 2;
                stack_pos = (ll *)realloc(stack_pos, stack_cap * sizeof(ll));
                stack_val = (ll *)realloc(stack_val, stack_cap * sizeof(ll));
            }
            stack_pos[stack_size] = pos;
            stack_val[stack_size] = sv;
            stack_size++;
        }

        /* Flush remaining */
        ll pos = n + 1;
        while (stack_size > 1) {
            stack_size--;
            ll v = stack_val[stack_size];
            ll p = stack_pos[stack_size];
            ll prev_p = stack_pos[stack_size - 1];
            M += v * (p - prev_p) * (pos - p);
        }

        points[mult - 1] = M;
        free(stack_pos);
        free(stack_val);
    }

    /* Lagrange interpolation: f(x) through (1, y0), (2, y1), (3, y2) */
    ll k = N_val / period;

    ll y0 = points[0], y1 = points[1], y2 = points[2];
    ll d0 = y0;
    ll d1 = y1 - y0;
    ll d2 = y2 - 2 * y1 + y0;

    ll result = d0 + d1 * (k - 1) + d2 * (k - 1) * (k - 2) / 2;

    printf("%lld\n", result);
    return 0;
}
