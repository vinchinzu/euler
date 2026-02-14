/*
 * Project Euler Problem 516: 5-smooth totients.
 * Sum of all n <= N where phi(n) is a Hamming number (5-smooth).
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <math.h>

typedef long long ll;
typedef unsigned int uint;
typedef __int128 lll;

#define M_VAL (1ULL << 32)
#define N_VAL 1000000000000LL

/* Miller-Rabin primality test */
uint64_t mulmod(uint64_t a, uint64_t b, uint64_t m) {
    return (unsigned __int128)a * b % m;
}

uint64_t powmod(uint64_t base, uint64_t exp, uint64_t mod) {
    uint64_t result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) result = mulmod(result, base, mod);
        base = mulmod(base, base, mod);
        exp >>= 1;
    }
    return result;
}

int is_prime_64(uint64_t n) {
    if (n < 2) return 0;
    int small_primes[] = {2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37};
    for (int i = 0; i < 12; i++) {
        if (n % small_primes[i] == 0) return n == (uint64_t)small_primes[i];
    }

    uint64_t d = n - 1;
    int s = 0;
    while (d % 2 == 0) { s++; d >>= 1; }

    int witnesses[] = {2, 3, 5, 7, 11, 13, 17};
    for (int w = 0; w < 7; w++) {
        uint64_t a = witnesses[w];
        uint64_t x = powmod(a, d, n);
        if (x == 1 || x == n - 1) continue;
        int found = 0;
        for (int r = 0; r < s - 1; r++) {
            x = mulmod(x, x, n);
            if (x == n - 1) { found = 1; break; }
        }
        if (!found) return 0;
    }
    return 1;
}

/* Hamming numbers generation */
ll *hammings;
int num_hammings;
ll *prefix;  /* prefix sums mod M */

void generate_hammings(ll limit) {
    /* Count first */
    int count = 0;
    for (ll n2 = 1; n2 <= limit; n2 *= 2)
        for (ll n3 = n2; n3 <= limit; n3 *= 3)
            for (ll n5 = n3; n5 <= limit; n5 *= 5)
                count++;

    hammings = (ll*)malloc(count * sizeof(ll));
    num_hammings = 0;
    for (ll n2 = 1; n2 <= limit; n2 *= 2)
        for (ll n3 = n2; n3 <= limit; n3 *= 3)
            for (ll n5 = n3; n5 <= limit; n5 *= 5)
                hammings[num_hammings++] = n5;

    /* Sort */
    int cmp_ll(const void *a, const void *b) {
        ll va = *(const ll*)a, vb = *(const ll*)b;
        return (va > vb) - (va < vb);
    }
    qsort(hammings, num_hammings, sizeof(ll), cmp_ll);

    /* Prefix sums mod M */
    prefix = (ll*)malloc((num_hammings + 1) * sizeof(ll));
    prefix[0] = 0;
    for (int i = 0; i < num_hammings; i++)
        prefix[i + 1] = (prefix[i] + (hammings[i] % M_VAL)) % M_VAL;
}

int bisect_right(ll *arr, int n, ll val) {
    int lo = 0, hi = n;
    while (lo < hi) {
        int mid = (lo + hi) / 2;
        if (arr[mid] <= val) lo = mid + 1;
        else hi = mid;
    }
    return lo;
}

/* Good primes: p > 5 with p-1 Hamming */
ll *good_primes;
int num_good_primes;

void find_good_primes(ll limit) {
    int max_gp = 10000;
    good_primes = (ll*)malloc(max_gp * sizeof(ll));
    num_good_primes = 0;

    for (int i = 0; i < num_hammings; i++) {
        ll p = hammings[i] + 1;
        if (p > 5 && p <= limit && is_prime_64(p)) {
            if (num_good_primes >= max_gp) {
                max_gp *= 2;
                good_primes = (ll*)realloc(good_primes, max_gp * sizeof(ll));
            }
            good_primes[num_good_primes++] = p;
        }
    }

    int cmp_ll(const void *a, const void *b) {
        ll va = *(const ll*)a, vb = *(const ll*)b;
        return (va > vb) - (va < vb);
    }
    qsort(good_primes, num_good_primes, sizeof(ll), cmp_ll);
}

/* Generate products and accumulate answer */
ll global_ans = 0;

void gen_products(int start, ll prod) {
    /* For this product, add g * sum(h) for h <= N/g */
    ll limit = N_VAL / prod;
    int idx = bisect_right(hammings, num_hammings, limit);
    ll sum_h = prefix[idx];
    global_ans = (global_ans + (prod % M_VAL) * sum_h) % M_VAL;

    for (int i = start; i < num_good_primes; i++) {
        ll p = good_primes[i];
        /* Check overflow */
        if (p > N_VAL / prod) break;
        ll new_prod = prod * p;
        if (new_prod > N_VAL) break;
        gen_products(i + 1, new_prod);
    }
}

int main() {
    generate_hammings(N_VAL);
    find_good_primes(N_VAL);

    gen_products(0, 1);

    printf("%lld\n", global_ans % M_VAL);

    free(hammings);
    free(prefix);
    free(good_primes);
    return 0;
}
