"""Project Euler Problem 839: Beans in Bowls.

Suppose there are N bowls, each with a given starting number of beans. At
each step, find the left-most bowl with more beans than the bowl to its
right, and move one bean to the bowl to its right. Find the number of
steps until the bowls are sorted by number of beans.

We process the bowls from left to right. If we find a bowl which has fewer
beans than the previous bowl, we need to "even out" the two bowls by having
each bowl have the average number of beans, with the rightmost bowls
getting the remainders. For performance, we then treat these two bowls as
a single "block". Later, two adjacent blocks can also be evened out to
form a larger block. This solves the problem in linear time, since each
block is processed at most once.

Given the final state of beans, we can then count the number of steps to
get there by moving the beans from left to right.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import List


@dataclass
class Block:
    """Block of bowls with total value and length."""

    value: int
    length: int


def blum_blum_shub(seed: int, n: int) -> List[int]:
    """Generate Blum Blum Shub sequence."""
    m = 2**32
    x = seed
    result = []
    for _ in range(n):
        x = (x * x) % m
        result.append(x)
    return result


def solve() -> int:
    """Solve Problem 839."""
    N = 10**7
    seed = 0

    S = blum_blum_shub(seed, N)

    blocks: List[Block] = []
    for n in S:
        blocks.append(Block(n, 1))
        while len(blocks) >= 2:
            # Check if we need to merge blocks
            block1 = blocks[-2]
            block2 = blocks[-1]
            avg1 = (block1.value + block1.length - 1) // block1.length
            avg2 = block2.value // block2.length
            if avg1 <= avg2:
                break
            # Merge blocks
            blocks.pop()
            blocks.pop()
            blocks.append(Block(block1.value + block2.value, block1.length + block2.length))

    # Compute final state
    T: List[int] = []
    for block in blocks:
        for i in range(block.length):
            T.append((block.value + i) // block.length)

    # Count steps
    ans = 0
    for i in range(N - 1):
        diff = S[i] - T[i]
        S[i + 1] += diff
        ans += diff

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
