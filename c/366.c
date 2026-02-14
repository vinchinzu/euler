/*
 * Project Euler Problem 366 - Stone game with restricted moves.
 *
 * M(n) = largest first move guaranteeing a win, or 0 if losing position.
 * Find sum of M(n) for n=1 to 10^18, modulo 10^8.
 *
 * Uses Fibonacci-based recursive formula.
 */

#include <stdio.h>

typedef long long ll;

#define MOD 100000000LL
#define MAX_FIBS 100

static ll fibs[MAX_FIBS];
static int nfibs;

static void gen_fibs(ll N) {
    fibs[0] = 1;
    fibs[1] = 2;
    nfibs = 2;
    while (fibs[nfibs - 1] <= N) {
        fibs[nfibs] = fibs[nfibs - 1] + fibs[nfibs - 2];
        nfibs++;
    }
}

static ll tr(ll n) {
    /* Triangular sum: 0+1+2+...+n mod MOD */
    if (n < 0) return 0;
    n = n % MOD;
    if (n % 2 == 0)
        return (n / 2) % MOD * ((n + 1) % MOD) % MOD;
    else
        return (n % MOD) * (((n + 1) / 2) % MOD) % MOD;
}

static ll sum_range(ll start, ll end) {
    if (start > end) return 0;

    /* Find largest Fibonacci <= start */
    ll fibonacci = 1;
    for (int i = 0; i < nfibs; i++) {
        if (fibs[i] <= start) fibonacci = fibs[i];
        else break;
    }

    ll max_identity = (fibonacci - 1) / 2;
    if (max_identity > end - fibonacci) max_identity = end - fibonacci;

    ll result = (tr(max_identity) - tr(start - fibonacci - 1) % MOD + MOD) % MOD;

    result = (result + sum_range(max_identity + 1, end - fibonacci)) % MOD;

    return result;
}

int main(void) {
    ll N = 1000000000000000000LL; /* 10^18 */
    gen_fibs(N);

    ll ans = 0;
    for (int i = 0; i < nfibs - 1; i++) {
        if (fibs[i] > N) break;
        ll start = fibs[i];
        ll end = fibs[i + 1] - 1;
        if (end > N) end = N;
        if (end >= start)
            ans = (ans + sum_range(start, end)) % MOD;
    }

    printf("%lld\n", ans);
    return 0;
}
