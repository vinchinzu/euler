"""Project Euler Problem 540: Counting primitive Pythagorean triples."""

import subprocess
import tempfile
import os

def solve():
    c_code = r'''
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <math.h>

#define N 3141592653589793LL

int64_t isqrt(int64_t n) {
    if (n <= 0) return 0;
    int64_t x = (int64_t)sqrt((double)n);
    while (x > 0 && x * x > n) x--;
    while ((x+1) * (x+1) <= n) x++;
    return x;
}

int64_t sq(int64_t x) { return x * x; }

int *phi;
int *ff;  // smallest prime factor

void preff(int limit) {
    ff = (int*)calloc(limit + 1, sizeof(int));
    for (int i = 2; i <= limit; i++) {
        if (ff[i] == 0) {  // i is prime
            for (int j = i; j <= limit; j += i) {
                if (ff[j] == 0) ff[j] = i;
            }
        }
    }
}

void prePhi(int limit) {
    phi = (int*)malloc((limit + 1) * sizeof(int));
    for (int i = 0; i <= limit; i++) phi[i] = i;
    for (int i = 2; i <= limit; i++) {
        if (phi[i] == i) {  // i is prime
            for (int j = i; j <= limit; j += i) {
                phi[j] -= phi[j] / i;
            }
        }
    }
}

// Get prime factors of m (using ff array)
int get_prime_factors(int m, int *factors) {
    int num = 0;
    while (m > 1) {
        int p = ff[m];
        if (num == 0 || factors[num - 1] != p) {
            factors[num++] = p;
        }
        m /= p;
    }
    return num;
}

// Count numbers <= limit that are relatively prime to m using inclusion-exclusion
int64_t numRelativelyPrime(int m, int64_t limit) {
    if (limit <= 0) return 0;

    int factors[20];
    int num_factors = get_prime_factors(m, factors);

    // Inclusion-exclusion over subsets of prime factors
    int64_t count = 0;
    for (int mask = 0; mask < (1 << num_factors); mask++) {
        int64_t prod = 1;
        int bits = 0;
        for (int i = 0; i < num_factors; i++) {
            if (mask & (1 << i)) {
                prod *= factors[i];
                bits++;
            }
        }
        if (bits % 2 == 0) {
            count += limit / prod;
        } else {
            count -= limit / prod;
        }
    }
    return count;
}

int main() {
    int64_t L = isqrt(N / 2);
    int sqrt_N = (int)isqrt(N);

    preff(sqrt_N);
    prePhi((int)L);

    int64_t ans = 0;

    for (int64_t m = 2; sq(m) <= N; m++) {
        int mult = (m % 2 == 0) ? 1 : 2;
        if (m <= L) {
            ans += phi[m] / mult;
        } else {
            int64_t limit = isqrt(N - sq(m)) / mult;
            ans += numRelativelyPrime((int)m, limit);
        }
    }

    free(phi);
    free(ff);

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
