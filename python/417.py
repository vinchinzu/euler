#!/usr/bin/env python3
"""Project Euler Problem 417: Reciprocal cycles II.

Compute sum_{n=3}^{10^8} L(n) where L(n) is the period length of 1/n.

Approach: For n coprime to 10, L(n) = multiplicative order of 10 mod n.
L(2^a * 5^b * m) = L(m) where gcd(m,10) = 1.

For primes p coprime to 10: L(p) = ord_10(p), computed using SPF sieve
to factor p-1 quickly.

For the final sum, we compute L(n) for each n by factoring n (via SPF)
and taking LCM of L(p^e) values.

The key optimization: use SPF to factor p-1 in O(log p) time instead of
O(sqrt(p)) trial division.

Uses C compiled with -O3 for performance.
"""
import os
import subprocess
import tempfile

C_CODE = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define NMAX 100000001

typedef long long ll;
typedef unsigned long long ull;
typedef __uint128_t u128;

/* Use byte-packed SPF: store the index of the smallest prime factor */
/* For numbers up to 10^8, smallest prime factor is at most 10^4 */
/* But we need the actual factor, not just an index */
/* Use int array */
static int spf[NMAX];
static int ord10[NMAX];  /* ord_10(p) for primes p, else 0 */

/* Fast pow_mod for mod < 2^31: use 64-bit for intermediate products */
static inline unsigned int pow_mod_32(unsigned int base, unsigned int exp, unsigned int mod) {
    ull result = 1;
    ull b = base % mod;
    while (exp > 0) {
        if (exp & 1) result = result * b % mod;
        b = b * b % mod;
        exp >>= 1;
    }
    return (unsigned int)result;
}

/* pow_mod for larger moduli */
static inline ll pow_mod_big(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) result = (u128)result * base % mod;
        base = (u128)base * base % mod;
        exp >>= 1;
    }
    return result;
}

static inline ll gcd_ll(ll a, ll b) { while (b) { ll t = b; b = a % b; a = t; } return a; }
static inline ll lcm_ll(ll a, ll b) { if (a == 0) return b; if (b == 0) return a; return a / gcd_ll(a, b) * b; }

int main() {
    int N = 100000000;

    /* Sieve smallest prime factors */
    /* Initialize with 0 (meaning "is prime" if still 0 after sieve, except 0 and 1) */
    memset(spf, 0, sizeof(spf));
    for (int i = 2; (ll)i * i <= N; i++) {
        if (spf[i] == 0) {  /* i is prime */
            for (int j = i * i; j <= N; j += i) {
                if (spf[j] == 0) spf[j] = i;
            }
        }
    }
    /* spf[p] == 0 means p is prime (for p >= 2) */
    /* For composites, spf gives the smallest prime factor */

    /* Compute ord_10(p) for all primes p != 2, 5 */
    memset(ord10, 0, sizeof(ord10));
    for (int p = 3; p <= N; p++) {
        if (spf[p] != 0 || p == 2 || p == 5) continue; /* skip composites and 2, 5 */

        /* Factor p-1 using spf */
        unsigned int result = (unsigned int)(p - 1);
        int temp = p - 1;
        while (temp > 1) {
            int q;
            if (spf[temp] == 0) q = temp;
            else q = spf[temp];

            while (temp % q == 0) temp /= q;
            while (result % q == 0 && pow_mod_32(10, result / q, p) == 1)
                result /= q;
        }
        ord10[p] = (int)result;
    }

    /* Compute sum of L(n) for n = 3..N */
    ll total = 0;
    for (int n = 3; n <= N; n++) {
        int temp = n;
        while (temp % 2 == 0) temp /= 2;
        while (temp % 5 == 0) temp /= 5;
        if (temp <= 1) continue;

        /* If temp is prime, use cached order */
        if (spf[temp] == 0) {
            total += ord10[temp];
            continue;
        }

        /* Factor temp, compute LCM of L(p^e) */
        ll result = 0;
        int t = temp;
        while (t > 1) {
            int p;
            if (spf[t] == 0) p = t;
            else p = spf[t];

            int e = 0;
            while (t % p == 0) { t /= p; e++; }

            ll Lp = ord10[p];
            if (Lp == 0) continue;

            ll Lpe = Lp;
            if (e >= 2) {
                ll pp = (ll)p * p;
                if (pow_mod_big(10, Lp, pp) == 1) {
                    int e0 = 2;
                    ll ppow = pp;
                    for (int i = 3; i <= e; i++) {
                        ppow *= p;
                        if (pow_mod_big(10, Lp, ppow) == 1) e0 = i; else break;
                    }
                    for (int i = e0; i < e; i++) Lpe *= p;
                } else {
                    for (int i = 1; i < e; i++) Lpe *= p;
                }
            }
            result = lcm_ll(result, Lpe);
        }
        total += result;
    }

    printf("%lld\n", total);
    return 0;
}
"""

def solve():
    tmpdir = tempfile.mkdtemp()
    c_file = os.path.join(tmpdir, "p417.c")
    exe_file = os.path.join(tmpdir, "p417")

    with open(c_file, "w") as f:
        f.write(C_CODE)

    subprocess.run(
        ["gcc", "-O3", "-march=native", "-o", exe_file, c_file, "-lm"],
        check=True, capture_output=True
    )

    result = subprocess.run(
        [exe_file], capture_output=True, text=True, check=True,
        timeout=60
    )
    return int(result.stdout.strip())

if __name__ == "__main__":
    print(solve())
