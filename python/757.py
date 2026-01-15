"""Project Euler Problem 757: Stealthy Numbers.

Find the number of integers n ≤ N that can be expressed as n = a*b = c*d
where a+b = c+d+1.

Given a≤c, we have b = N/a and d = N/c, so:

a + N/a = c + N/c + 1
   => N = a*c*(c-a+1)/(c-a).

This means b = c*(c-a+1)/(c-a), and we must have c-a|c. Similarly,
d = a*(c-a+1)/(c-a), so c-a|a. This means c-a = GCD(a,c) = g, and
consequently a = g*x and c = g*(x+1) for some x. This results in
N = x(x+1)g(g+1), and we can easily find all possible values of N.
"""

from __future__ import annotations

from math import isqrt
from typing import List


def solve() -> int:
    """Solve Problem 757."""
    N = 10**14
    L = 10**8

    stealthies: List[int] = []
    size = 0

    # Generate all stealthy numbers
    x = 1
    while x * (x + 1) * x * (x + 1) <= N:
        g = x
        while x * (x + 1) * g * (g + 1) <= N:
            stealthies.append(x * (x + 1) * g * (g + 1))
            size += 1
            g += 1
        x += 1

    # Sort and count distinct values
    stealthies.sort()
    count = 0
    for i in range(size):
        if i == 0 or stealthies[i] != stealthies[i - 1]:
            count += 1

    return count


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
