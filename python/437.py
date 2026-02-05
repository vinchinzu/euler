"""Project Euler Problem 437: Fibonacci primitive roots.

An integer x is a Fibonacci primitive root (mod p) if its powers cycle
through all values from 1 to p-1 (mod p), and x^n + x^{n+1} ≡ x^{n+2}.
Find the sum of all primes p ≤ N that have a Fibonacci primitive root.

From Lemmas 5 and 6 in this paper:
https://www.mathstat.dal.ca/FQ/Scanned/15-4/deleon.pdf, an equivalent
condition is that either p = 5, or p = 1 or 9 (mod 10) and the Fibonacci
numbers (mod p) have period p-1. The first condition is equivalent to p² ≢
4 (mod 5). For the second condition, we need to check the pairs of
Fibonacci numbers F_{(p-1)/d} and F_{(p-1)/d + 1} for all divisors d of
p-1. If none of the pairs are congruent to 0 and 1 respectively, then the
prime p has a Fibonacci primitive root.
"""

from __future__ import annotations

import os
import subprocess


def solve() -> int:
    """Solve Problem 437 using the C helper."""
    c_file = os.path.join(os.path.dirname(__file__), "437_helper.c")
    c_bin = "/tmp/p437_euler"

    if not os.path.exists(c_bin):
        subprocess.run(["gcc", "-O2", c_file, "-o", c_bin], check=True)

    result = subprocess.run([c_bin], capture_output=True, text=True, check=True)
    return int(result.stdout.strip())


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
