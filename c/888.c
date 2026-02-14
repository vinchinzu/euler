#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;

#define N_VAL 12491249
#define K 1249
#define M 912491249LL
#define L 25000

int Ds[] = {1, 2, 4, 9};
int num_Ds = 4;

int nimbers[L];

ll power_mod(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) result = result * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return result;
}

int main(void) {
    /* Compute nimbers */
    memset(nimbers, 0, sizeof(nimbers));
    for (int n = 0; n < L; n++) {
        /* Find mex of reachable nimber values */
        /* First, figure out max possible nimber - use a bitset */
        /* Max nimber is bounded by L, use dynamic allocation */
        int used_size = L + 1;
        char *used = (char *)calloc(used_size, 1);

        for (int di = 0; di < num_Ds; di++) {
            int d = Ds[di];
            if (d <= n) {
                if (nimbers[n - d] < used_size)
                    used[nimbers[n - d]] = 1;
            }
        }
        /* Split: pile n into (i, n-i) for i=1..floor((n)/2) */
        for (int i = 1; i <= n / 2; i++) {
            int v = nimbers[i] ^ nimbers[n - i];
            if (v < used_size)
                used[v] = 1;
        }
        /* Also i = (n+1)/2 if n is odd: i = (n+1)/2, n-i = (n-1)/2 */
        if (n > 0 && n % 2 == 1) {
            int i = (n + 1) / 2;
            int v = nimbers[i] ^ nimbers[n - i];
            if (v < used_size)
                used[v] = 1;
        }

        int nimber = 0;
        while (nimber < used_size && used[nimber]) nimber++;
        nimbers[n] = nimber;
        free(used);
    }

    /* Find max nimber and cap */
    int max_nimber = 0;
    for (int i = 0; i < L; i++)
        if (nimbers[i] > max_nimber) max_nimber = nimbers[i];
    int cap = 1;
    while (cap < max_nimber) cap *= 2;

    /* Find period */
    int half = L / 2;
    int period = 0;
    for (int start = 0; start < half; start++) {
        int match = 1;
        for (int j = 0; j < half; j++) {
            if (nimbers[start + j] != nimbers[half + j]) {
                match = 0;
                break;
            }
        }
        if (match) {
            period = half - start;
            break;
        }
    }

    /* Compute counts */
    ll *counts = (ll *)calloc(cap, sizeof(ll));
    for (int n = 1; n < period; n++) {
        counts[nimbers[n]]++;
    }
    for (int n = 0; n < period; n++) {
        counts[nimbers[n + period]] += (N_VAL - n) / period;
    }

    /* Precompute factorials and inverse factorials */
    ll max_val = 0;
    for (int i = 0; i < cap; i++)
        if (counts[i] > max_val) max_val = counts[i];
    max_val += K + 10;

    ll *fact = (ll *)malloc((max_val + 1) * sizeof(ll));
    ll *inv_fact = (ll *)malloc((max_val + 1) * sizeof(ll));
    fact[0] = 1;
    for (ll i = 1; i <= max_val; i++)
        fact[i] = fact[i - 1] * i % M;
    inv_fact[max_val] = power_mod(fact[max_val], M - 2, M);
    for (ll i = max_val - 1; i >= 0; i--)
        inv_fact[i] = inv_fact[i + 1] * (i + 1) % M;

    /* DP: dp[xor_val][num_piles] */
    /* Process one nimber at a time */
    ll **prev_dp = (ll **)malloc(cap * sizeof(ll *));
    for (int i = 0; i < cap; i++) {
        prev_dp[i] = (ll *)calloc(K + 1, sizeof(ll));
    }
    prev_dp[0][0] = 1;

    for (int nimber = 0; nimber < cap; nimber++) {
        ll **next_dp = (ll **)malloc(cap * sizeof(ll *));
        for (int i = 0; i < cap; i++) {
            next_dp[i] = (ll *)calloc(K + 1, sizeof(ll));
        }

        for (int d = 0; d <= K; d++) {
            ll mult;
            if (d == 0)
                mult = 1;
            else {
                /* nCr(counts[nimber] + d - 1, d) */
                ll cn = counts[nimber] + d - 1;
                if (cn < d || cn < 0)
                    mult = 0;
                else if (cn <= max_val)
                    mult = fact[cn] % M * inv_fact[d] % M * inv_fact[cn - d] % M;
                else {
                    /* Compute falling factorial */
                    mult = 1;
                    for (int j = 0; j < d; j++) {
                        mult = mult * ((counts[nimber] + d - 1 - j) % M) % M;
                    }
                    mult = mult * inv_fact[d] % M;
                }
            }
            if (mult == 0) continue;

            int new_n_xor;
            for (int n = 0; n < cap; n++) {
                if (d % 2 == 0)
                    new_n_xor = n;
                else
                    new_n_xor = n ^ nimber;
                for (int k = 0; k + d <= K; k++) {
                    if (prev_dp[n][k]) {
                        next_dp[new_n_xor][k + d] = (next_dp[new_n_xor][k + d] + mult * prev_dp[n][k]) % M;
                    }
                }
            }
        }

        for (int i = 0; i < cap; i++) free(prev_dp[i]);
        free(prev_dp);
        prev_dp = next_dp;
    }

    printf("%lld\n", prev_dp[0][K]);

    for (int i = 0; i < cap; i++) free(prev_dp[i]);
    free(prev_dp);
    free(counts);
    free(fact);
    free(inv_fact);
    return 0;
}
