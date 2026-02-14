#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

/* Sieve Mobius function up to L using linear sieve */
static int *mobius_arr;
static char *is_prime;
static int *primes;
static int nprimes;

void sieve_mobius(int L) {
    mobius_arr = (int *)calloc(L + 1, sizeof(int));
    is_prime = (char *)malloc(L + 1);
    primes = (int *)malloc((L / 2 + 1) * sizeof(int));
    memset(is_prime, 1, L + 1);
    nprimes = 0;
    mobius_arr[1] = 1;

    for (int i = 2; i <= L; i++) {
        if (is_prime[i]) {
            primes[nprimes++] = i;
            mobius_arr[i] = -1;
        }
        for (int j = 0; j < nprimes; j++) {
            long long ip = (long long)i * primes[j];
            if (ip > L) break;
            is_prime[ip] = 0;
            if (i % primes[j] == 0) {
                mobius_arr[ip] = 0;
                break;
            }
            mobius_arr[ip] = -mobius_arr[i];
        }
    }
}

long long isqrt_ll(long long n) {
    if (n <= 0) return 0;
    long long x = (long long)sqrt((double)n);
    while (x * x > n) x--;
    while ((x + 1) * (x + 1) <= n) x++;
    return x;
}

/* D(n) = sum_{k=1}^{n} floor(n/k) in O(sqrt(n)) time */
long long D(long long n) {
    if (n <= 0) return 0;
    long long sq = isqrt_ll(n);
    long long s = 0;
    for (long long k = 1; k <= sq; k++) {
        s += n / k;
    }
    return 2 * s - sq * sq;
}

long long icbrt(long long n) {
    if (n <= 0) return 0;
    long long x = (long long)cbrt((double)n);
    while (x > 0 && x * x * x > n) x--;
    while ((x + 1) * (x + 1) * (x + 1) <= n) x++;
    return x;
}

/* T(m) = number of ordered triples (a,b,c) with a*b*c <= m */
long long T(long long m) {
    if (m <= 0) return 0;

    long long cbrt_m = icbrt(m);
    long long total = 0;

    for (long long a = 1; a <= cbrt_m; a++) {
        total += D(m / a);
    }

    long long a = cbrt_m + 1;
    while (a <= m) {
        long long v = m / a;
        long long a_max = m / v;
        total += D(v) * (a_max - a + 1);
        a = a_max + 1;
    }

    return total;
}

int main() {
    long long N = 1000000000000LL;
    long long L = isqrt_ll(N);

    sieve_mobius((int)L);

    long long ans = 0;
    for (long long d = 1; d <= L; d++) {
        if (mobius_arr[d] != 0) {
            ans += mobius_arr[d] * T(N / (d * d));
        }
    }
    ans += N;
    ans /= 2;

    printf("%lld\n", ans);

    free(mobius_arr);
    free(is_prime);
    free(primes);
    return 0;
}
