"""Project Euler Problem 521: Smallest Prime Factor."""

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
#define M 1000000000LL

int64_t isqrt(int64_t n) {
    int64_t x = (int64_t)sqrt((double)n);
    while (x * x > n) x--;
    while ((x+1)*(x+1) <= n) x++;
    return x;
}

int64_t L;
int64_t *big_cnt, *small_cnt;
int64_t *big_sum, *small_sum;

int64_t cnt_div(int64_t x) {
    if (x == 0) return 0;
    int64_t q = N / x;
    if (q < N / L) return small_cnt[q];
    return big_cnt[N / q];
}

int64_t sum_div(int64_t x) {
    if (x == 0) return 0;
    int64_t q = N / x;
    if (q < N / L) return small_sum[q];
    return big_sum[N / q];
}

// Sum 2 + 3 + ... + n mod M
int64_t sum_2_to_n(int64_t n) {
    if (n < 2) return 0;
    // n*(n+1)/2 - 1 mod M
    // Use __int128 to avoid overflow
    __int128 s = (__int128)n * (n + 1) / 2 - 1;
    return (int64_t)(s % M);
}

int main() {
    L = isqrt(N);

    // Sieve for primes up to L
    char *is_prime = (char*)malloc((L + 1) * sizeof(char));
    for (int64_t i = 0; i <= L; i++) is_prime[i] = 1;
    is_prime[0] = is_prime[1] = 0;
    for (int64_t i = 2; i * i <= L; i++) {
        if (is_prime[i]) {
            for (int64_t j = i * i; j <= L; j += i)
                is_prime[j] = 0;
        }
    }

    // Collect primes
    int num_small_primes = 0;
    for (int64_t i = 2; i <= L; i++) if (is_prime[i]) num_small_primes++;
    int *primes = (int*)malloc(num_small_primes * sizeof(int));
    int idx = 0;
    for (int64_t i = 2; i <= L; i++) if (is_prime[i]) primes[idx++] = (int)i;

    // Initialize arrays
    big_cnt = (int64_t*)malloc((L + 2) * sizeof(int64_t));
    small_cnt = (int64_t*)malloc((N / L + 2) * sizeof(int64_t));
    big_sum = (int64_t*)malloc((L + 2) * sizeof(int64_t));
    small_sum = (int64_t*)malloc((N / L + 2) * sizeof(int64_t));

    // Initialize: count numbers > 1, sum numbers > 1
    for (int64_t i = 1; i <= L; i++) {
        big_cnt[i] = N / i - 1;  // count of 2..N/i
        big_sum[i] = sum_2_to_n(N / i);
    }
    for (int64_t i = 1; i < N / L; i++) {
        small_cnt[i] = i - 1;  // count of 2..i
        small_sum[i] = sum_2_to_n(i);
    }

    int64_t ans = 0;

    // Lucy_Hedgehog sieve
    // After processing prime p, big_cnt[i] = pi(N/i) using only primes >= p
    // Actually after full sieve, it becomes pi(N/i)

    for (int pi = 0; pi < num_small_primes; pi++) {
        int64_t p = primes[pi];
        int64_t p2 = p * p;

        // Before update: cnt_div(p) is count of numbers in [2, N/p] with all prime factors >= p
        // After removing multiples of p (composite with smallest factor p):
        // count removed = cnt_div(p) - pi(p-1) where pi(p-1) = small_cnt[p-1]
        // This is the count of composites with smallest prime factor p

        int64_t removed = cnt_div(p) - small_cnt[p - 1];
        ans = (ans + (p % M) * (removed % M)) % M;

        // Update arrays: remove numbers divisible by p (those counted as p * something)
        for (int64_t i = 1; i <= L && N / i >= p2; i++) {
            int64_t cnt_remove = cnt_div(i * p) - small_cnt[p - 1];
            big_cnt[i] -= cnt_remove;
            int64_t sum_remove = (sum_div(i * p) - small_sum[p - 1] + M) % M;
            big_sum[i] = (big_sum[i] - (p % M) * sum_remove % M + M) % M;
        }
        for (int64_t i = N / L - 1; i >= p2; i--) {
            int64_t cnt_remove = small_cnt[i / p] - small_cnt[p - 1];
            small_cnt[i] -= cnt_remove;
            int64_t sum_remove = (small_sum[i / p] - small_sum[p - 1] + M) % M;
            small_sum[i] = (small_sum[i] - (p % M) * sum_remove % M + M) % M;
        }
    }

    // Add sum of all primes <= N (which is now in sum_div(1))
    ans = (ans + sum_div(1)) % M;

    free(is_prime);
    free(primes);
    free(big_cnt);
    free(small_cnt);
    free(big_sum);
    free(small_sum);

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
