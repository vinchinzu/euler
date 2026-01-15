"""Project Euler Problem 230: Fibonacci Words.

Define F(A, B) to be the sequence A, B, AB, BAB, where each term is the
concatenation of the previous two. Find sum of digits at specific positions.
"""

from __future__ import annotations

from typing import List


def solve() -> int:
    """Solve Problem 230."""
    A = (
        "14159265358979323846264338327950288419716939937510"
        "58209749445923078164062862089986280348253421170679"
    )
    B = (
        "82148086513282306647093844609550582231725359408128"
        "48111745028410270193852110555964462294895493038196"
    )

    def D(n: int) -> int:
        """Return nth digit of the sequence."""
        lens: List[int] = [len(A), len(B)]
        while lens[-1] < n:
            lens.append(lens[-2] + lens[-1])

        k = len(lens) - 1
        while k >= 2:
            if n <= lens[k - 2]:
                k -= 2
            else:
                n -= lens[k - 2]
                k -= 1

        source = A if k == 0 else B
        return int(source[n - 1])

    ans = 0
    for n in range(18):
        pos = (127 + 19 * n) * (7**n)
        ans += (10**n) * D(pos)

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
