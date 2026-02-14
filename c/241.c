/*
 * Project Euler Problem 241: Perfection Quotients
 *
 * Find the sum of all positive integers n <= 10^18 such that sigma(n)/n
 * is of the form (2k+1)/2.
 *
 * Uses recursive search over prime power factorizations with exact
 * rational arithmetic (num/den pairs).
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define NMAX 1000000000000000000LL  /* 10^18 */
#define L 1000000

typedef long long ll;
typedef unsigned long long ull;
typedef __int128 lll;

static int spf[L + 1];  /* smallest prime factor */
static char is_prime[L + 1];
static int primes[80000];
static int nprimes;

static ll ans = 0;

void build_sieve(void) {
    memset(is_prime, 1, sizeof(is_prime));
    is_prime[0] = is_prime[1] = 0;
    for (int i = 2; i < L + 1; i++) spf[i] = i;
    for (int i = 2; (ll)i * i <= L; i++) {
        if (is_prime[i]) {
            for (int j = i * i; j <= L; j += i) {
                is_prime[j] = 0;
                if (spf[j] == j) spf[j] = i;
            }
        }
    }
    nprimes = 0;
    for (int i = 2; i <= L; i++)
        if (is_prime[i])
            primes[nprimes++] = i;
}

/* GCD for long long */
ll gcd(ll a, ll b) {
    if (a < 0) a = -a;
    if (b < 0) b = -b;
    while (b) { ll t = b; b = a % b; a = t; }
    return a;
}

/*
 * Recursive helper: we have built partial product prod_val with remaining
 * fraction r_num/r_den (= target * prod_val / sigma(prod_val)).
 * If r_num/r_den == 1, then prod_val is a solution.
 * We try adding prime powers p^e to the factorization.
 */
void helper(ll prod_val, ll r_num, ll r_den) {
    if (r_num == r_den) {
        ans += prod_val;
        return;
    }

    if (r_den > L) return;

    int p;
    if (r_den < L + 1)
        p = spf[(int)r_den];
    else
        p = (int)r_den;  /* r_den itself is prime if > L */

    if (p > 1 && prod_val % p != 0) {
        ll pe = 1;
        ll mult = 1;  /* 1 + p + p^2 + ... = sigma(p^e) / sigma(1) */
        while (1) {
            /* Check overflow: prod_val * pe * p <= NMAX */
            if (pe > NMAX / p) break;
            pe *= p;
            if (prod_val > NMAX / pe) break;
            mult += pe;

            /* New fraction: r * pe / mult = (r_num * pe) / (r_den * mult) */
            ll new_num = r_num * pe;
            ll new_den = r_den * mult;
            ll g = gcd(new_num, new_den);
            new_num /= g;
            new_den /= g;

            helper(prod_val * pe, new_num, new_den);
        }
    }
}

int main(void) {
    build_sieve();

    /* Compute max perfection quotient for numbers up to NMAX */
    double max_pq = 1.0;
    ll prod = 1;
    for (int i = 0; i < nprimes; i++) {
        int p = primes[i];
        if (prod > NMAX / p) break;
        prod *= p;
        max_pq *= (double)p / (p - 1);
    }

    for (int k = 1; k < (int)max_pq; k++) {
        /* target = k + 1/2 = (2k+1)/2 */
        ll t_num = 2 * k + 1;
        ll t_den = 2;
        helper(1, t_num, t_den);
    }

    printf("%lld\n", ans);
    return 0;
}
