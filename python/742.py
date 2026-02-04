"""Project Euler Problem 742: Minimum Area of a Convex Grid Polygon.

Find the minimum area of a convex polygon with N lattice point vertices and
with both horizontal and vertical symmetry.

Uses a C helper for performance. See 742_helper.c for the algorithm.
"""

from __future__ import annotations

import os
import subprocess


def solve():
    """Solve Problem 742."""
    script_dir = os.path.dirname(os.path.abspath(__file__))
    helper_c = os.path.join(script_dir, "742_helper.c")
    helper_bin = os.path.join(script_dir, "742_helper")
    if not os.path.exists(helper_bin) or os.path.getmtime(helper_c) > os.path.getmtime(helper_bin):
        subprocess.run(["gcc", "-O2", "-lm", "-o", helper_bin, helper_c], check=True)

    result = subprocess.run(
        [helper_bin],
        capture_output=True, text=True, check=True
    )
    return int(result.stdout.strip())


def main():
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
