/*
 * Project Euler Problem 245: Coresilience
 *
 * Find the sum of composite integers n <= N such that (n - phi(n)) / (n - 1)
 * is a unit fraction.
 *
 * Extracted from embedded C in Python solution.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define N 200000000000LL
#define L 447213
#define LIMIT 34199519

static char is_prime_sieve[LIMIT + 1];
static char is_prime_small[L + 1];

/* Lists of prime factors for each index up to L */
static int pf_data[L + 1][20];  /* max 20 prime factors per entry */
static int pf_count[L + 1];

static int primes[40000];
static int nprimes;

void build_sieves(void) {
    memset(is_prime_sieve, 1, sizeof(is_prime_sieve));
    is_prime_sieve[0] = is_prime_sieve[1] = 0;
    for (long i = 2; i * i <= LIMIT; i++)
        if (is_prime_sieve[i])
            for (long j = i * i; j <= LIMIT; j += i)
                is_prime_sieve[j] = 0;

    memset(is_prime_small, 1, sizeof(is_prime_small));
    is_prime_small[0] = is_prime_small[1] = 0;
    for (int i = 2; (long)i * i <= L; i++)
        if (is_prime_small[i])
            for (int j = i * i; j <= L; j += i)
                is_prime_small[j] = 0;

    nprimes = 0;
    for (int i = 3; i <= L; i++)
        if (is_prime_small[i])
            primes[nprimes++] = i;
}

int check_prime(long n) {
    if (n < 2) return 0;
    if (n <= L) return is_prime_small[n];
    if (n <= LIMIT) return is_prime_sieve[n];
    /* Miller-Rabin */
    if (n % 2 == 0) return 0;
    long d = n - 1;
    int r = 0;
    while (d % 2 == 0) { d /= 2; r++; }
    int witnesses[] = {2, 3, 5, 7, 11, 13};
    for (int w = 0; w < 6; w++) {
        long a = witnesses[w];
        if (a >= n) continue;
        /* Compute a^d mod n using __int128 */
        __int128 x = 1, base = a;
        long dd = d;
        while (dd > 0) {
            if (dd & 1) x = x * base % n;
            base = base * base % n;
            dd >>= 1;
        }
        long xl = (long)x;
        if (xl == 1 || xl == n - 1) continue;
        int cont = 0;
        for (int i = 0; i < r - 1; i++) {
            x = (__int128)xl * xl % n;
            xl = (long)x;
            if (xl == n - 1) { cont = 1; break; }
        }
        if (!cont) return 0;
    }
    return 1;
}

long power_mod(long base, long exp, long mod) {
    __int128 result = 1, b = base % mod;
    while (exp > 0) {
        if (exp & 1) result = result * b % mod;
        b = b * b % mod;
        exp >>= 1;
    }
    return (long)result;
}

int is_sq(long n, long p) {
    return power_mod(n, (p - 1) / 2, p) <= 1;
}

long sqrt_mod(long n, long p) {
    n = n % p;
    if (n < 0) n += p;
    if (n == 0) return 0;
    if (p % 4 == 3) return power_mod(n, (p + 1) / 4, p);
    /* Tonelli-Shanks */
    long q = p - 1;
    int s = 0;
    while (q % 2 == 0) { q /= 2; s++; }
    long z = 2;
    while (power_mod(z, (p - 1) / 2, p) != p - 1) z++;
    int m = s;
    long c = power_mod(z, q, p);
    long t = power_mod(n, q, p);
    long r = power_mod(n, (q + 1) / 2, p);
    while (1) {
        if (t == 1) return r;
        int i = 1;
        __int128 tmp = (__int128)t * t % p;
        while ((long)tmp != 1) { tmp = tmp * (long)tmp % p; i++; }
        long b = c;
        for (int j = 0; j < m - i - 1; j++) b = (__int128)b * b % p;
        m = i;
        c = (__int128)b * b % p;
        t = (__int128)t * c % p;
        r = (__int128)r * b % p;
    }
}

/* Get all divisors of n using precomputed prime factors */
static long divisors[10000];
int all_divisors(long n, int idx) {
    divisors[0] = 1;
    int ndivs = 1;
    long temp = n;
    for (int pi = 0; pi < pf_count[idx]; pi++) {
        int p = pf_data[idx][pi];
        if (temp % p == 0) {
            int size = ndivs;
            long power = 1;
            while (temp % p == 0) {
                temp /= p;
                power *= p;
                for (int i = 0; i < size; i++)
                    divisors[ndivs++] = divisors[i] * power;
            }
        }
    }
    if (temp > 1) {
        int size = ndivs;
        for (int i = 0; i < size; i++)
            divisors[ndivs++] = divisors[i] * temp;
    }
    return ndivs;
}

static long ans = 0;

void helper(int index, long P, long phi, int *factors, int nfactors) {
    if (nfactors >= 2) {
        int smallest = factors[0];
        for (int k = 2; k < smallest; k += 2) {
            long num = phi * k + 1;
            long den = P - (P - phi) * k;
            if (den > 0 && num % den == 0) {
                long q = num / den;
                if (factors[nfactors-1] < q && P * q <= N && check_prime(q))
                    ans += P * q;
            }
        }
    }
    int idx = index;
    while (idx < nprimes) {
        long q = primes[idx];
        if (P * q * q > N) break;
        factors[nfactors] = (int)q;
        helper(idx + 1, P * q, phi * (q - 1), factors, nfactors + 1);
        idx++;
    }
}

int main(void) {
    build_sieves();

    memset(pf_count, 0, sizeof(pf_count));
    for (int qi = 0; qi < nprimes; qi++) {
        int q = primes[qi];
        if (q >= 3 && is_sq(q - 3, q)) {
            long r1 = sqrt_mod(q - 3, q);
            long inv2 = (q + 1) / 2;
            long s1 = ((1 + r1) * inv2) % q;
            if (s1 == 0) s1 = q;
            for (long p = s1; p <= L; p += q)
                pf_data[p][pf_count[p]++] = q;
            long s2 = ((1 - r1 + q) * inv2) % q;
            if (s2 == 0) s2 = q;
            for (long p = s2; p <= L; p += q)
                pf_data[p][pf_count[p]++] = q;
        }
    }

    /* Two primes case */
    for (int i = 0; i < nprimes; i++) {
        long p = primes[i];
        long val = p * (p - 1) + 1;
        int nd = all_divisors(val, (int)p);
        for (int j = 0; j < nd; j++) {
            long d = divisors[j];
            if (d >= p) {
                long q = d - (p - 1);
                if (p < q && p * q <= N && check_prime(q))
                    ans += p * q;
            }
        }
    }

    /* More than two primes case */
    int factors[20];
    helper(0, 1, 1, factors, 0);

    printf("%ld\n", ans);
    return 0;
}
