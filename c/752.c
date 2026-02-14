/* Project Euler Problem 752: Powers of 1+sqrt(7).
 * Extracted from embedded C in python/752.py
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAXN 1000001

static int spf[MAXN];  /* smallest prime factor */
static long long g_val[MAXN]; /* g(x) for each x */

void sieve(void) {
    for (int i = 0; i < MAXN; i++) spf[i] = i;
    for (int i = 2; (long long)i * i < MAXN; i++) {
        if (spf[i] == i) {
            for (int j = i * i; j < MAXN; j += i)
                if (spf[j] == j) spf[j] = i;
        }
    }
}

/* 2x2 matrix multiplication mod m */
typedef struct { long long a, b, c, d; } Mat;

Mat mat_mul(Mat A, Mat B, long long m) {
    Mat C;
    C.a = ((__int128)A.a * B.a + (__int128)A.b * B.c) % m;
    C.b = ((__int128)A.a * B.b + (__int128)A.b * B.d) % m;
    C.c = ((__int128)A.c * B.a + (__int128)A.d * B.c) % m;
    C.d = ((__int128)A.c * B.b + (__int128)A.d * B.d) % m;
    return C;
}

Mat mat_pow(Mat A, long long e, long long m) {
    Mat R = {1, 0, 0, 1}; /* identity */
    A.a %= m; A.b %= m; A.c %= m; A.d %= m;
    while (e > 0) {
        if (e & 1) R = mat_mul(R, A, m);
        A = mat_mul(A, A, m);
        e >>= 1;
    }
    return R;
}

int is_identity(Mat M, long long m) {
    return M.a % m == 1 && M.b % m == 0 && M.c % m == 0 && M.d % m == 1;
}

/* Find order of matrix A mod p, knowing it divides n = p^2-1 */
long long mat_order(long long p) {
    Mat A = {1, 7, 1, 1};
    long long n = (p - 1) * (p + 1);  /* p^2 - 1 */

    /* Factorize n */
    long long temp = n;
    long long primes[64];
    int exps[64], np = 0;
    for (long long d = 2; d * d <= temp; d++) {
        if (temp % d == 0) {
            primes[np] = d;
            exps[np] = 0;
            while (temp % d == 0) { exps[np]++; temp /= d; }
            np++;
        }
    }
    if (temp > 1) { primes[np] = temp; exps[np] = 1; np++; }

    long long order = n;
    for (int i = 0; i < np; i++) {
        while (order % primes[i] == 0) {
            long long trial = order / primes[i];
            Mat M = mat_pow(A, trial, p);
            if (is_identity(M, p))
                order = trial;
            else
                break;
        }
    }
    return order;
}

long long gcd(long long a, long long b) {
    while (b) { long long t = b; b = a % b; a = t; }
    return a;
}

long long lcm(long long a, long long b) {
    if (a == 0 || b == 0) return 0;
    return a / gcd(a, b) * b;
}

int main(void) {
    int N = 1000000;
    sieve();

    memset(g_val, 0, sizeof(g_val));

    for (int p = 2; p <= N; p++) {
        if (spf[p] != p) continue; /* not prime */
        if (p == 2 || p == 3) {
            g_val[p] = 0; /* gcd(p, 6) > 1 */
        } else if (p == 7) {
            g_val[p] = 7;
        } else {
            g_val[p] = mat_order((long long)p);
        }
    }

    long long ans = 0;
    for (int x = 2; x <= N; x++) {
        if (spf[x] == x) {
            /* Prime, already computed */
            ans += g_val[x];
            continue;
        }

        /* Factor x and compute g(x) = lcm of g(p^e) over prime powers */
        int temp = x;
        long long gx = 1;
        int is_zero = 0;
        while (temp > 1) {
            int p = spf[temp];
            int e = 0;
            while (temp % p == 0) { temp /= p; e++; }
            long long gpe = g_val[p];
            if (gpe == 0) { is_zero = 1; break; }
            for (int i = 1; i < e; i++) gpe *= p;
            gx = lcm(gx, gpe);
        }
        if (is_zero) gx = 0;
        g_val[x] = gx;
        ans += gx;
    }

    printf("%lld\n", ans);
    return 0;
}
