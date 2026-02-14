"""Project Euler Problem 756: Approximating a Sum — embedded C with GMP."""

import subprocess, tempfile, os

def solve():
    c_code = r'''
#include <stdio.h>
#include <stdlib.h>
#include <gmp.h>

int main() {
    int N = 12345678;
    int K = 12345;

    /* Sieve for Euler's totient */
    int *phi = (int*)malloc((N + 1) * sizeof(int));
    for (int i = 0; i <= N; i++) phi[i] = i;
    for (int i = 2; i <= N; i++) {
        if (phi[i] == i) {
            for (int j = i; j <= N; j += i)
                phi[j] = phi[j] / i * (i - 1);
        }
    }

    /* Prefix sums */
    long long *sum_phis = (long long*)malloc((N + 1) * sizeof(long long));
    sum_phis[0] = 0;
    for (int i = 1; i <= N; i++)
        sum_phis[i] = sum_phis[i - 1] + phi[i];
    free(phi);

    /* GMP accumulation */
    mpf_set_default_prec(256);
    mpf_t d, ans, diff, temp, num_t, den_t;
    mpf_inits(d, ans, diff, temp, num_t, den_t, NULL);

    mpf_set_si(ans, sum_phis[N]);
    mpf_set_ui(num_t, (unsigned long)K);
    mpf_set_ui(den_t, (unsigned long)N);
    mpf_div(d, num_t, den_t);

    for (int i = 1; i <= N; i++) {
        long long tail = sum_phis[N] - sum_phis[i - 1];
        mpf_set_si(temp, tail);
        mpf_mul(diff, d, temp);

        if (mpf_get_d(diff) == 0.0) break;

        mpf_sub(ans, ans, diff);

        long long nr = (long long)N - K - i + 1;
        if (nr <= 0) break;
        long long dn = (long long)N - i;
        if (dn <= 0) break;

        mpf_set_ui(num_t, (unsigned long)nr);
        mpf_mul(d, d, num_t);
        mpf_set_ui(den_t, (unsigned long)dn);
        mpf_div(d, d, den_t);
    }

    gmp_printf("%.6Ff\n", ans);

    mpf_clears(d, ans, diff, temp, num_t, den_t, NULL);
    free(sum_phis);
    return 0;
}
'''
    with tempfile.NamedTemporaryFile(suffix='.c', delete=False) as f:
        f.write(c_code.encode())
        c_file = f.name
    exe = c_file[:-2]
    subprocess.run(['gcc', '-O3', '-o', exe, c_file, '-lgmp'], check=True, capture_output=True)
    result = subprocess.check_output([exe], timeout=280).decode().strip()
    os.unlink(c_file)
    os.unlink(exe)
    return result

if __name__ == "__main__":
    print(solve())
