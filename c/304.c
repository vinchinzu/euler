/*
 * Project Euler Problem 304: Primonacci
 *
 * Sum of Fibonacci(a(n)) mod 1234567891011 for n=1..100000,
 * where a(1) is smallest prime > 10^14, a(n+1) is next prime after a(n).
 *
 * Uses segmented sieve + fast doubling Fibonacci.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef long long ll;
typedef unsigned long long ull;
typedef unsigned __int128 u128;

#define MOD 1234567891011LL
#define START 100000000000000LL /* 10^14 */
#define COUNT 100000

static ll mulmod(ll a, ll b, ll m) {
    return (u128)a * b % m;
}

/* Fast doubling Fibonacci mod m */
/* Returns (F(n), F(n+1)) */
static void fib_pair(ll n, ll m, ll *fn, ll *fn1) {
    if (n == 0) { *fn = 0; *fn1 = 1; return; }
    ll a, b;
    fib_pair(n >> 1, m, &a, &b);
    ll c = mulmod(a, ((2*b - a) % m + m) % m, m);
    ll d = (mulmod(a, a, m) + mulmod(b, b, m)) % m;
    if (n & 1) {
        *fn = d;
        *fn1 = (c + d) % m;
    } else {
        *fn = c;
        *fn1 = d;
    }
}

static ll fib_mod(ll n, ll m) {
    ll f, f1;
    fib_pair(n, m, &f, &f1);
    return f;
}

/* Sieve of Eratosthenes for small primes */
#define SMALL_LIMIT 10000100
static int small_primes[700000];
static int nsmall;

static void sieve_small(void) {
    char *is_p = calloc(SMALL_LIMIT, 1);
    memset(is_p, 1, SMALL_LIMIT);
    is_p[0] = is_p[1] = 0;
    for (int i = 2; (ll)i*i < SMALL_LIMIT; i++) {
        if (is_p[i]) {
            for (int j = i*i; j < SMALL_LIMIT; j += i)
                is_p[j] = 0;
        }
    }
    nsmall = 0;
    for (int i = 2; i < SMALL_LIMIT; i++)
        if (is_p[i]) small_primes[nsmall++] = i;
    free(is_p);
}

/* Segmented sieve: find primes in [low, high) */
static ll *seg_sieve(ll low, ll high, int *out_count) {
    ll size = high - low;
    char *sieve = malloc(size);
    memset(sieve, 1, size);
    for (int i = 0; i < nsmall; i++) {
        ll p = small_primes[i];
        if (p * p >= high) break;
        ll start = ((low + p - 1) / p) * p;
        if (start < p * p) start = p * p;
        for (ll j = start - low; j < size; j += p)
            sieve[j] = 0;
    }
    /* Count */
    int cnt = 0;
    for (ll i = 0; i < size; i++)
        if (sieve[i]) cnt++;
    ll *result = malloc(cnt * sizeof(ll));
    int idx = 0;
    for (ll i = 0; i < size; i++)
        if (sieve[i]) result[idx++] = low + i;
    free(sieve);
    *out_count = cnt;
    return result;
}

int main(void) {
    sieve_small();

    /* Find first prime > START */
    ll first = START + 1;
    if (first % 2 == 0) first++;

    /* Estimate range needed: 100000 primes near 10^14, prime gap ~ ln(10^14) ~ 32 */
    ll delta = (ll)((double)COUNT * log((double)START) * 1.5);
    ll high = first + delta;

    int pcnt;
    ll *seg_primes = seg_sieve(first, high, &pcnt);

    if (pcnt < COUNT) {
        fprintf(stderr, "Not enough primes found\n");
        return 1;
    }

    ll total = 0;
    for (int i = 0; i < COUNT; i++) {
        total = (total + fib_mod(seg_primes[i], MOD)) % MOD;
    }

    printf("%lld\n", total);
    free(seg_primes);
    return 0;
}
