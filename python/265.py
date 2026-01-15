"""Project Euler Problem 265: Binary Circles.

Find all circular arrangements of 2^N bits such that all clockwise
subsequences of length N are unique, and find the sum of all arrangements
when interpreted as a binary number starting from the subsequence of all
zeros.
"""

from __future__ import annotations

from typing import Set


def solve() -> int:
    """Solve Problem 265."""
    N = 5
    ans = [0]

    def helper(index: int, seq: int, used: Set[int]) -> None:
        """Recursive helper."""
        if index == (1 << N) - 1:
            ans[0] += seq >> (N - 1)
            return

        for bit in range(2):
            subseq = (seq % (1 << (N - 1))) * 2 + bit
            if subseq not in used:
                used.add(subseq)
                helper(index + 1, seq * 2 + bit, used)
                used.remove(subseq)

    helper(0, 0, {0})
    return ans[0]


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
