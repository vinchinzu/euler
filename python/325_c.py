"""Project Euler Problem 325 - Stone Game with C acceleration."""
import tempfile
import subprocess
import os

def solve():
    N = 10**16
    M = 7**10

    # Write C code for the floor sum computation
    c_code = r'''
#include <stdio.h>
#include <stdint.h>
#include <math.h>

#define M 282475249LL  // 7^10

long long mod_inv(long long a, long long m) {
    long long result = 1, base = a % m, exp = m - 2;
    while (exp > 0) {
        if (exp & 1) result = (__int128)result * base % m;
        base = (__int128)base * base % m;
        exp >>= 1;
    }
    return result;
}

long long sum_1_to_n(long long n) {
    long long n_mod = n % M;
    long long np1_mod = (n + 1) % M;
    long long inv2 = mod_inv(2, M);
    return (__int128)n_mod * np1_mod % M * inv2 % M;
}

long long sum_squares_1_to_n(long long n) {
    long long n_mod = n % M;
    long long np1_mod = (n + 1) % M;
    long long tn1_mod = (2 * n + 1) % M;
    long long inv6 = mod_inv(6, M);
    return (__int128)n_mod * np1_mod % M * tn1_mod % M * inv6 % M;
}

int main() {
    const long long N = 10000000000000000LL;  // 10^16
    const double phi = (1.0 + sqrt(5.0)) / 2.0;
    const double phi_inv = 2.0 / (1.0 + sqrt(5.0));

    // Part 1
    long long sum_y2 = sum_squares_1_to_n(N);
    long long sum_y = sum_1_to_n(N);
    long long part1 = (3LL * ((sum_y2 - sum_y + M) % M) % M * mod_inv(2, M)) % M;

    // Parts 2 and 3 using hyperbola method
    // We need to iterate smartly, not over all f values
    // Instead, iterate over y ranges where floor(y/phi) changes

    long long sum_f = 0, sum_f2 = 0, sum_yf = 0;

    long long y = 1;
    while (y <= N) {
        long long f = (long long)(y * phi_inv);
        // Find max y with floor(y/phi) = f
        // floor(y/phi) = f means f*phi <= y < (f+1)*phi
        long long y_next = (long long)((f + 1) * phi);
        long long y_max = (y_next <= N) ? y_next - 1 : N;

        long long count = y_max - y + 1;
        long long sum_y_range = ((y % M) + (y_max % M)) % M * ((count % M) * mod_inv(2, M) % M) % M;

        sum_f = (sum_f + (__int128)(f % M) * (count % M)) % M;
        sum_f2 = (sum_f2 + (__int128)(f % M) * (f % M) % M * (count % M)) % M;
        sum_yf = (sum_yf + (__int128)(f % M) * sum_y_range) % M;

        y = y_max + 1;
    }

    long long part2 = ((sum_f2 + sum_f) % M * mod_inv(2, M)) % M;
    long long part3 = sum_yf % M;

    long long ans = (part1 - part2 - part3 + 2*M) % M;

    printf("%lld\n", ans);
    return 0;
}
'''

    # Compile and run
    with tempfile.TemporaryDirectory() as tmpdir:
        src_file = os.path.join(tmpdir, "solve.c")
        bin_file = os.path.join(tmpdir, "solve")

        with open(src_file, 'w') as f:
            f.write(c_code)

        # Compile
        result = subprocess.run(
            ["gcc", "-O3", "-o", bin_file, src_file, "-lm"],
            capture_output=True,
            text=True
        )
        if result.returncode != 0:
            raise RuntimeError(f"Compilation failed: {result.stderr}")

        # Run
        result = subprocess.run([bin_file], capture_output=True, text=True, timeout=180)
        if result.returncode != 0:
            raise RuntimeError(f"Execution failed: {result.stderr}")

        return int(result.stdout.strip())

if __name__ == "__main__":
    print(solve())
