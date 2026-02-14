#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
#define MOD 1000000007LL

ll power_mod(ll base, ll exp, ll mod) {
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

int triangular(int k) {
    return k * (k + 1) / 2;
}

/* Build sigma permutation for given m (1-indexed) */
void build_sigma(int m, int *sigma) {
    int n = triangular(m);
    for (int i = 0; i < n; i++) sigma[i] = i + 1;
    for (int k = 1; k <= m; k++) {
        int pos = triangular(k);
        sigma[pos - 1] = triangular(k - 1) + 1;
    }
}

/* Build tau permutation (1-indexed) */
void build_tau(int m, int *tau) {
    int n = triangular(m);
    ll a = 1000000007LL;
    for (int i = 1; i <= n; i++) {
        tau[i - 1] = (int)((a * i) % n) + 1;
    }
}

/* Compose two permutations: result[i] = p1[p2[i]-1] (1-indexed) */
void compose(int *p1, int *p2, int *result, int n) {
    for (int i = 0; i < n; i++) {
        result[i] = p1[p2[i] - 1];
    }
}

/* Compute rank of permutation using Lehmer code mod MOD */
ll rank_perm(int *perm, int n, ll *fact) {
    /* Use a BIT (Fenwick tree) for efficient counting */
    int *bit = (int *)calloc(n + 2, sizeof(int));

    /* Initialize: all positions available */
    for (int i = 1; i <= n; i++) {
        /* Update BIT: add 1 at position i */
        for (int j = i; j <= n; j += j & (-j))
            bit[j]++;
    }

    ll rank = 0;
    for (int i = 0; i < n; i++) {
        /* Count elements less than perm[i] that are still available */
        int val = perm[i] - 1; /* Count positions 1..val */
        int count = 0;
        for (int j = val; j > 0; j -= j & (-j))
            count += bit[j];

        rank = (rank + (ll)count % MOD * fact[n - 1 - i]) % MOD;

        /* Remove perm[i] from BIT */
        for (int j = perm[i]; j <= n; j += j & (-j))
            bit[j]--;
    }
    free(bit);
    return (rank + 1) % MOD;
}

/* Compute factorial mod MOD using fact_m = m! mod MOD */
ll factorial_mod(int m) {
    ll result = 1;
    for (int i = 2; i <= m; i++)
        result = result * i % MOD;
    return result;
}

int main(void) {
    int m = 100;
    int n = triangular(m);  /* 5050 */

    int *sigma = (int *)malloc(n * sizeof(int));
    int *tau = (int *)malloc(n * sizeof(int));
    int *tau_inv = (int *)malloc((n + 1) * sizeof(int));
    int *temp = (int *)malloc(n * sizeof(int));
    int *pi = (int *)malloc(n * sizeof(int));

    build_sigma(m, sigma);
    build_tau(m, tau);

    /* Build tau inverse */
    for (int i = 0; i < n; i++) {
        tau_inv[tau[i]] = i + 1;
    }
    /* Shift to 0-indexed array */
    int *tau_inv_arr = (int *)malloc(n * sizeof(int));
    for (int i = 0; i < n; i++) tau_inv_arr[i] = tau_inv[i + 1];

    /* pi = tau_inv o sigma o tau */
    compose(sigma, tau, temp, n);
    compose(tau_inv_arr, temp, pi, n);

    /* Precompute factorials mod MOD */
    ll *fact = (ll *)malloc((n + 1) * sizeof(ll));
    fact[0] = 1;
    for (int i = 1; i <= n; i++)
        fact[i] = fact[i - 1] * i % MOD;

    /* Find order of pi and collect ranks */
    int *identity = (int *)malloc(n * sizeof(int));
    for (int i = 0; i < n; i++) identity[i] = i + 1;

    int *current = (int *)malloc(n * sizeof(int));
    memcpy(current, pi, n * sizeof(int));

    /* Collect ranks in a dynamic array */
    int cap = 1024;
    ll *ranks = (ll *)malloc(cap * sizeof(ll));
    int d = 1;
    ranks[0] = rank_perm(current, n, fact);

    int *next_perm = (int *)malloc(n * sizeof(int));

    while (1) {
        /* Check if current == identity */
        int is_id = 1;
        for (int i = 0; i < n; i++) {
            if (current[i] != i + 1) { is_id = 0; break; }
        }
        if (is_id) break;

        compose(current, pi, next_perm, n);
        memcpy(current, next_perm, n * sizeof(int));
        d++;

        if (d > cap) {
            cap *= 2;
            ranks = (ll *)realloc(ranks, cap * sizeof(ll));
        }
        ranks[d - 1] = rank_perm(current, n, fact);
    }

    /* Compute sum efficiently */
    /* fact_m = m! mod MOD */
    ll fact_m = factorial_mod(m);
    ll q_val = fact_m;
    /* q = fact_m / d, r = fact_m % d -- but fact_m is mod MOD...
       We need the actual m! / d and m! % d.
       m = 100, so m! is huge. We need m! mod d and m!/d mod MOD.
       Since d divides the order which divides m!, we know d | m!.
       So r = 0 and q = m!/d.

       To compute q mod MOD: q = m! / d mod MOD = m! * d^{-1} mod MOD.
       But we need actual m! mod MOD, and m!/d mod MOD.
    */
    /* Actually, the Python code computes q = fact_m // d and r = fact_m % d
       using actual big integers. Since d | m! (because the order d divides m!),
       r = 0.

       So total = q * sum_ranks mod MOD.
       q = m!/d, computed as factorial_mod(m) * power_mod(d, MOD-2, MOD) mod MOD.
    */
    ll sum_r = 0;
    for (int i = 0; i < d; i++)
        sum_r = (sum_r + ranks[i]) % MOD;

    /* q = m! / d mod MOD */
    ll q_mod = factorial_mod(m) * power_mod(d, MOD - 2, MOD) % MOD;

    /* r = m! % d. Since d | m!, r = 0, so sum_first_r = 0 */
    ll total = q_mod * sum_r % MOD;

    printf("%lld\n", total);

    free(sigma);
    free(tau);
    free(tau_inv);
    free(tau_inv_arr);
    free(temp);
    free(pi);
    free(fact);
    free(identity);
    free(current);
    free(next_perm);
    free(ranks);
    return 0;
}
