"""Project Euler Problem 390 - Triangles with Non Rational Sides and Integral Area.

Compute S(n) = sum of areas of all triangles with sides
    sqrt(1 + b^2), sqrt(1 + c^2), sqrt(b^2 + c^2),
for positive integers b <= c, whose area is a positive integer not exceeding n.

For integral area A: 4A^2 = b^2*c^2 + b^2 + c^2, requiring (b^2+1)(c^2+1) = 4A^2+1.
Both b and c must be even. Direct search over even (a, t) with perfect-square check.

Uses a compiled C helper for the performance-critical inner loop.
"""

from __future__ import annotations

import os
import subprocess
import tempfile

C_SOURCE = r"""
#include <stdio.h>
#include <math.h>
#include <stdint.h>

typedef unsigned __int128 uint128;

static inline uint64_t isqrt128(uint128 n) {
    if (n == 0) return 0;
    uint64_t x = (uint64_t)sqrt((double)n);
    for (int i = 0; i < 5; i++) {
        if (x == 0) break;
        uint128 x128 = (uint128)x;
        uint128 next = (x128 + n / x128) / 2;
        if (next >= x128 && next - x128 <= 1) break;
        if (x128 >= next && x128 - next <= 1) break;
        x = (uint64_t)next;
    }
    while ((uint128)x * x > n) x--;
    while ((uint128)(x + 1) * (x + 1) <= n) x++;
    return x;
}

int main(void) {
    const uint64_t N = 20000000000ULL;  /* 2 * 10^10 */
    uint128 ans = 0;

    for (uint64_t a = 2; (uint128)a * a + 1 <= N; a += 2) {
        uint64_t a2 = a * a;
        uint64_t upper_bound = N / (a2 + 1);
        for (uint64_t t = 2; t <= upper_bound; t += 2) {
            uint128 s = (uint128)a2 * t * t - a2 + (uint128)t * t;
            uint64_t v = isqrt128(s);
            if ((uint128)v * v != s) continue;
            uint128 b = (uint128)a * t + v;
            uint128 n_val = (uint128)a * b + t;
            if (n_val > N) break;
            ans += n_val / 2;
        }
    }

    uint64_t hi = (uint64_t)(ans / 1000000000000ULL);
    uint64_t lo = (uint64_t)(ans % 1000000000000ULL);
    if (hi > 0) printf("%lu%012lu\n", hi, lo);
    else printf("%lu\n", lo);
    return 0;
}
"""


def main() -> None:
    """Compute and print S(10^10)."""
    with tempfile.TemporaryDirectory() as tmpdir:
        c_path = os.path.join(tmpdir, "sol.c")
        bin_path = os.path.join(tmpdir, "sol")
        with open(c_path, "w") as f:
            f.write(C_SOURCE)
        subprocess.run(["gcc", "-O2", "-o", bin_path, c_path, "-lm"], check=True)
        result = subprocess.run(
            [bin_path], capture_output=True, text=True, check=True, timeout=280
        )
    print(result.stdout.strip())


if __name__ == "__main__":
    main()
