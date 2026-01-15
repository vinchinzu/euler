"""Project Euler Problem 729: Range of Periodic Sequence.

Find the sum of the ranges of every distinct sequence a_{n+1} = a_n - 1/a_n
with period ≤ N.

Every a_{n+1} can be reached from two distinct a_n, given by
a_n = f(a_{n+1}) = ( a_{n+1} ± √((a_{n+1})²+4) ) / 2, and so any a_N can be
reached from 2^N possible starting values a_0. We can try each of the 2^N
branching combinations, finding the solution to f^N(a_0) = a_0 by repeatedly
applying f^N until the value stabilizes. The range of any sequence can be
computed straightforwardly.

For efficiency, we don't need to find starting values a_0 which are a_i in some
other sequence, i.e. we don't need to consider branches which are cycles of
other branches. Also, we don't want to consider branching combinations which
consist of multiple copies of a smaller branching combination (those will
already be counted for the smaller period). This set of branching combinations
are exactly the Lyndon words, for which there is a direct enumeration algorithm
maintaining the current "branches" word and its length "len". For each Lyndon
word, we find the corresponding a_0, and multiply by the number of distinct
rotations of that word.
"""

from __future__ import annotations

import math
from typing import Set


def hypot(x: float, y: float) -> float:
    """Compute sqrt(x^2 + y^2)."""
    return math.sqrt(x * x + y * y)


def parity(n: int) -> int:
    """Return 1 if n is even, -1 if odd."""
    return 1 if n % 2 == 0 else -1


def feq(a: float, b: float, eps: float = 1e-10) -> bool:
    """Check if two floats are approximately equal."""
    return abs(a - b) < eps


def solve() -> float:
    """Solve Problem 729."""
    N = 25
    d = 1.0
    ans = 0.0

    branches = 1
    length = N

    while length > 1:
        # Find fixed point by iterating f^len
        prev_d = 0.0
        while not feq(d, prev_d):
            prev_d = d
            for i in range(length):
                sign = parity(branches >> i)
                d = (d + sign * hypot(d, 2)) / 2

        # Compute range
        min_val = d
        max_val = d
        for i in range(length):
            sign = parity(branches >> i)
            d = (d + sign * hypot(d, 2)) / 2
            min_val = min(min_val, d)
            max_val = max(max_val, d)

        # Count distinct rotations
        cycles: Set[int] = set()
        cycle = branches
        for i in range(length):
            cycles.add(cycle)
            # Rotate left
            cycle = ((cycle << 1) % (1 << length)) + (cycle >> (length - 1))

        ans += len(cycles) * (max_val - min_val)

        # Generate next Lyndon word
        # Extend to length N if needed
        while length < N:
            length *= 2
            branches = (branches << (length // 2)) + branches

        # Find next Lyndon word
        branches >>= length - N
        branches += 1
        length = N
        while branches % 2 == 0:
            branches //= 2
            length -= 1

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(f"{result:.4f}")
    return 0


if __name__ == "__main__":
    main()
