"""Project Euler Problem 571: Super Pandigital Numbers.

Find the smallest K numbers that are pandigital in all bases from 2 to N.

We just brute force all permutations of base N digits, then checking those
numbers in all smaller bases. The only optimization is to hardcode the
condition for base 11 (this filters out enough numbers so the remaining bases
don't need to be optimized).
"""

from __future__ import annotations

from typing import List


def solve() -> int:
    """Solve Problem 571."""
    N = 12
    K = 10

    count = 0
    ans = 0

    def is_pandigital_in_base11(n: int) -> bool:
        """Check if n is pandigital in base 11."""
        used = [False] * 11
        while n > 0:
            used[n % 11] = True
            n //= 11
        return all(used[i] for i in range(11))

    def is_pandigital(n: int, base: int) -> bool:
        """Check if n is pandigital in given base."""
        used = [False] * base
        while n > 0:
            used[n % base] = True
            n //= base
        return all(used[i] for i in range(base))

    def helper(index: int, n: int, visited: List[bool]) -> None:
        """Recursively generate permutations and check pandigital property."""
        nonlocal count, ans
        if index == N:
            if is_pandigital_in_base11(n) and all(
                is_pandigital(n, base) for base in range(2, N)
            ):
                count += 1
                ans += n
            return
        if count == K:
            return
        for i in range(N):
            if not visited[i]:
                visited[i] = True
                helper(index + 1, n * N + i, visited)
                visited[i] = False

    helper(0, 0, [False] * N)
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
