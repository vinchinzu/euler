"""Project Euler Problem 550: Divisor game."""

import subprocess
import tempfile
import os

def solve():
    c_code = r'''
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

typedef int64_t i64;
typedef __int128 i128;

#define N 10000000
#define K 1000000000000LL
#define L 64
#define M 987654321LL

// Primes
int *primes;
int prime_count;
int *num_primes;

void sieve() {
    char *is_prime = (char*)calloc(N + 1, sizeof(char));
    for (int i = 2; i <= N; i++) is_prime[i] = 1;
    for (i64 i = 2; i * i <= N; i++) {
        if (is_prime[i]) {
            for (i64 j = i * i; j <= N; j += i)
                is_prime[j] = 0;
        }
    }
    primes = (int*)malloc((N + 1) * sizeof(int));
    prime_count = 0;
    for (int i = 2; i <= N; i++)
        if (is_prime[i]) primes[prime_count++] = i;

    num_primes = (int*)calloc(N + 1, sizeof(int));
    int cnt = 0;
    for (int i = 0; i <= N; i++) {
        if (i >= 2 && is_prime[i]) cnt++;
        num_primes[i] = cnt;
    }

    free(is_prime);
}

int divisors[1000];
int divisor_count;

void get_divisors(int n) {
    divisor_count = 0;
    for (i64 i = 1; i * i <= n; i++) {
        if (n % i == 0) {
            divisors[divisor_count++] = (int)i;
            if (i != n / i) divisors[divisor_count++] = n / (int)i;
        }
    }
    for (int i = 0; i < divisor_count; i++) {
        for (int j = i + 1; j < divisor_count; j++) {
            if (divisors[i] > divisors[j]) {
                int t = divisors[i]; divisors[i] = divisors[j]; divisors[j] = t;
            }
        }
    }
}

int *nimbers;
i64 counts[L];

void helper(int min_idx, int n) {
    if (n > 1) {
        int used[L] = {0};
        get_divisors(n);
        for (int i = 1; i < divisor_count - 1; i++) {
            for (int j = i; j < divisor_count - 1; j++) {
                int xor_val = nimbers[divisors[i]] ^ nimbers[divisors[j]];
                if (xor_val < L) used[xor_val] = 1;
            }
        }
        while (nimbers[n] < L && used[nimbers[n]])
            nimbers[n]++;
        counts[nimbers[n]]++;
    }

    for (int idx = min_idx; idx < prime_count; idx++) {
        int p = primes[idx];
        if ((i64)n * p > N) break;

        for (int new_n = n; (i64)new_n * p <= N; new_n *= p)
            helper(idx + 1, new_n * p);

        if ((i64)n * (i64)p * p > N) {
            if (idx > 0) {
                int prev_p = primes[idx - 1];
                int add = num_primes[N / n] - num_primes[prev_p] - 1;
                if (add > 0) counts[nimbers[n * p]] += add;
            }
            return;
        }
    }
}

// Extended GCD for modular inverse
i64 extended_gcd(i64 a, i64 b, i64 *x, i64 *y) {
    if (a == 0) {
        *x = 0;
        *y = 1;
        return b;
    }
    i64 x1, y1;
    i64 gcd = extended_gcd(b % a, a, &x1, &y1);
    *x = y1 - (b / a) * x1;
    *y = x1;
    return gcd;
}

i64 mod_inverse(i64 a, i64 m) {
    i64 x, y;
    extended_gcd(a % m, m, &x, &y);
    return ((x % m) + m) % m;
}

// Fast Walsh-Hadamard Transform for XOR convolution
void fwht(i64 *a, int n, int inv) {
    for (int len = 1; len < n; len <<= 1) {
        for (int i = 0; i < n; i += len << 1) {
            for (int j = 0; j < len; j++) {
                i64 u = a[i + j];
                i64 v = a[i + j + len];
                a[i + j] = (u + v) % M;
                a[i + j + len] = ((u - v) % M + M) % M;
            }
        }
    }
    if (inv) {
        i64 inv_n = mod_inverse(n, M);
        for (int i = 0; i < n; i++)
            a[i] = (i128)a[i] * inv_n % M;
    }
}

i64 power(i64 base, i64 exp) {
    i64 result = 1;
    base = ((base % M) + M) % M;
    while (exp > 0) {
        if (exp & 1) result = (i128)result * base % M;
        base = (i128)base * base % M;
        exp >>= 1;
    }
    return result;
}

int main() {
    sieve();

    nimbers = (int*)calloc(N + 1, sizeof(int));

    helper(0, 1);

    i64 arr[L];
    for (int i = 0; i < L; i++) arr[i] = counts[i] % M;

    fwht(arr, L, 0);
    for (int i = 0; i < L; i++)
        arr[i] = power(arr[i], K);
    fwht(arr, L, 1);

    i64 ans = 0;
    for (int i = 1; i < L; i++)
        ans = (ans + arr[i]) % M;

    printf("%lld\n", ans);

    free(primes);
    free(num_primes);
    free(nimbers);

    return 0;
}
'''

    with tempfile.NamedTemporaryFile(suffix='.c', delete=False) as f:
        f.write(c_code.encode())
        c_file = f.name

    exe = c_file[:-2]
    subprocess.run(['gcc', '-O3', '-o', exe, c_file], check=True, capture_output=True)
    result = subprocess.check_output([exe]).decode().strip()

    os.unlink(c_file)
    os.unlink(exe)

    return int(result)

if __name__ == "__main__":
    print(solve())
