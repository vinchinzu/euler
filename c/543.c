/*
 * Project Euler Problem 543: Prime-Sum Numbers.
 * Let P(n,k) = 1 if n can be written as sum of k primes.
 * S(n) = sum P(i,k) for 1<=i,k<=n.
 * Find sum_{n=3}^{44} S(F_n) where F_n = nth Fibonacci number.
 *
 * Key insight: S(n) = pi(n) + (n/2 - 1) + (pi(n-2) - 1) + (n+1)*(n/2-2) - 2*(tr(n/2)-3)
 * for n >= 4, where pi(n) = number of primes <= n.
 *
 * F_44 = 701408733, so we sieve up to that.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef long long ll;

/* Sieve: we need prime counting up to ~701M.
 * Use segmented sieve to count primes. Actually, since we need pi(n) for
 * specific values, we can sieve up to F_44 and build a prefix count.
 * But that's ~700MB for a byte array. Use a bitarray instead.
 */

#define MAX_FIB 701408733LL

static unsigned char *sieve_bits = NULL;
static ll sieve_size;

static inline int is_prime_bit(ll n) {
    if (n < 2) return 0;
    return (sieve_bits[n >> 3] >> (n & 7)) & 1;
}

static inline void clear_bit(ll n) {
    sieve_bits[n >> 3] &= ~(1 << (n & 7));
}

static void do_sieve(ll limit) {
    sieve_size = limit + 1;
    ll bytes = (sieve_size + 7) / 8;
    sieve_bits = (unsigned char*)malloc(bytes);
    memset(sieve_bits, 0xFF, bytes);

    /* Clear 0 and 1 */
    clear_bit(0);
    clear_bit(1);

    ll sq = (ll)sqrt((double)limit) + 1;
    for (ll i = 2; i <= sq; i++) {
        if (is_prime_bit(i)) {
            for (ll j = i * i; j <= limit; j += i)
                clear_bit(j);
        }
    }
}

/* Count primes <= n using popcount on the sieve */
static ll count_primes(ll n) {
    if (n < 2) return 0;
    if (n >= sieve_size) n = sieve_size - 1;

    ll count = 0;
    ll full_bytes = n / 8;

    for (ll i = 0; i < full_bytes; i++)
        count += __builtin_popcount(sieve_bits[i]);

    /* Remaining bits */
    int rem = (int)(n % 8) + 1;
    unsigned char mask = (1 << rem) - 1;
    count += __builtin_popcount(sieve_bits[full_bytes] & mask);

    return count;
}

static ll triangular(ll n) {
    return n * (n + 1) / 2;
}

static ll compute_S(ll n) {
    ll result = count_primes(n);
    if (n >= 4) {
        result += n / 2 - 1;
        result += count_primes(n - 2) - 1;
        ll half = n / 2;
        if (half >= 3) {
            result += (n + 1) * (half - 2) - 2 * (triangular(half) - 3);
        }
    }
    return result;
}

int main(void) {
    int N_fib = 44;

    /* Compute Fibonacci numbers */
    ll fibs[45];
    fibs[0] = 0; fibs[1] = 1;
    for (int i = 2; i <= N_fib; i++)
        fibs[i] = fibs[i-1] + fibs[i-2];

    ll max_fib = fibs[N_fib];

    /* Sieve up to max_fib */
    do_sieve(max_fib + 10);

    ll ans = 0;
    for (int k = 3; k <= N_fib; k++) {
        ans += compute_S(fibs[k]);
    }

    printf("%lld\n", ans);

    free(sieve_bits);
    return 0;
}
