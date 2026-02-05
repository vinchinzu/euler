"""Project Euler Problem 461: Almost Pi.

Let f_n(k) = e^{k/n} - 1. Find a² + b² + c² + d² for the tuple (a, b, c, d)
that minimizes |f_n(a) + f_n(b) + f_n(c) + f_n(d) - π|.

Solution uses a C helper for speed.
"""

from __future__ import annotations

import os
import subprocess


def solve() -> int:
    """Solve Problem 461 using the C helper."""
    c_file = os.path.join(os.path.dirname(__file__), "461_helper.c")
    c_bin = "/tmp/p461_euler"

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
