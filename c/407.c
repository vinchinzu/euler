#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/*
 * For each n, M(n) = max a in [0,n) with a^2 = a (mod n), i.e. n | a(a-1).
 * By CRT, if n = p1^e1 * ... * pk^ek, then a = 0 or 1 mod each pi^ei.
 * We enumerate all coprime factorizations and compute CRT idempotents.
 */

#define MAXN 10000001

int M[MAXN];

long long gcd_func(long long a, long long b) {
    while (b) { long long t = b; b = a % b; a = t; }
    return a;
}

/* Extended GCD: returns gcd, sets *x, *y such that a*x + b*y = gcd */
long long extgcd(long long a, long long b, long long *x, long long *y) {
    if (b == 0) { *x = 1; *y = 0; return a; }
    long long x1, y1;
    long long g = extgcd(b, a % b, &x1, &y1);
    *x = y1;
    *y = x1 - (a / b) * y1;
    return g;
}

/* modular inverse of a mod m, returns -1 if not coprime */
long long modinv(long long a, long long m) {
    long long x, y;
    long long g = extgcd(a, m, &x, &y);
    if (g != 1) return -1;
    return ((x % m) + m) % m;
}

int main() {
    int N = 10000000;

    /* Initialize M[n] = 1 for all n >= 2 (a=1 is always idempotent) */
    for (int n = 0; n <= N; n++) M[n] = 1;
    M[0] = 0;
    M[1] = 0;  /* M(1) = 0 since only a=0 works for n=1 */

    /* For each d >= 2, iterate over multiples n = d*k where gcd(d,k)=1 */
    for (int d = 2; d <= N; d++) {
        for (long long n = (long long)d * 2; n <= N; n += d) {
            int k = (int)(n / d);
            if (gcd_func(d, k) != 1) continue;

            /* CRT(1 mod d, 0 mod k): a = k * modinv(k, d) */
            long long inv = modinv(k, d);
            if (inv < 0) continue;
            long long a = ((long long)k * inv) % n;
            if (a > M[n]) M[n] = (int)a;
        }
    }

    long long sum = 0;
    for (int n = 1; n <= N; n++) {
        sum += M[n];
    }
    printf("%lld\n", sum);
    return 0;
}
