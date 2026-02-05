"""Project Euler Problem 639: Summing a multiplicative function."""

import subprocess
import tempfile
import os

def solve():
    c_code = r'''
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <math.h>

#define N 1000000000000LL
#define K 50
#define M 1000000007LL

int64_t isqrt(int64_t n) {
    int64_t x = (int64_t)sqrt((double)n);
    while (x * x > n) x--;
    while ((x+1)*(x+1) <= n) x++;
    return x;
}

int64_t icbrt(int64_t n) {
    int64_t x = (int64_t)cbrt((double)n);
    while (x > 0 && x * x * x > n) x--;
    while ((x+1)*(x+1)*(x+1) <= n) x++;
    return x;
}

int L;
char *is_prime;
int *primes;
int num_primes;
int64_t *nth_pows;
int64_t *sum_powers;
int64_t *sum_coeffs;
int64_t ans = 0;

int64_t mod(int64_t x) {
    x %= M;
    if (x < 0) x += M;
    return x;
}

int64_t pow_mod(int64_t base, int64_t exp) {
    int64_t result = 1;
    base = mod(base);
    while (exp > 0) {
        if (exp & 1) result = mod(result * base);
        base = mod(base * base);
        exp >>= 1;
    }
    return result;
}

// Sum i^k for i=1 to n using Faulhaber / power sum formula
// For large n, we use the formula for sum of k-th powers
// Actually, for moderate k (<=50) and large n, we compute iteratively
int64_t sum_powers_func(int64_t n, int k) {
    if (n <= L) return sum_powers[n];
    // For very large n, we need Faulhaber's formula
    // For now, use a loop (this shouldn't happen often)
    int64_t result = 0;
    for (int64_t i = 1; i <= n; i++) {
        result = mod(result + pow_mod(i, k));
    }
    return result;
}

void helper(int min_index, int64_t d, int64_t mult, int prev_e, int k) {
    int64_t n = N / d;

    if (prev_e != 2) {
        int64_t sp = (n <= L) ? sum_powers[n] : sum_powers_func(n, k);
        ans = mod(ans + sp * mult);
    }

    int64_t lim = icbrt(n);

    // Sum over primes p where p^2 <= n/lim
    for (int i = min_index; i < num_primes; i++) {
        int64_t p = primes[i];
        if (p * p > n / lim) break;
        int64_t q = n / (p * p);
        int64_t sp_q = (q <= L) ? sum_powers[q] : sum_powers_func(q, k);
        int64_t coeff = mod(nth_pows[p] * mod(1 - nth_pows[p]));
        ans = mod(ans + sp_q * mult % M * coeff);
    }

    // Sum over ranges of primes
    for (int64_t q = 1; q < lim; q++) {
        int64_t high = isqrt(n / q);
        int64_t low_sq = isqrt(n / (q + 1));
        int64_t low = (min_index > 0 && primes[min_index] > 1) ? primes[min_index] - 1 : 1;
        if (low_sq > low) low = low_sq;
        if (high > L) high = L;
        if (high >= low && high >= 1 && low >= 0) {
            int64_t coeff_sum = mod(sum_coeffs[high] - (low > 0 ? sum_coeffs[low] : 0));
            ans = mod(ans + sum_powers[q] * mult % M * coeff_sum);
        } else {
            break;
        }
    }

    // Recurse with higher prime powers
    for (int index = min_index; index < num_primes; index++) {
        int64_t p = primes[index];
        if (d * p * p * p > N) break;
        int64_t new_d = d * p;
        for (int e = 2; (double)new_d * p <= (double)N; e++) {
            new_d *= p;
            int64_t new_mult = mod(mult * nth_pows[p] % M * mod(1 - nth_pows[p]));
            helper(index + 1, new_d, new_mult, e, k);
        }
    }
}

int main() {
    L = (int)isqrt(N);

    // Sieve primes
    is_prime = (char*)malloc((L + 1) * sizeof(char));
    for (int i = 0; i <= L; i++) is_prime[i] = 1;
    is_prime[0] = is_prime[1] = 0;
    for (int64_t i = 2; i * i <= L; i++) {
        if (is_prime[i]) {
            for (int64_t j = i * i; j <= L; j += i) is_prime[j] = 0;
        }
    }
    num_primes = 0;
    for (int i = 2; i <= L; i++) if (is_prime[i]) num_primes++;
    primes = (int*)malloc(num_primes * sizeof(int));
    int idx = 0;
    for (int i = 2; i <= L; i++) if (is_prime[i]) primes[idx++] = i;

    // Allocate arrays
    nth_pows = (int64_t*)malloc((L + 1) * sizeof(int64_t));
    sum_powers = (int64_t*)malloc((L + 1) * sizeof(int64_t));
    sum_coeffs = (int64_t*)malloc((L + 1) * sizeof(int64_t));

    for (int i = 0; i <= L; i++) nth_pows[i] = 1;

    for (int k = 1; k <= K; k++) {
        sum_powers[0] = 0;
        sum_coeffs[0] = 0;
        for (int i = 1; i <= L; i++) {
            nth_pows[i] = mod(nth_pows[i] * i);
            sum_powers[i] = mod(sum_powers[i - 1] + nth_pows[i]);
            int64_t coeff = is_prime[i] ? mod(nth_pows[i] * mod(1 - nth_pows[i])) : 0;
            sum_coeffs[i] = mod(sum_coeffs[i - 1] + coeff);
        }
        helper(0, 1, 1, 0, k);
    }

    ans = mod(ans);

    free(is_prime);
    free(primes);
    free(nth_pows);
    free(sum_powers);
    free(sum_coeffs);

    printf("%lld\n", ans);
    return 0;
}
'''
    with tempfile.NamedTemporaryFile(suffix='.c', delete=False) as f:
        f.write(c_code.encode())
        c_file = f.name
    exe = c_file[:-2]
    subprocess.run(['gcc', '-O3', '-o', exe, c_file, '-lm'], check=True, capture_output=True)
    result = subprocess.check_output([exe]).decode().strip()
    os.unlink(c_file)
    os.unlink(exe)
    return int(result)

if __name__ == "__main__":
    print(solve())
