"""Project Euler Problem 800 - Hybrid Integers. Embedded C port for speed."""

import subprocess
import tempfile
import os

C_CODE = r"""
#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

int main(void) {
    int N = 800800;
    /* Upper bound for prime sieve: N * (log2(N) + 1) ~ 800800 * 21 ~ 16.8M */
    int max_prime = N * 21;

    /* Sieve of Eratosthenes */
    char *is_prime = (char *)malloc((size_t)(max_prime + 1));
    if (!is_prime) { fprintf(stderr, "malloc failed\n"); return 1; }
    memset(is_prime, 1, (size_t)(max_prime + 1));
    is_prime[0] = is_prime[1] = 0;

    for (int i = 2; (long long)i * i <= max_prime; i++) {
        if (is_prime[i]) {
            for (int j = i * i; j <= max_prime; j += i) {
                is_prime[j] = 0;
            }
        }
    }

    /* Count primes */
    int num_primes = 0;
    for (int i = 2; i <= max_prime; i++) {
        if (is_prime[i]) num_primes++;
    }

    /* Collect primes */
    int *primes = (int *)malloc((size_t)num_primes * sizeof(int));
    if (!primes) { fprintf(stderr, "malloc failed\n"); return 1; }
    int idx = 0;
    for (int i = 2; i <= max_prime; i++) {
        if (is_prime[i]) primes[idx++] = i;
    }
    free(is_prime);

    /* Precompute log values */
    double *log_primes = (double *)malloc((size_t)num_primes * sizeof(double));
    if (!log_primes) { fprintf(stderr, "malloc failed\n"); return 1; }
    for (int i = 0; i < num_primes; i++) {
        log_primes[i] = log((double)primes[i]);
    }

    double limit = (double)N * log((double)N);

    long long ans = 0;
    for (int i = 0; i < num_primes; i++) {
        int p = primes[i];
        double lp = log_primes[i];

        /* Binary search for max index where p*log(q) + q*log(p) < N*log(N) */
        int low = i;
        int high = num_primes;
        while (low + 1 < high) {
            int mid = (low + high) / 2;
            int q = primes[mid];
            double val = (double)p * log_primes[mid] + (double)q * lp;
            if (val < limit) {
                low = mid;
            } else {
                high = mid;
            }
        }
        ans += low - i;
    }

    printf("%lld\n", ans);

    free(primes);
    free(log_primes);
    return 0;
}
"""

def main():
    with tempfile.TemporaryDirectory() as tmpdir:
        src = os.path.join(tmpdir, "p800.c")
        exe = os.path.join(tmpdir, "p800")
        with open(src, "w") as f:
            f.write(C_CODE)
        subprocess.run(["gcc", "-O2", "-o", exe, src, "-lm"], check=True)
        result = subprocess.run([exe], capture_output=True, text=True, timeout=280)
        print(result.stdout.strip())

if __name__ == "__main__":
    main()
