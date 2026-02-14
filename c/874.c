#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

/*
 * Project Euler 874 - Prime Score
 *
 * M(7000, p(7000)): max prime score with sum of indices divisible by k=7000.
 * Start with all a_i = k-1, adjust remainder using knapsack DP.
 */

typedef long long ll;

#define MAX_PRIMES 7002
#define SIEVE_LIMIT 100000

int is_prime_arr[SIEVE_LIMIT + 1];
int primes[MAX_PRIMES];
int num_primes;

void sieve(void) {
    memset(is_prime_arr, 1, sizeof(is_prime_arr));
    is_prime_arr[0] = is_prime_arr[1] = 0;
    for (int i = 2; i * i <= SIEVE_LIMIT; i++) {
        if (is_prime_arr[i]) {
            for (int j = i * i; j <= SIEVE_LIMIT; j += i)
                is_prime_arr[j] = 0;
        }
    }
    num_primes = 0;
    for (int i = 2; i <= SIEVE_LIMIT && num_primes < MAX_PRIMES; i++) {
        if (is_prime_arr[i]) primes[num_primes++] = i;
    }
}

ll cost[7001]; /* cost[d] = p(k-1) - p(k-1-d) */
ll dp[7001];   /* dp[w] = min cost to achieve reduction w */

int main(void) {
    sieve();

    int k = 7000;
    ll n = primes[k]; /* p(7000) = 7001st prime (0-indexed) */
    ll p_max = primes[k - 1]; /* p(k-1) */

    ll current_sum_indices = n * (ll)(k - 1);
    int remainder = (int)(current_sum_indices % k);

    if (remainder == 0) {
        printf("%lld\n", n * p_max);
        return 0;
    }

    int target_R = remainder;

    /* Precompute costs */
    for (int d = 1; d <= target_R; d++) {
        cost[d] = primes[k - 1] - primes[k - 1 - d];
    }

    /* DP */
    for (int i = 0; i <= target_R; i++) dp[i] = (ll)1e18;
    dp[0] = 0;

    for (int w = 1; w <= target_R; w++) {
        ll min_c = cost[w]; /* single reduction of size w */

        /* Try splitting at j */
        for (int j = 1; j <= w / 2; j++) {
            ll c = dp[j] + dp[w - j];
            if (c < min_c) min_c = c;
        }
        dp[w] = min_c;
    }

    ll max_score = n * p_max - dp[target_R];
    printf("%lld\n", max_score);
    return 0;
}
