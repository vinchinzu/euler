"""Project Euler Problem 586: Numbers expressible as a^2+3ab+b^2."""

import subprocess
import tempfile
import os

def solve():
    c_code = r'''
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <math.h>

#define N 1000000000000000LL
#define K 40

int *ff;  // smallest prime factor
int min_power[2 * K + 3];
int *primes;
int num_primes;
int64_t ans = 0;

void preff(int limit) {
    ff = (int*)calloc(limit + 1, sizeof(int));
    for (int i = 2; i <= limit; i++) {
        if (ff[i] == 0) {
            ff[i] = i;
            for (int64_t j = (int64_t)i * i; j <= limit; j += i) {
                if (ff[j] == 0) ff[j] = i;
            }
        }
    }
}

void sieve_primes(int limit) {
    char *is_prime = (char*)malloc((limit + 1) * sizeof(char));
    for (int i = 0; i <= limit; i++) is_prime[i] = 1;
    is_prime[0] = is_prime[1] = 0;
    for (int64_t i = 2; i * i <= limit; i++) {
        if (is_prime[i]) {
            for (int64_t j = i * i; j <= limit; j += i) {
                is_prime[j] = 0;
            }
        }
    }
    num_primes = 0;
    for (int i = 2; i <= limit; i++) if (is_prime[i]) num_primes++;
    primes = (int*)malloc(num_primes * sizeof(int));
    int idx = 0;
    for (int i = 2; i <= limit; i++) if (is_prime[i]) primes[idx++] = i;
    free(is_prime);
}

// Returns p^exp using __int128 to avoid overflow
__int128 power128(int64_t base, int exp) {
    __int128 result = 1;
    for (int i = 0; i < exp; i++) {
        result *= base;
        if (result > N) return result;
    }
    return result;
}

// Check if prod * p^exp > N
int exceeds(__int128 prod, int p, int exp) {
    for (int i = 0; i < exp; i++) {
        prod *= p;
        if (prod > N) return 1;
    }
    return 0;
}

void helper(int k, int last_index, __int128 prod) {
    if (k == 1)
        ans++;

    for (int index = last_index + 1; index < num_primes; index++) {
        int p = primes[index];
        if (exceeds(prod, p, min_power[k])) break;

        __int128 new_prod = prod;
        int e = 1;
        while (new_prod * p <= N) {
            new_prod *= p;
            if (p % 5 == 1 || p % 5 == 4) {
                if (k % (e + 1) == 0) {
                    helper(k / (e + 1), index, new_prod);
                }
            } else if (p == 5 || e % 2 == 0) {
                helper(k, index, new_prod);
            }
            e++;
        }
    }
}

int main() {
    // Compute min_power
    preff(2 * K + 2);
    for (int k = 0; k < 2 * K + 2; k++) {
        int n = k;
        min_power[k] = 0;
        while (n > 1) {
            int p = ff[n];
            int e = 0;
            while (n % p == 0) {
                n /= p;
                e++;
            }
            min_power[k] += e * (p - 1);
        }
    }
    min_power[1] = 2;

    // Calculate max prime
    int min_mp = min_power[2 * K] < min_power[2 * K + 1] ? min_power[2 * K] : min_power[2 * K + 1];
    int64_t max_prime = N / power128(11, min_mp - 1);
    sieve_primes((int)max_prime);

    free(ff);

    helper(2 * K, -1, 1);
    helper(2 * K + 1, -1, 1);

    free(primes);

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
