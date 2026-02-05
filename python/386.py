#!/usr/bin/env python3
"""Project Euler Problem 386 - Antichain Counting

N(n) is the maximum size of an antichain in the divisor lattice of n.
Find sum of N(n) for n=1 to 10^8.

Solution uses C helper for speed.
"""

import subprocess
import os
import tempfile

def solve():
    # Check if C helper is compiled
    c_file = os.path.join(os.path.dirname(__file__), "386_c_helper.c")
    c_bin = "/tmp/p386_euler"

    if not os.path.exists(c_bin):
        # Compile C helper
        subprocess.run(["gcc", "-O2", c_file, "-o", c_bin], check=True)

    # Run C helper
    result = subprocess.run([c_bin], capture_output=True, text=True, check=True)
    return int(result.stdout.strip())

if __name__ == "__main__":
    print(solve())
