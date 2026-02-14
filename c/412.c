/*
 * Project Euler Problem 412: Young Tableaux.
 *
 * Using hook formula with Wilson's theorem for computing large factorials mod prime.
 * L(m, n) is an m x m grid with top-right n x n removed.
 * N=10000, K=5000, M=76543217.
 */
#include <stdio.h>
#include <stdlib.h>

typedef long long ll;

#define PRIME 76543217LL

static ll *fact_arr;
static ll *inv_fact_arr;

static ll pow_mod(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) result = result * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return result;
}

static ll mod_inv(ll a, ll m) {
    return pow_mod(a, m - 2, m);
}

int main(void) {
    int N = 10000;
    int K = 5000;
    ll M = PRIME;

    ll L = M - ((ll)N * N - (ll)K * K);

    /* Precompute factorials and inverse factorials up to L */
    fact_arr = (ll *)malloc((L + 1) * sizeof(ll));
    inv_fact_arr = (ll *)malloc((L + 1) * sizeof(ll));

    fact_arr[0] = 1;
    for (ll i = 1; i <= L; i++)
        fact_arr[i] = fact_arr[i-1] * i % M;

    inv_fact_arr[L] = pow_mod(fact_arr[L], M - 2, M);
    for (ll i = L - 1; i >= 0; i--)
        inv_fact_arr[i] = inv_fact_arr[i+1] * (i + 1) % M;

    /* Compute numerator using Wilson's theorem */
    /* (N^2-K^2)! = (-1)^L * (L-1)!^{-1} (mod M) */
    ll parity = (L % 2 == 0) ? 1 : M - 1;
    ll ans = mod_inv(parity * fact_arr[L - 1] % M, M);

    /* Compute denominator using hook lengths */
    /* Top part: rows 0 to K-1 (done twice, s=0 and s=1) */
    for (int s = 0; s < 2; s++) {
        for (int i = 0; i < K; i++) {
            ans = ans * inv_fact_arr[N - K + i] % M;
            ans = ans * fact_arr[i] % M;
        }
    }

    /* Bottom part: rows K to N-1 */
    for (int i = K; i < N; i++) {
        ans = ans * inv_fact_arr[N + i] % M;
        ans = ans * fact_arr[K + i] % M;
    }

    ans = ((ans % M) + M) % M;
    printf("%lld\n", ans);

    free(fact_arr);
    free(inv_fact_arr);
    return 0;
}
