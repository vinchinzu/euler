"""Project Euler Problem 821: 123-Separable.

Find the maximum number of elements in {S ∪ 2S ∪ 3S} ∩ {1...N} where S is
a set of integers such that S, 2S, and 3S are disjoint.

Firstly, if any integer can be expressed as 2^a 3^b c, where c has no
factors of 2 or 3, then two integers with different values of c are
unrelated in this problem. This means that for each c, we can independently
consider all integers with that c, and find the maximum number of these
integers that can be put in S.

This can be visualized as a 2D grid of all integers with a given c, with
one axis corresponding to a and the other corresponding to b. Including
any number 2^a 3^b c is equivalent to placing a L triomino with the
corner at (a,b), and we want to cover as many numbers with non-overlapping
triominos. For an infinite grid the best tiling is:

AA AA AA
ABBABBABB
 BCCBCCBCC ...
AACAACAAC
ABBABBA
 B  B
 ...

which misses numbers of the form 4*8^n and 9*27^n. But for a bound of up
to 4*8^n to 6*8^n, we can replace the triomino rooted at 6*8^n with one
rooted at 4*8^n. Also, some small finite grids have better configurations,
which are hard-coded.
"""

from __future__ import annotations

from typing import List


def solve() -> int:
    """Solve Problem 821."""
    N = 10**16

    nums: List[int] = [1, 6, 24, 54, N + 1]
    i = 384
    while i <= N:
        nums.append(i)
        i *= 8
    i = 243
    while i <= N:
        nums.append(i)
        i *= 27
    nums.sort()

    ans = N
    for i in range(len(nums) - 1):
        low = N // nums[i + 1]
        high = N // nums[i]
        low -= low // 2 + low // 3 - low // 6
        high -= high // 2 + high // 3 - high // 6
        ans -= (high - low) * i

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
