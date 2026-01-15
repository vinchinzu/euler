"""Project Euler Problem 576: Irrational jumps.

Let S(l, g, d) be the total distance a point travels if it starts on a circle
and makes jumps of length l counter-clockwise until it falls into a gap at
position d with size g. Find the maximum value of Σ_p S(√(1/p),g,d) for all
primes p≤N over all d.

We can generate all jump locations for the point, stopping once all jump
locations for a given p are within d of each other (because it wouldn't be
possible to jump further). Now maintain a sliding window of width d over the
sorted jump locations. For each window, find Σ_p S(√(1/p),g,d), and take the
maximum over all windows.
"""

from __future__ import annotations

import math
from dataclasses import dataclass
from math import isqrt
from typing import List


@dataclass
class JumpPosition:
    """Represents a jump position."""

    index: int
    total_len: float


def sieve(limit: int) -> List[int]:
    """Generate all primes up to limit using Sieve of Eratosthenes."""
    if limit < 2:
        return []
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, isqrt(limit) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(limit + 1) if is_prime[i]]


def fractional_part(x: float) -> float:
    """Return fractional part of x."""
    return x - math.floor(x)


def solve() -> float:
    """Solve Problem 576."""
    N = 100
    D = 0.00002

    primes = sieve(N)
    all_jump_positions: List[JumpPosition] = []

    # Generate jump positions for each prime
    for p in primes:
        if p < 2:
            continue
        jump_positions: List[JumpPosition] = []
        sqrt_inv_p = math.sqrt(1.0 / p)

        i = 0
        while True:
            jump_positions.append(JumpPosition(p, i * sqrt_inv_p))
            i += 1

            # Check if all positions are within D
            if i > 1 and (i & (i + 1)) == 0:
                jump_positions.sort(key=lambda pos: fractional_part(pos.total_len))
                if len(jump_positions) > 1:
                    all_within = True
                    for j in range(1, len(jump_positions)):
                        if (
                            fractional_part(jump_positions[j].total_len)
                            - fractional_part(jump_positions[j - 1].total_len)
                            > D
                        ):
                            all_within = False
                            break
                    if all_within:
                        break

        all_jump_positions.extend(jump_positions)

    # Sort all positions by fractional part
    all_jump_positions.sort(key=lambda pos: fractional_part(pos.total_len))

    # Sliding window
    ans = 0.0
    start = 0
    end = len(primes)

    while end < len(all_jump_positions):
        # Adjust window
        while (
            fractional_part(all_jump_positions[end].total_len)
            - fractional_part(all_jump_positions[start].total_len)
            > D
        ):
            start += 1

        # Compute S for each prime
        S: dict[int, float] = {}
        for p in primes:
            S[p] = float("inf")

        for i in range(start, end):
            pos = all_jump_positions[i]
            if pos.total_len < S[pos.index]:
                S[pos.index] = pos.total_len

        # Sum S values
        total = sum(S[p] for p in primes if S[p] != float("inf"))
        if total > ans:
            ans = total

        end += 1

    return ans


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.4f}")
    return result


if __name__ == "__main__":
    main()
