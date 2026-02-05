"""Project Euler Problem 338 - Cutting Rectangles with C acceleration."""
import tempfile
import subprocess
import os

def solve():
    c_code = r'''
#include <stdio.h>
#include <stdint.h>
#include <math.h>

#define M 100000000LL  // 10^8

long long isqrt_ll(long long n) {
    if (n < 0) return 0;
    if (n < 2) return n;
    long long x = n, y = (x + 1) / 2;
    while (y < x) {
        x = y;
        y = (x + n / x) / 2;
    }
    return x;
}

long long sum_floor_quotients(long long m) {
    // sum_{k=1}^m floor(m/k)
    long long s = isqrt_ll(m);
    long long result = 0;
    for (long long k = 1; k <= s; k++) {
        result = (result + m / k) % M;
    }
    result = (2 * result - (__int128)s * s % M + M) % M;
    return result;
}

long long num_triplets_mod(long long N) {
    // Count ordered triplets (a,b,c) with a*b*c <= N, mod M
    // = sum_{a=1}^N D(floor(N/a)) where D(m) = sum_{k=1}^m floor(m/k)
    long long total = 0;
    long long a = 1;
    while (a <= N) {
        long long v = N / a;
        // Find range of a with same floor(N/a) = v
        long long a_end = N / v;
        long long count = (a_end - a + 1) % M;
        long long dfq = sum_floor_quotients(v);
        total = (total + (__int128)count * dfq) % M;
        a = a_end + 1;
    }
    return total;
}

int main() {
    const long long N = 1000000000000LL;  // 10^12
    const long long L = isqrt_ll(N);

    long long ans = 0;

    // Part 1: sum_{k=2}^N floor(N/k)*floor(N/(k-1))
    // For k = 2 to L
    for (long long k = 2; k <= L; k++) {
        long long nk = N / k % M;
        long long nkm1 = N / (k - 1) % M;
        ans = (ans + (__int128)nk * nkm1) % M;
    }

    // For k > L, group by t = floor(N/k)
    for (long long t = 1; t < N / L; t++) {
        long long block = N / t - N / (t + 1);
        long long val = ((((block - 1) % M) * ((__int128)(t % M) * (t % M) % M) % M +
                         (__int128)(t % M) * ((t + 1) % M) % M) % M + M) % M;
        ans = (ans + val) % M;
    }

    // Part 2: numTripletsWithProductAtMost(N)
    long long triplets = num_triplets_mod(N);

    // Part 3: sumFloorQuotients(N)
    long long sfq = sum_floor_quotients(N);

    // Combine
    long long result = (ans - triplets + sfq + 2*M) % M;

    printf("%lld\n", result);
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

        result = subprocess.run([bin_file], capture_output=True, text=True, timeout=180)
        if result.returncode != 0:
            raise RuntimeError(f"Execution failed: {result.stderr}")

        return int(result.stdout.strip())

if __name__ == "__main__":
    print(solve())
