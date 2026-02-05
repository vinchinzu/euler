"""Project Euler Problem 522: Hilbert's Hotel.

In a Hilbert Hotel graph G, each vertex has a directed edge to some other
vertex. Let f(G) be the minimum number of vertices whose directed edge
needs to be updated to another vertex, such that we obtain a single cycle
over all nodes. Find Σ f(G) over all Hilbert Hotel graphs G with N
vertices.

Solution uses a C helper for speed.
"""

from __future__ import annotations

import os
import subprocess


def solve() -> int:
    """Solve Problem 522 using the C helper."""
    c_file = os.path.join(os.path.dirname(__file__), "522_helper.c")
    c_bin = "/tmp/p522_euler"

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
