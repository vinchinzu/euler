"""Project Euler Problem 325 - Stone Game with optimized C implementation."""
import tempfile
import subprocess
import os

def solve():
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

int main() {
    const long long N = 10000000000000000LL;  // 10^16
    const double phi = (1.0 + sqrt(5.0)) / 2.0;

    // Part 1: 3*(sum y^2 - sum y)/2
    long long n_mod = N % M;
    long long np1_mod = (N + 1) % M;
    long long tn1_mod = (2 * N + 1) % M;

    long long sum_y = (__int128)n_mod * np1_mod % M * mod_inv(2, M) % M;
    long long sum_y2 = (__int128)n_mod * np1_mod % M * tn1_mod % M * mod_inv(6, M) % M;
    long long part1 = (3 * ((sum_y2 - sum_y + M) % M) % M * mod_inv(2, M)) % M;

    // Parts 2 and 3: Iterate over distinct f values
    long long sum_f = 0, sum_f2 = 0, sum_yf = 0;
    long long max_f = (long long)(N / phi);

    // Use a step size to process in chunks
    // For large N, we can't iterate one by one
    // Instead, use the fact that y ranges are approximately phi wide
    // So we can compute in O(N/phi) time which is still 6e15 iterations

    // Alternative: Use recursion or more advanced formula
    // For now, let's try batching

    long long f = 0;
    while (f <= max_f) {
        // Find y range for f
        long long y_min, y_max;
        if (f == 0) {
            y_min = 1;
            y_max = (long long)phi;
            if (y_max > N) y_max = N;
        } else {
            y_min = (long long)(f * phi) + 1;
            // Adjust y_min to ensure floor(y_min / phi) >= f
            while (y_min > 0 && (long long)(y_min / phi) < f) y_min++;

            y_max = (long long)((f + 1) * phi);
            // Adjust y_max to ensure floor(y_max / phi) <= f
            while (y_max > y_min && (long long)(y_max / phi) > f) y_max--;

            if (y_max > N) y_max = N;
        }

        if (y_max >= y_min && y_max >= 1) {
            long long count = y_max - y_min + 1;
            long long y_min_mod = y_min % M;
            long long y_max_mod = y_max % M;
            long long count_mod = count % M;

            long long sum_y_range = ((__int128)(y_min_mod + y_max_mod) % M * count_mod % M * mod_inv(2, M)) % M;

            long long f_mod = f % M;
            sum_f = (sum_f + (__int128)f_mod * count_mod) % M;
            sum_f2 = (sum_f2 + (__int128)f_mod * f_mod % M * count_mod) % M;
            sum_yf = (sum_yf + (__int128)f_mod * sum_y_range) % M;
        }

        f++;

        // Progress indicator for very large f
        if (f % 1000000000LL == 0) {
            fprintf(stderr, "Progress: f = %lld / %lld\n", f, max_f);
        }
    }

    long long part2 = ((sum_f2 + sum_f) % M * mod_inv(2, M)) % M;
    long long part3 = sum_yf % M;

    long long ans = (part1 - part2 - part3 + 2*M) % M;

    printf("%lld\n", ans);
    return 0;
}
'''

    with tempfile.TemporaryDirectory() as tmpdir:
        src_file = os.path.join(tmpdir, "solve.c")
        bin_file = os.path.join(tmpdir, "solve")

        with open(src_file, 'w') as f:
            f.write(c_code)

        result = subprocess.run(
            ["gcc", "-O3", "-o", bin_file, src_file, "-lm"],
            capture_output=True,
            text=True
        )
        if result.returncode != 0:
            raise RuntimeError(f"Compilation failed: {result.stderr}")

        # This will take a very long time - maybe hours
        result = subprocess.run([bin_file], capture_output=True, text=True, timeout=3600)
        if result.returncode != 0:
            raise RuntimeError(f"Execution failed: {result.stderr}")

        return int(result.stdout.strip())

if __name__ == "__main__":
    print(solve())
