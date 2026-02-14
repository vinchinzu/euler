/*
 * Project Euler 622: Riffle Shuffles
 *
 * Find sum of all even n such that a deck of n cards returns to original
 * after exactly 60 perfect riffle shuffles.
 *
 * A perfect riffle shuffle on n cards has order = multiplicative order of 2 mod (n-1).
 * So we need order(2, n-1) = 60, meaning n-1 | 2^60 - 1.
 * Enumerate all divisors of 2^60 - 1 and check each.
 */
#include <stdio.h>
#include <stdlib.h>

typedef long long ll;
typedef unsigned long long ull;
typedef __int128 lll;

/* 2^60 - 1 = 1152921504606846975 */
/* Prime factorization: 3 * 5^2 * 7 * 11 * 13 * 31 * 41 * 61 * 151 * 331 * 1321 */
/* Wait, let me factor it properly:
 * 2^60 - 1 = (2^30-1)(2^30+1)
 * 2^30-1 = 1073741823 = 3 * 7 * 11 * 31 * 151 * 331
 * 2^30+1 = 1073741825 = 5^2 * 13 * 41 * 61 * 1321
 */

static ll primes[] = {3, 5, 7, 11, 13, 31, 41, 61, 151, 331, 1321};
static int exponents[] = {1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1};
static int nprimes = 11;

ll powmod(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) result = (lll)result * base % mod;
        base = (lll)base * base % mod;
        exp >>= 1;
    }
    return result;
}

/* Compute multiplicative order of 2 modulo n */
ll order2(ll n) {
    if (n <= 1) return 1;
    /* First compute phi(n) by trial division with small primes */
    ll phi = n;
    ll temp = n;
    for (int i = 0; i < nprimes; i++) {
        ll p = primes[i];
        if (p * p > temp) break;
        if (temp % p == 0) {
            phi = phi / p * (p - 1);
            while (temp % p == 0) temp /= p;
        }
    }
    if (temp > 1) phi = phi / temp * (temp - 1);

    /* Now find the order: smallest d | phi such that 2^d = 1 mod n */
    ll result = phi;

    /* Factor phi */
    ll phi_factors[64];
    int phi_exps[64];
    int nfactors = 0;
    temp = phi;
    for (ll p = 2; p * p <= temp; p++) {
        if (temp % p == 0) {
            phi_factors[nfactors] = p;
            phi_exps[nfactors] = 0;
            while (temp % p == 0) {
                phi_exps[nfactors]++;
                temp /= p;
            }
            nfactors++;
        }
    }
    if (temp > 1) {
        phi_factors[nfactors] = temp;
        phi_exps[nfactors] = 1;
        nfactors++;
    }

    for (int i = 0; i < nfactors; i++) {
        for (int j = 0; j < phi_exps[i]; j++) {
            if (powmod(2, result / phi_factors[i], n) == 1) {
                result /= phi_factors[i];
            } else {
                break;
            }
        }
    }
    return result;
}

/* Enumerate divisors of 2^60-1 */
ll ans = 0;

void enumerate_divisors(int idx, ll d) {
    if (idx == nprimes) {
        if (d > 1 && order2(d) == 60) {
            ans += d + 1;
        }
        return;
    }
    ll pp = 1;
    for (int e = 0; e <= exponents[idx]; e++) {
        enumerate_divisors(idx + 1, d * pp);
        pp *= primes[idx];
    }
}

int main() {
    enumerate_divisors(0, 1);
    printf("%lld\n", ans);
    return 0;
}
