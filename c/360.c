/* Project Euler 360 - Scary Sphere
 *
 * S(r) = sum of |x|+|y|+|z| for all integer (x,y,z) on x^2+y^2+z^2=r^2.
 * S(2r) = 2*S(r), so S(10^10) = 2^10 * S(5^10).
 *
 * Extracted from embedded C in Python solution.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef __int128 i128;

#define R5 9765625LL
#define MAX_N 19531260LL
#define PRIME_LIM 5000

static int primes[700];
static int nprimes;

void build_primes(void) {
    nprimes = 0;
    char sieve[PRIME_LIM];
    memset(sieve, 0, sizeof(sieve));
    for (int i = 2; i < PRIME_LIM; i++) {
        if (!sieve[i]) {
            primes[nprimes++] = i;
            for (int j = i*i; j < PRIME_LIM; j += i)
                sieve[j] = 1;
        }
    }
}

static short f_no5[MAX_N];
static signed char v5[MAX_N];
static int cof[MAX_N];

void compute_sieve(void) {
    for (ll n = 0; n < MAX_N; n++) { f_no5[n] = 1; v5[n] = 0; }

    for (ll n = 1; n < MAX_N; n++) {
        ll tmp = n;
        while (tmp % 2 == 0) tmp /= 2;
        int v = 0;
        while (tmp % 5 == 0) { tmp /= 5; v++; }
        v5[n] = v;
        cof[n] = (int)tmp;
    }

    for (int pi = 0; pi < nprimes; pi++) {
        int p = primes[pi];
        if (p == 2 || p == 5) continue;
        int mod4 = p & 3;
        for (ll n = p; n < MAX_N; n += p) {
            if (f_no5[n] == 0) continue;
            if (cof[n] % p != 0) continue;
            int v = 0;
            while (cof[n] % p == 0) { cof[n] /= p; v++; }
            if (mod4 == 1) f_no5[n] *= (v + 1);
            else if (v & 1) f_no5[n] = 0;
        }
    }

    for (ll n = 1; n < MAX_N; n++) {
        if (f_no5[n] == 0) continue;
        if (cof[n] > 1) {
            if ((cof[n] & 3) == 1) f_no5[n] *= 2;
            else f_no5[n] = 0;
        }
    }
}

int main(void) {
    build_primes();
    compute_sieve();

    ll r = R5;
    i128 partial = 0;
    for (ll x = 1; x < r; x++) {
        ll a = r - x, b = r + x;
        if (f_no5[a] == 0 || f_no5[b] == 0) continue;
        int cv = v5[a] + v5[b];
        ll f_combined = (ll)f_no5[a] * f_no5[b] * (cv + 1);
        partial += (i128)x * f_combined;
    }
    i128 S_r5 = 6 * (i128)r + 24 * partial;
    i128 S_final = 1024 * S_r5;
    printf("%lld\n", (ll)S_final);
    return 0;
}
