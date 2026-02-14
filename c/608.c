/*
 * Project Euler Problem 608: Divisor Sums
 *
 * Embedded C extracted from Python wrapper.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <math.h>

typedef int64_t i64;
typedef __int128 i128;

#define N 1000000000000LL
#define K 200
#define M 1000000007LL

i64 isqrt(i64 n) {
    i64 x = (i64)sqrtl((long double)n);
    while (x * x > n) x--;
    while ((x+1) * (x+1) <= n) x++;
    return x;
}

i64 sum_floor_quotients(i64 n) {
    if (n <= 0) return 0;
    i64 sqrtn = isqrt(n);
    i64 result = 0;

    for (i64 d = 1; d <= sqrtn; d++) {
        result += n / d;
    }

    result = 2 * result - sqrtn * sqrtn;

    return result;
}

int primes[100];
int prime_count = 0;

void sieve_primes() {
    char is_prime[K + 1];
    memset(is_prime, 1, sizeof(is_prime));
    is_prime[0] = is_prime[1] = 0;
    for (int i = 2; i * i <= K; i++) {
        if (is_prime[i]) {
            for (int j = i * i; j <= K; j += i)
                is_prime[j] = 0;
        }
    }
    for (int i = 2; i <= K; i++) {
        if (is_prime[i])
            primes[prime_count++] = i;
    }
}

i64 num_factors_in_factorial(int n, int p) {
    i64 count = 0;
    i64 power = p;
    while (power <= n) {
        count += n / power;
        power *= p;
    }
    return count;
}

i64 tr(i64 n) {
    return n * (n + 1) / 2;
}

i64 mod_inv(i64 a, i64 m) {
    i64 t = 0, new_t = 1;
    i64 r = m, new_r = a % m;
    while (new_r != 0) {
        i64 q = r / new_r;
        i64 tmp = new_t;
        new_t = t - q * new_t;
        t = tmp;
        tmp = new_r;
        new_r = r - q * new_r;
        r = tmp;
    }
    if (t < 0) t += m;
    return t;
}

int L;
i64 *sum_floor_small;
i64 *product_updates;
i64 ans = 0;

void helper(int min_index, i64 d, i64 mult) {
    i64 q = N / d;
    i64 sum_val;
    if (q >= L) {
        sum_val = sum_floor_quotients(q) % M;
    } else {
        sum_val = sum_floor_small[q];
    }
    ans = (ans + (i128)sum_val * mult % M + M) % M;

    for (int index = min_index; index < prime_count; index++) {
        int p = primes[index];
        if ((double)d * p > N) break;
        i64 new_mult = (i128)mult * product_updates[p] % M;
        if (new_mult < 0) new_mult += M;
        helper(index + 1, d * p, new_mult);
    }
}

int main() {
    sieve_primes();

    L = (int)pow((double)N, 2.0 / 3.0);

    int *num_divs = (int*)calloc(L + 1, sizeof(int));
    for (int i = 1; i <= L; i++) {
        for (int j = i; j <= L; j += i) {
            num_divs[j]++;
        }
    }

    sum_floor_small = (i64*)calloc(L + 1, sizeof(i64));
    for (int i = 1; i <= L; i++) {
        sum_floor_small[i] = (sum_floor_small[i - 1] + num_divs[i]) % M;
    }
    free(num_divs);

    product_updates = (i64*)calloc(K + 1, sizeof(i64));
    i64 mult = 1;
    for (int i = 0; i < prime_count; i++) {
        int p = primes[i];
        i64 e = num_factors_in_factorial(K, p);
        i64 tr_e_plus_1 = tr(e + 1) % M;
        mult = (i128)mult * tr_e_plus_1 % M;
        i64 tr_e = tr(e) % M;
        product_updates[p] = (-(i128)tr_e * mod_inv(tr_e_plus_1, M) % M + M) % M;
    }

    helper(0, 1, mult);

    printf("%lld\n", (ans % M + M) % M);

    free(sum_floor_small);
    free(product_updates);

    return 0;
}
