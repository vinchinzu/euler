"""Project Euler Problem 413: One-child numbers.

A d-digit positive integer is a one-child number if exactly one of its
substrings is divisible by d. Find the number of one-child numbers with up to
N digits.

For a given d, we use dynamic programming. Suppose that after i-1 digits, we
keep track of the number of suffixes (ending at digit i-1) with each remainder
modulo d. We also keep track of the number of substrings so far that are
divisible by d.

Suppose we now add the i'th digit. We can now compute the new set of suffixes
modulo d, as well as increment the number of substrings by the number of those
suffixes that are divisible by d. After iterating over all d digits, the
answer is the number of integers with exactly one such substring.

Optimizations:
- We don't have to distinguish between 2 suffixes with a particular remainder,
  from more than 2 suffixes with that remainder. Either way, if that suffix were
  to be extended to a substring divisible by d, then we would have too many
  substrings divisible by d.
- Furthermore, if gcd(d, 10) = 1, then we don't even need to distinguish
  between 1 suffix and 2 suffixes: if there are 2 suffixes, then the
  difference between those suffixes is already a substring divisible by d, so
  we cannot let any of these suffixes to be extended to a substring divisible
  by d.
- This means we can encode the number of suffixes with each remainder modulo d,
  as a small integer, using 2 bits for each remainder, or only 1 bit if
  gcd(d, 10) = 1.
- Furthermore, at any point, all remainders must have the same remainder (mod
  gcd(d, 10)). This means that if gcd(d, 10) > 1, we only need (2 * d /
  gcd(d, 10)) bits, not (2 * d) bits.
- Finally, we also don't need to keep track of numbers with more than one
  substring divisible by d.
"""

from __future__ import annotations

from math import gcd
from typing import List


def solve() -> int:
    """Solve Problem 413."""
    N = 19
    B = 10
    ans = 0

    for d in range(1, N + 1):
        g = gcd(d, B)
        num_bits = 1 if g == 1 else 2
        max_suffix_mod_counts = g << (d // g * num_bits)

        # Cache for new suffix mod counts
        new_suffix_mod_counts_cache: List[List[int]] = [
            [0] * max_suffix_mod_counts for _ in range(B)
        ]

        # DP: dp[suffix_mod_counts][num_substrings]
        dp: List[List[int]] = [[0] * 2 for _ in range(max_suffix_mod_counts)]
        dp[0][0] = 1

        for i in range(d):
            new_dp: List[List[int]] = [
                [0] * 2 for _ in range(max_suffix_mod_counts)
            ]

            for digit in range(1 if i == 0 else 0, B):
                for suffix_mod_counts in range(max_suffix_mod_counts):
                    for num_substrings in range(2):
                        if dp[suffix_mod_counts][num_substrings] > 0:
                            # Compute new suffix mod counts
                            if new_suffix_mod_counts_cache[digit][
                                suffix_mod_counts
                            ] == 0:
                                res = 1 << (digit % d // g * num_bits)
                                r = suffix_mod_counts >> (d // g * num_bits)

                                for j in range(d // g):
                                    new_j = (
                                        ((j * g + r) * B + digit) % d // g
                                    )
                                    curr_count = (
                                        res >> (new_j * num_bits)
                                    ) % (1 << num_bits)
                                    new_count = (
                                        suffix_mod_counts >> (j * num_bits)
                                    ) % (1 << num_bits)
                                    res += (
                                        min(new_count, num_bits - curr_count)
                                        << (new_j * num_bits)
                                    )

                                res += (digit % d % g) << (
                                    d // g * num_bits
                                )
                                new_suffix_mod_counts_cache[digit][
                                    suffix_mod_counts
                                ] = res

                            new_suffix_mod_counts = (
                                new_suffix_mod_counts_cache[digit][
                                    suffix_mod_counts
                                ]
                            )

                            # Check if we have a new substring divisible by d
                            check_val = new_suffix_mod_counts >> (
                                d // g * num_bits
                            )
                            new_num_substrings = num_substrings + (
                                1
                                if check_val == 0
                                and new_suffix_mod_counts % (1 << num_bits) > 0
                                else 0
                            )

                            if new_num_substrings <= 1:
                                new_dp[new_suffix_mod_counts][
                                    new_num_substrings
                                ] += dp[suffix_mod_counts][num_substrings]

            dp = new_dp

        # Sum up all states with exactly one substring divisible by d
        for suffix_mod_counts in range(max_suffix_mod_counts):
            ans += dp[suffix_mod_counts][1]

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
