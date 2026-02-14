/* Project Euler 200: Find the 200th prime-proof sqube containing "200". */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <stdbool.h>

static const int SMALL_PRIMES[] = {2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31};
#define N_SMALL_PRIMES 11

static long long mul_mod(long long a, long long b, long long mod) {
    return (unsigned __int128)a * b % mod;
}

static long long pow_mod(long long base, long long exp, long long mod) {
    long long result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) result = mul_mod(result, base, mod);
        base = mul_mod(base, base, mod);
        exp >>= 1;
    }
    return result;
}

static bool miller_rabin_witness(long long a, long long n, long long d, int r) {
    long long x = pow_mod(a, d, n);
    if (x == 1 || x == n - 1) return false;
    for (int i = 0; i < r - 1; i++) {
        x = mul_mod(x, x, n);
        if (x == n - 1) return false;
    }
    return true;
}

static bool is_prime(long long n) {
    if (n < 2) return false;
    for (int i = 0; i < N_SMALL_PRIMES; i++) {
        if (n == SMALL_PRIMES[i]) return true;
        if (n % SMALL_PRIMES[i] == 0) return false;
    }
    long long d = n - 1;
    int r = 0;
    while ((d & 1) == 0) { d >>= 1; r++; }
    long long bases[] = {2, 3, 5, 7, 11, 13, 17};
    for (int i = 0; i < 7; i++) {
        if (bases[i] % n == 0) continue;
        if (miller_rabin_witness(bases[i], n, d, r)) return false;
    }
    return true;
}

static bool contains_200(long long n) {
    char buf[32];
    sprintf(buf, "%lld", n);
    return strstr(buf, "200") != NULL;
}

static bool prime_proof(long long n) {
    char s[32];
    int len = sprintf(s, "%lld", n);
    for (int i = 0; i < len; i++) {
        char orig = s[i];
        for (char ch = '0'; ch <= '9'; ch++) {
            if (ch == orig) continue;
            if (i == 0 && ch == '0') continue;
            if (i == len - 1 && ch == '0') continue;
            if (i == len - 1 && ch == '5' && len > 1) continue;
            s[i] = ch;
            long long m = atoll(s);
            if (m >= 2 && is_prime(m)) {
                s[i] = orig;
                return false;
            }
        }
        s[i] = orig;
    }
    return true;
}

/* Sieve primes up to limit */
static int *sieve_primes(int limit, int *count) {
    char *sieve = calloc(limit + 1, 1);
    sieve[0] = sieve[1] = 1;
    for (int i = 2; (long long)i * i <= limit; i++) {
        if (!sieve[i]) {
            for (int j = i * i; j <= limit; j += i)
                sieve[j] = 1;
        }
    }
    int n = 0;
    for (int i = 2; i <= limit; i++) if (!sieve[i]) n++;
    int *primes = malloc(sizeof(int) * n);
    int idx = 0;
    for (int i = 2; i <= limit; i++) if (!sieve[i]) primes[idx++] = i;
    free(sieve);
    *count = n;
    return primes;
}

static int cmp_ll(const void *a, const void *b) {
    long long va = *(const long long *)a, vb = *(const long long *)b;
    if (va < vb) return -1;
    if (va > vb) return 1;
    return 0;
}

int main(void) {
    int target_index = 200;
    long long limit = 300000000000LL; /* 3*10^11 */

    int nprimes;
    int bound = 600000; /* sqrt(3e11) ~ 548k, cbrt ~ 6.7k */
    int *primes = sieve_primes(bound, &nprimes);

    /* Enumerate squbes p^2 * q^3 up to limit */
    long long *squbes = malloc(sizeof(long long) * 10000000);
    int nsqubes = 0;

    for (int i = 0; i < nprimes; i++) {
        long long p = primes[i];
        long long p2 = p * p;
        if (p2 > limit) break;
        /* q^3 <= limit / p^2 */
        long long q_max_cube = limit / p2;
        /* cube root */
        long long q_max = (long long)cbrt((double)q_max_cube);
        while ((q_max + 1) * (q_max + 1) * (q_max + 1) <= q_max_cube) q_max++;
        while (q_max * q_max * q_max > q_max_cube) q_max--;
        if (q_max < 2) continue;

        for (int j = 0; j < nprimes; j++) {
            long long q = primes[j];
            if (q > q_max) break;
            if (q == p) continue;
            long long v = p2 * q * q * q;
            if (v > limit) break;
            if (contains_200(v)) {
                squbes[nsqubes++] = v;
            }
        }
    }

    /* Sort and deduplicate */
    qsort(squbes, nsqubes, sizeof(long long), cmp_ll);
    int unique = 0;
    for (int i = 0; i < nsqubes; i++) {
        if (i == 0 || squbes[i] != squbes[i - 1]) {
            squbes[unique++] = squbes[i];
        }
    }
    nsqubes = unique;

    /* Filter for prime-proof */
    int count = 0;
    for (int i = 0; i < nsqubes; i++) {
        if (prime_proof(squbes[i])) {
            count++;
            if (count == target_index) {
                printf("%lld\n", squbes[i]);
                free(squbes);
                free(primes);
                return 0;
            }
        }
    }

    /* If not enough, increase limit - shouldn't happen for this problem */
    printf("Not found within limit\n");
    free(squbes);
    free(primes);
    return 1;
}
