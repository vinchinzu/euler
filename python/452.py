"""Project Euler Problem 452: N-tuples with product <= N.

Find the number of N-tuples of positive integers whose product does not exceed N.

Solution uses a C helper for speed.
"""

from __future__ import annotations

import os
import subprocess


def solve() -> int:
    """Solve Problem 452 using the C helper."""
    c_file = os.path.join(os.path.dirname(__file__), "452_helper.c")
    c_bin = "/tmp/p452_euler"

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
