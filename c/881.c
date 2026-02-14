#include <stdio.h>
#include <limits.h>

/* Primes up to 100 */
int primes[] = {2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47,
                53, 59, 61, 67, 71, 73, 79, 83, 89, 97};
int num_primes = 25;

#define N_TARGET 10000
#define MAX_ES 30

typedef long long ll;
typedef unsigned long long ull;
typedef __int128 i128;

ll best;
int es[MAX_ES];
int es_len;

/* DP to compute number of ways to pick half from sum_e items
   with per-group limits */
long long compute_g(int sum_e) {
    int half = sum_e / 2;
    /* Use 1D DP array */
    long long dp[half + 1];
    for (int j = 0; j <= half; j++) dp[j] = 0;
    dp[0] = 1;
    for (int i = 0; i < es_len; i++) {
        /* Process in reverse to avoid double-counting */
        for (int j = half; j >= 1; j--) {
            for (int k = 1; k <= es[i] && k <= j; k++) {
                dp[j] += dp[j - k];
            }
        }
    }
    return dp[half];
}

void helper(int index, ll n, int sum_e) {
    /* Compute g(n) */
    long long g = compute_g(sum_e);
    if (g > N_TARGET && n < best) {
        best = n;
    }

    if (index >= num_primes) return;

    int p = primes[index];
    for (int e = 1; ; e++) {
        if (es_len > 0 && e > es[es_len - 1])
            break;
        /* Check overflow: n * p^e */
        ll power = 1;
        int overflow = 0;
        for (int j = 0; j < e; j++) {
            if (power > (ll)1e18 / p) { overflow = 1; break; }
            power *= p;
        }
        if (overflow) break;
        if (n > (ll)1e18 / power) break;
        ll new_n = n * power;
        if (new_n >= best) break;

        es[es_len] = e;
        es_len++;
        helper(index + 1, new_n, sum_e + e);
        es_len--;
    }
}

int main(void) {
    best = (ll)1e18;
    es_len = 0;
    helper(0, 1, 0);
    printf("%lld\n", best);
    return 0;
}
