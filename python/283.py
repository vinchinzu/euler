"""Project Euler Problem 283: Integer Sided Triangles."""
from __future__ import annotations
import subprocess
import tempfile
import os


def solve() -> int:
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

/*
 * Find the sum of perimeters of all integer-sided triangles whose
 * area/perimeter ratio is a positive integer r, for r = 1..N.
 *
 * Key identity: let p = semi-perimeter, and x = p-a, y = p-b, z = p-c
 * with x >= y >= z >= 1.  Then xyz = 4 r^2 (x+y+z), and
 * (x*y - 4r^2)(x*z - 4r^2) = 4 r^2 (x^2 + 4 r^2)  [= "product"].
 *
 * We loop over r and the smallest parameter z (called "x" in the loop
 * variable below, confusingly), enumerate divisors d of "product" with
 * d <= sqrt(product), and recover the other two parameters.
 */

#define N 1000
static int spf[16000001];
static long long divisors[100000];

void build_spf(int limit) {
    for (int i = 0; i <= limit; i++) spf[i] = i;
    for (int i = 2; (long long)i * i <= limit; i++) {
        if (spf[i] == i) {
            for (int j = i * i; j <= limit; j += i) {
                if (spf[j] == j) spf[j] = i;
            }
        }
    }
}

/* Overflow-safe check: val*val <= product, using __int128 to avoid overflow */
static inline int sq_le(long long val, long long product) {
    return (__int128)val * val <= (__int128)product;
}

/* Trial division for values larger than SPF array */
static long long smallest_factor_large(long long n) {
    if (n % 2 == 0) return 2;
    for (long long i = 3; i * i <= n; i += 2) {
        if (n % i == 0) return i;
    }
    return n;
}

int main() {
    int L = (int)(2.0 * sqrt(3.0) * N);
    int max_val = 4 * N * N + L * L;
    build_spf(max_val);

    long long ans = 0;
    divisors[0] = 1;

    for (int r = 1; r <= N; r++) {
        long long r2_4 = 4LL * r * r;

        /* Get distinct prime factors of 4r^2 */
        int factors[20];
        int nfactors = 0;
        {
            int tmp = (int)r2_4;
            while (tmp > 1) {
                int f = spf[tmp];
                factors[nfactors++] = f;
                while (tmp % f == 0) tmp /= f;
            }
        }

        for (int x = 1; x <= L; x++) {
            long long product = r2_4 * (r2_4 + (long long)x * x);
            long long k = product;
            int divisors_size = 1;

            /* Add divisors from known factors of 4r^2 */
            for (int fi = 0; fi < nfactors; fi++) {
                int d = factors[fi];
                int e = 0;
                while (k % d == 0) { k /= d; e++; }
                int old_size = divisors_size;
                for (int i = old_size - 1; i >= 0; i--) {
                    long long mult = d;
                    for (int j = 0; j < e; j++) {
                        long long val = divisors[i] * mult;
                        if (sq_le(val, product))
                            divisors[divisors_size++] = val;
                        mult *= d;
                    }
                }
            }

            /* Add divisors from remaining factors */
            while (k > 1) {
                long long d;
                if (k <= max_val)
                    d = spf[(int)k];
                else
                    d = smallest_factor_large(k);
                int e = 0;
                while (k % d == 0) { k /= d; e++; }
                int old_size = divisors_size;
                for (int i = old_size - 1; i >= 0; i--) {
                    long long mult = d;
                    for (int j = 0; j < e; j++) {
                        long long val = divisors[i] * mult;
                        if (sq_le(val, product))
                            divisors[divisors_size++] = val;
                        mult *= d;
                    }
                }
            }

            /* Check each divisor */
            for (int i = 0; i < divisors_size; i++) {
                long long xy = divisors[i] + r2_4;
                long long xz = product / divisors[i] + r2_4;
                if (xy % x == 0 && xz % x == 0 && (long long)x * x <= xy)
                    ans += 2 * (x + xy / x + xz / x);
            }
        }
    }

    printf("%lld\n", ans);
    return 0;
}
"""
    tmpdir = tempfile.mkdtemp()
    c_path = os.path.join(tmpdir, "p283.c")
    bin_path = os.path.join(tmpdir, "p283")
    with open(c_path, "w") as f:
        f.write(c_code)
    subprocess.run(["gcc", "-O2", "-lm", "-o", bin_path, c_path], check=True, capture_output=True)
    result = subprocess.run([bin_path], capture_output=True, text=True, check=True)
    os.unlink(c_path)
    os.unlink(bin_path)
    os.rmdir(tmpdir)
    return int(result.stdout.strip())


def main() -> None:
    print(solve())


if __name__ == "__main__":
    main()
