"""Project Euler Problem 426: Box-Ball System.

Consider a Box-Ball System (BBS) consisting of t_0 boxes with a ball, then
t_1 empty boxes, then t_2 boxes with a ball, etc., up to t_N. At each turn,
every ball is moved, from left to right, to the next empty box on its right.

Eventually the lengths of the consecutive occupied boxes will be constant.
Find the sum of the squares of these lengths.

We simulate a single turn. At some point in this turn, after moving the balls
in the sequence of t_i occupied boxes, none of the balls that have been moved
will have jumped over any of the remaining balls (this might not happen until
the end, in which case there are 0 remaining balls). This means we can treat
the moved balls and the remaining balls separately.

First we process the remaining balls, because then we can remove those
elements from the list before processing the moved balls, which helps
performance by avoiding copying lists. This is just the sequence starting at
t_{i+2}.

Then, we process the moved balls. All the previous balls must have filled a
box that was previously empty, so the sequence is the same up to t_i, with
the start term removed. The length of the last sequence can be computed by
taking the difference of the previous lengths of occupied boxes, and the
previous lengths of empty boxes.

For the base case, any sequence with a single term t_i must be constant, so
we can add (t_i)².
"""

from __future__ import annotations

from typing import List


def blum_blum_shub(seed: int, n: int) -> List[int]:
    """Generate Blum Blum Shub sequence."""
    m = 2**32
    x = seed
    result = []
    for _ in range(n):
        x = (x * x) % m
        result.append(x)
    return result


def sq(n: int) -> int:
    """Return n squared."""
    return n * n


def helper(start: int, lengths: List[int]) -> int:
    """Recursive helper function."""
    if start + 1 == len(lengths):
        return sq(lengths[start])

    diff = 0
    i = start
    while True:
        diff += lengths[i]
        if i + 1 == len(lengths):
            lengths.append(diff)
            return helper(start + 1, lengths)
        elif diff <= lengths[i + 1]:
            res = helper(i + 2, lengths)
            while len(lengths) > i + 1:
                lengths.pop()
            lengths.append(diff)
            return res + helper(start + 1, lengths)
        diff -= lengths[i + 1]
        i += 2


def solve() -> int:
    """Solve Problem 426."""
    N = 10**7

    # Generate lengths using Blum Blum Shub
    bbs_sequence = blum_blum_shub(0, N + 1)
    lengths = [(x % 64) + 1 for x in bbs_sequence]

    return helper(0, lengths)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
