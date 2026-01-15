"""Project Euler Problem 693: Finite Sequence Generator.

Define g(x) to be the maximum number of terms of a sequence starting at some
positive integer a_x = y < x, and a_{z+1} = (a_z)² (mod z), until the sequence
becomes 0 or 1. Find the maximum value of g(x) for x≤n.

We have g(x+1) ≥ g(x)-1, because if g(x) is obtained for some y, it's always
possible to use the next term in that sequence for g(x+1). This means that in
some range [l, h], the highest possible value of g is g(h)+h-l. So we can use
depth-limited search on [0, N], skipping any ranges that cannot possibly become
larger than the currently found maximum.

To compute g(x), we can simulate the sequences for all starting y, simultaneously
because many sequences have the same suffix. We use raw arrays to avoid allocations.
"""

from __future__ import annotations

from functools import lru_cache


def sq(n: int) -> int:
    """Square of n."""
    return n * n


def imod(a: int, b: int) -> int:
    """Integer modulo."""
    return a % b


@lru_cache(maxsize=None)
def g(x: int) -> int:
    """Compute g(x)."""
    if x <= 2:
        return 0

    ys = [i for i in range(2, x)]
    ys_size = len(ys)
    used = {}
    z = x

    while True:
        if ys_size == 0:
            return z - x + 1

        new_ys = []
        used_this_z = set()
        for i in range(ys_size):
            new_y = imod(sq(ys[i]), z)
            if new_y > 1 and new_y not in used_this_z:
                new_ys.append(new_y)
                used_this_z.add(new_y)

        ys = new_ys
        ys_size = len(ys)
        z += 1


def solve() -> int:
    """Solve Problem 693."""
    N = 3000000
    ans = 0

    def helper(low: int, high: int, depth: int) -> None:
        """Helper function for depth-limited search."""
        nonlocal ans
        g_high = g(high)
        if g_high > ans:
            ans = g_high
        if low + 1 == high or depth == 0 or ans >= g_high + high - low:
            return
        mid = (low + high) // 2
        helper(low, mid, depth - 1)
        helper(mid, high, depth - 1)

    depth = 1
    while 1 << depth < N:
        depth += 1

    helper(0, N, depth)
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
