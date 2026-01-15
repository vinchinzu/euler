"""Project Euler Problem 414: Kaprekar constant.

Start with a 5-digit number, sort the digits in increasing order, and then
subtract that from the number with digits sorted in decreasing order. In base
b = 6k+3, repeatedly doing this to any number with more than one distinct
digit will eventually result in the "Kaprekar constant" of that base, and we
can define sb(i) to be the number of times we apply this operation to i before
arriving at the Kaprekar constant. (If i consists of 5 identical digits, then
we define sb(i) = 0). Let S(b) = sum_{i=1}^{b^5 - 1} sb(i) in base b. Find
sum_{k=2}^N S(6k+3).

First, note that sb(i) depends on only d1(i), the difference between the
largest and smallest digits, and d2(i), the difference between the second
largest and second smallest digits. To compute sb(i), we can instead compute
sb([0, 0, 0, d2, d1]), reusing any memoized computation for any pair (d1,
d2). The base case is the Kaprekar constant, which can be proved by induction
to be of the form [_, 2b/3, _, _, b/3].

The remaining step is to compute, for each (d1, d2), how many 5-digit numbers
correspond to that (d1, d2). First, the smallest digit can be one of [0, 1,
... b - d1 - 1], so there are b - d1 choices, after which the largest digit
is fixed. Then we have the following cases:

- d2 = 0. We can choose one of d1 - 1 values for the middle digits; then the
  sorted digits are of the form ABBBC. There are 20 ways to arrange these
  sorted digits. Alternatively, we can make the middle digits the same as
  either the smallest or largest digit, so that the sorted digits are of the
  form ABBBB or AAAAB, with 5 permutations each. The total count for this case
  is 20(d1 - 1) + 2(5) = 20*d1 - 10.

- d2 = d1. There are d1 - 1 ways to select a distinct middle digit, with 30
  ways of permuting the digits of the form AABCC. Or, the middle digit can
  match an existing one: AAABB or AABBB, with 10 permutations each. The
  total count is 30(d1 - 1) + 2(10) = 30*d1 - 10.

- Other. There are (d1 - d2 - 1)*(d2 - 1) ways to choose digits of the form
  ABCDE, of which there are 120 permutations; (d1 - d2 - 1) ways for ABBCD
  or ABCCD, of which there are 60 permutations, (d2 - 1) ways for AABCD or
  ABCDD, of which there are 60 permutations, one way for AABBC or ABBCC, with
  30 permutations each, and one way for AAABC or ABCCC, with 20 permutations
  each. The total count is 120(d1 - d2 - 1)(d2 - 1) + 2*60(d1 - d2 - 1) +
  2*60(d2 - 1) + 2*30 + 2*20 = 120*d2*(d1 - d2) - 20.
"""

from __future__ import annotations

from typing import List


def sb(b: int, d1: int, d2: int, cache: List[List[int]]) -> int:
    """Compute sb(b, d1, d2) using memoization."""
    # Base case: Kaprekar constant
    if d1 == 2 * b // 3 and d2 == b // 3:
        return 1

    if cache[d1][d2] == 0:
        # Construct digits [0, 0, 0, d2, d1] or [d1, d2-1, b-1, b-d2-1, b-d1]
        if d2 == 0:
            digits = [d1 - 1, b - 1, b - 1, b - 1, b - d1]
        else:
            digits = [d1, d2 - 1, b - 1, b - d2 - 1, b - d1]

        digits.sort()
        # Compute new d1 and d2
        new_d1 = digits[4] - digits[0]
        new_d2 = digits[3] - digits[1]
        cache[d1][d2] = 1 + sb(b, new_d1, new_d2, cache)

    return cache[d1][d2]


def solve() -> int:
    """Solve Problem 414."""
    N = 300
    M = 10**18
    ans = 0

    for k in range(2, N + 1):
        b = 6 * k + 3
        cache: List[List[int]] = [[0] * b for _ in range(b)]

        for d1 in range(1, b):
            for d2 in range(d1 + 1):
                mult = b - d1

                if d2 == 0:
                    mult *= 20 * d1 - 10
                elif d2 == d1:
                    mult *= 30 * d1 - 10
                else:
                    mult *= 120 * d2 * (d1 - d2) - 20

                ans = (ans + mult * sb(b, d1, d2, cache)) % M

        ans = (ans - 1) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
