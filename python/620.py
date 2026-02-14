"""Project Euler Problem 620: Gears — embedded C for speedup."""

from __future__ import annotations

import os
import subprocess
import sys
import tempfile

C_CODE = r"""
#include <stdio.h>
#include <math.h>

int main(void) {
    int N = 500;
    long long ans = 0;
    for (int s = 5; s < N - 9; s++) {
        for (int p = 5; p < N - s; p++) {
            for (int q = p + 1; q <= N - s - p; q++) {
                double a = s + p;
                double b = p + q - 2.0 * M_PI;
                double c = s + q;
                double alpha = acos((a*a + b*b - c*c) / (2.0 * a * b));
                double beta = asin(a * sin(alpha) / c);
                int g = (int)(((s + q) * beta - (s + p) * alpha) / M_PI + s + p);
                ans += g;
            }
        }
    }
    printf("%lld\n", ans);
    return 0;
}
"""


def main() -> int:
    """Main entry point."""
    with tempfile.NamedTemporaryFile(suffix=".c", mode="w", delete=False) as f:
        f.write(C_CODE)
        c_path = f.name
    bin_path = c_path.replace(".c", "")
    try:
        subprocess.run(
            ["gcc", "-O2", "-o", bin_path, c_path, "-lm"],
            check=True, capture_output=True,
        )
        result = subprocess.run(
            [bin_path],
            capture_output=True, text=True, timeout=280,
        )
        answer = int(result.stdout.strip())
        print(answer)
        return answer
    finally:
        for p in [c_path, bin_path]:
            if os.path.exists(p):
                os.unlink(p)


if __name__ == "__main__":
    main()
