/*
 * Project Euler Problem 234: Semidivisible numbers
 *
 * Find sum of semidivisible numbers up to N = 999966663333.
 * n is semidivisible if exactly one of lps(n), ups(n) divides n,
 * where lps(n) = largest prime <= sqrt(n), ups(n) = smallest prime >= sqrt(n).
 */
#include <stdio.h>
#include <math.h>

#define SIEVE_LIMIT 2000001

static char is_prime[SIEVE_LIMIT];
static int primes[200000];
static int prime_count = 0;

static void sieve(void) {
    for (int i = 0; i < SIEVE_LIMIT; i++) is_prime[i] = 1;
    is_prime[0] = is_prime[1] = 0;
    for (int i = 2; (long long)i * i < SIEVE_LIMIT; i++) {
        if (is_prime[i]) {
            for (int j = i * i; j < SIEVE_LIMIT; j += i)
                is_prime[j] = 0;
        }
    }
    for (int i = 2; i < SIEVE_LIMIT; i++)
        if (is_prime[i])
            primes[prime_count++] = i;
}

static long long round_up(long long n, long long k) {
    return ((n + k - 1) / k) * k;
}

static long long round_down(long long n, long long k) {
    return (n / k) * k;
}

int main(void) {
    long long N = 999966663333LL;
    sieve();

    long long ans = 0;

    for (int i = 0; i < prime_count - 1; i++) {
        long long prev_p = (i > 0) ? primes[i - 1] : 0;
        long long p = primes[i];
        long long next_p = primes[i + 1];

        long long lo = prev_p * prev_p;
        if (lo < 4) lo = 4;
        long long hi = next_p * next_p;
        if (hi > N) hi = N;

        long long min_val = round_up(lo, p);
        long long max_val = round_down(hi, p);

        if (min_val > max_val) continue;

        long long count = (max_val - min_val) / p + 1;
        /* Sum of arithmetic progression: count * (min_val + max_val) / 2 */
        ans += (max_val + min_val) / 2 * count;
        if ((max_val + min_val) % 2 != 0)
            ans += count / 2; /* handle odd sum */

        /* Subtract numbers divisible by both p and prev_p, p, next_p */
        long long vals[3] = {prev_p, p, next_p};
        for (int j = 0; j < 3; j++) {
            long long pq = p * vals[j];
            if (pq >= 4 && pq <= N && pq >= lo && pq <= hi)
                ans -= pq;
        }
    }

    printf("%lld\n", ans);
    return 0;
}
