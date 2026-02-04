"""Project Euler Problem 686: Powers of Two.

Find the Nth positive integer j such that 2^j starts with the digits "123".
Uses logarithms: fractional part of j*log10(2) determines leading digits.
Compiled C inner loop for speed.
"""

from __future__ import annotations

import subprocess
import tempfile
import os


def solve() -> int:
    """Solve Problem 686 using compiled C for performance."""
    c_code = r"""
#include <stdio.h>
#include <math.h>

int main(void) {
    int N = 678910;
    double log2 = log10(2.0);
    double lo = log10(1.23);
    double hi = log10(1.24);
    int count = 0;
    long long j = 0;
    while (count < N) {
        j++;
        double val = j * log2;
        double frac = val - (long long)val;
        if (frac >= lo && frac < hi) {
            count++;
        }
    }
    printf("%lld\n", j);
    return 0;
}
"""
    with tempfile.NamedTemporaryFile(suffix='.c', mode='w', delete=False) as f:
        f.write(c_code)
        c_path = f.name

    bin_path = c_path.replace('.c', '')
    try:
        subprocess.run(['gcc', '-O2', '-o', bin_path, c_path, '-lm'],
                       check=True, capture_output=True)
        result = subprocess.run([bin_path], capture_output=True, text=True, check=True)
        return int(result.stdout.strip())
    finally:
        os.unlink(c_path)
        if os.path.exists(bin_path):
            os.unlink(bin_path)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
