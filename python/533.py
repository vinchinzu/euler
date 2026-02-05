"""Project Euler Problem 533: Carmichael Lambda Function.

Let λ(n) be the smallest positive integer m such that a^m ≡ 1 (mod n) for
all (a, n) = 1. Find the smallest m such that λ(k) ≥ N for all k ≥ m.

Solution uses a C helper for speed.
"""

from __future__ import annotations

import os
import subprocess


def solve() -> int:
    """Solve Problem 533 using the C helper."""
    c_file = os.path.join(os.path.dirname(__file__), "533_helper.c")
    c_bin = "/tmp/p533_euler"

    if not os.path.exists(c_bin):
        subprocess.run(["gcc", "-O2", c_file, "-o", c_bin, "-lm"], check=True)

    result = subprocess.run([c_bin], capture_output=True, text=True, check=True)
    return int(result.stdout.strip())


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
