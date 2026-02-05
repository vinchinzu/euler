#!/usr/bin/env python3
"""Project Euler Problem 394 - Eating Pie

Jeff eats a pie by repeatedly making two cuts and eating the first two slices.
Find the expected number of steps before he finishes when threshold is 1/N.

Solution: Uses the closed-form formula derived from differential equations:
E(x) = (6*ln(x) + 2/x³ + 7) / 9
"""

import math

def solve():
    N = 40
    # E(x) = (6*ln(x) + 2/x³ + 7) / 9
    result = (6 * math.log(N) + 2.0 / (N**3) + 7) / 9
    return f"{result:.10f}"

if __name__ == "__main__":
    print(solve())
