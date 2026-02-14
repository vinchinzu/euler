/*
 * Project Euler Problem 549: Divisibility of Factorials.
 * Let s(n) be the smallest m such that n | m!. Find sum_{i=2}^N s(i).
 * N = 10^8.
 *
 * Uses recursive enumeration of prime factorizations with Lucy_Hedgehog
 * algorithm for prime-sum lookups to handle the single-prime tail efficiently.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef long long ll;

#define N_VAL 100000000

static int *primes;
static int nprimes;

/* Lucy Hedgehog arrays */
static ll *small_arr;
static ll *large_arr;
static int sqrt_n;

static void sieve_primes(int limit) {
    char *is_p = (char*)calloc(limit + 1, 1);
    memset(is_p + 2, 1, limit - 1);
    for (int i = 2; (ll)i * i <= limit; i++)
        if (is_p[i])
            for (int j = i * i; j <= limit; j += i)
                is_p[j] = 0;

    primes = (int*)malloc((limit / 5) * sizeof(int));
    nprimes = 0;
    for (int i = 2; i <= limit; i++)
        if (is_p[i]) primes[nprimes++] = i;
    free(is_p);
}

static void lucy_init(void) {
    ll N = N_VAL;
    sqrt_n = (int)sqrt((double)N);

    small_arr = (ll*)malloc((sqrt_n + 2) * sizeof(ll));
    large_arr = (ll*)malloc((sqrt_n + 2) * sizeof(ll));

    for (int i = 1; i <= sqrt_n; i++)
        small_arr[i] = (ll)i * (i + 1) / 2 - 1;
    for (int i = 1; i <= sqrt_n; i++) {
        ll v = N / i;
        large_arr[i] = v * (v + 1) / 2 - 1;
    }

    for (int p = 2; p <= sqrt_n; p++) {
        if (small_arr[p] == small_arr[p - 1]) continue;
        ll sp1 = small_arr[p - 1];
        ll p2 = (ll)p * p;

        int upper = sqrt_n;
        if (N / p2 < upper) upper = (int)(N / p2);
        for (int i = 1; i <= upper; i++) {
            ll ip = (ll)i * p;
            if (ip <= sqrt_n)
                large_arr[i] -= p * (large_arr[(int)ip] - sp1);
            else
                large_arr[i] -= p * (small_arr[(int)(N / ip)] - sp1);
        }

        int limit2 = sqrt_n;
        if (N / p2 < limit2) limit2 = (int)(N / p2);
        for (int i = limit2; i >= (int)p2; i--)
            small_arr[i] -= p * (small_arr[i / p] - sp1);
    }
}

static ll sum_primes_up_to(ll x) {
    if (x <= 0) return 0;
    if (x <= sqrt_n) return small_arr[(int)x];
    ll k = (ll)N_VAL / x;
    return large_arr[(int)k];
}

/* Count factors of p in m! (Legendre's formula) */
static int num_factors_in_factorial(int m, int p) {
    int count = 0;
    ll power = p;
    while (power <= m) {
        count += m / (int)power;
        power *= p;
    }
    return count;
}

static ll ans = 0;

static void helper(int min_index, ll n, int s) {
    if (n > 1)
        ans += s;

    for (int index = min_index; index < nprimes; index++) {
        int p = primes[index];
        if ((ll)p > s && (ll)n * p * p > N_VAL) {
            /* Only single-prime factors remain */
            if ((ll)p <= N_VAL / n) {
                ll sp_upper = sum_primes_up_to(N_VAL / n);
                ll sp_lower = (index > 0) ? sum_primes_up_to(primes[index - 1]) : 0;
                ans += sp_upper - sp_lower;
            }
            return;
        }

        ll new_n = n;
        int e = 1;
        while (1) {
            new_n *= p;
            if (new_n > N_VAL) break;

            int mult = p;
            while (1) {
                if (num_factors_in_factorial(mult, p) >= e) {
                    int new_s = (mult > s) ? mult : s;
                    helper(index + 1, new_n, new_s);
                    break;
                }
                mult += p;
            }
            e++;
        }
    }
}

int main(void) {
    /* Sieve primes up to 2*sqrt(N) */
    int limit = 2 * (int)sqrt((double)N_VAL) + 10;
    sieve_primes(limit);

    lucy_init();

    helper(0, 1, 0);

    printf("%lld\n", ans);

    free(primes);
    free(small_arr);
    free(large_arr);
    return 0;
}
