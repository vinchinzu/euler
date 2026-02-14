/*
 * Project Euler 271: Modular Cubes, part 1
 *
 * Find the sum of all cube roots of 1 (mod N), other than 1 itself.
 * N = 13082761331670030 = 2*3*5*7*11*13*17*19*23*29*31*37*41*43
 *
 * For each prime factor p, find all x with x^3 = 1 (mod p).
 * Combine using CRT. Sum all combinations minus 1.
 */
#include <stdio.h>
#include <stdint.h>

typedef long long ll;
typedef __int128 i128;

static ll primes[] = {2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43};
static int num_primes = 14;

/* Cube roots of 1 mod each prime */
static ll cube_roots[14][4];
static int num_roots[14];

static ll ext_gcd(ll a, ll b, ll *x, ll *y) {
    if (b == 0) { *x = 1; *y = 0; return a; }
    ll x1, y1;
    ll g = ext_gcd(b, a % b, &x1, &y1);
    *x = y1;
    *y = x1 - (a / b) * y1;
    return g;
}

/* CRT: combine system of congruences */
static ll crt(ll *remainders, ll *moduli, int n) {
    i128 result = remainders[0];
    i128 mod = moduli[0];

    for (int i = 1; i < n; i++) {
        ll x, y;
        ll g = ext_gcd((ll)mod, moduli[i], &x, &y);
        i128 diff = remainders[i] - result;
        /* Ensure diff is divisible by g (it always is since g=1 for distinct primes) */
        result = result + (i128)mod * x % ((i128)mod * moduli[i] / g) * (diff / g);
        mod = (i128)mod * moduli[i] / g;
        result = ((result % mod) + mod) % mod;
    }
    return (ll)result;
}

/* Print __int128 */
static void print_i128(i128 v) {
    if (v == 0) { printf("0"); return; }
    if (v < 0) { printf("-"); v = -v; }
    char buf[50];
    int pos = 0;
    while (v > 0) { buf[pos++] = '0' + (int)(v % 10); v /= 10; }
    for (int i = pos - 1; i >= 0; i--) putchar(buf[i]);
}

int main(void) {
    /* Find cube roots of 1 mod each prime */
    for (int i = 0; i < num_primes; i++) {
        ll p = primes[i];
        num_roots[i] = 0;
        for (ll x = 0; x < p; x++) {
            if ((x * x % p) * x % p == 1) {
                cube_roots[i][num_roots[i]++] = x;
            }
        }
    }

    /* Enumerate all combinations and CRT */
    /* Total combinations = product of num_roots[i] */
    /* For primes = 1 mod 3: 3 cube roots. For others: 1. */
    /* Total = 3^8 = 6561 (primes 7,13,19,31,37,43 are 1 mod 3, plus maybe others) */

    i128 total = 0;
    ll rems[14];

    /* Recursive enumeration */
    int indices[14];
    for (int i = 0; i < num_primes; i++) indices[i] = 0;

    while (1) {
        for (int i = 0; i < num_primes; i++)
            rems[i] = cube_roots[i][indices[i]];
        total += (i128)crt(rems, primes, num_primes);

        /* Increment indices */
        int carry = 1;
        for (int i = num_primes - 1; i >= 0 && carry; i--) {
            indices[i]++;
            if (indices[i] >= num_roots[i]) {
                indices[i] = 0;
            } else {
                carry = 0;
            }
        }
        if (carry) break;
    }

    total -= 1; /* exclude 1 itself */
    print_i128(total);
    printf("\n");
    return 0;
}
