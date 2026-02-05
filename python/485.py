"""Project Euler Problem 485: Maximum number of divisors.

Let M(n, k) be the maximum number of divisors of a number from n to n+k-1
inclusive. Find Σ_{n=1}^{N-K+1} M(n, K).

Solution uses a C helper for speed.
"""

from __future__ import annotations

import os
import subprocess


def solve() -> int:
    """Solve Problem 485 using the C helper."""
    c_file = os.path.join(os.path.dirname(__file__), "485_helper.c")
    c_bin = "/tmp/p485_euler"

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
