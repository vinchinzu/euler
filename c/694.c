/*
 * Project Euler 694 - Cube-full Divisors
 *
 * A cube-full number has all prime exponents >= 3.
 * For each cube-full k, floor(N/k) numbers have k as a divisor.
 * Enumerate all cube-full numbers recursively.
 *
 * N = 10^18. Primes up to N^(1/3) ~ 10^6.
 */
#include <stdio.h>
#include <math.h>

typedef long long ll;
typedef unsigned long long ull;
typedef __int128 lll;

#define MAX_PRIMES 80000
static int primes[MAX_PRIMES];
static int nprimes;
static char sieve[1000001];

void build_sieve(int limit) {
    for (int i = 2; i <= limit; i++) sieve[i] = 1;
    for (int i = 2; i * i <= limit; i++)
        if (sieve[i])
            for (int j = i*i; j <= limit; j += i)
                sieve[j] = 0;
    nprimes = 0;
    for (int i = 2; i <= limit; i++)
        if (sieve[i]) primes[nprimes++] = i;
}

static ll N;
static ll total;

void helper(int min_idx, ll k) {
    total += N / k;
    for (int i = min_idx; i < nprimes; i++) {
        ll p = primes[i];
        ll p3 = p * p * p;
        if (k > N / p3) break;
        ll nk = k * p3;
        while (1) {
            helper(i + 1, nk);
            if (nk > N / p) break;
            nk *= p;
        }
    }
}

int main(void) {
    N = 1000000000000000000LL; /* 10^18 */
    int limit = (int)cbrt((double)N) + 2;
    build_sieve(limit);

    total = 0;
    helper(0, 1);

    printf("%lld\n", total);
    return 0;
}
