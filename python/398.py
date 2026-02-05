#!/usr/bin/env python3
"""Project Euler Problem 398 - Cutting Rope

A rope of length N has N-1 points. If cut at K-1 points to form K pieces,
find the expected size of the second smallest piece.

Solution uses C helper for speed.
"""

import subprocess
import os

def solve():
    c_file = os.path.join(os.path.dirname(__file__), "398_c_helper.c")
    c_bin = "/tmp/p398_euler"

    if not os.path.exists(c_bin):
        subprocess.run(["gcc", "-O2", c_file, "-o", c_bin, "-lm"], check=True)

    result = subprocess.run([c_bin], capture_output=True, text=True, check=True)
    # Return just stdout, ignoring stderr warnings
    return result.stdout.strip()

if __name__ == "__main__":
    print(solve())
