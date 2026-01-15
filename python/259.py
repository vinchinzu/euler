"""Project Euler Problem 259: Reachable Numbers.

Find all positive integers that can be expressed by concatenating and
performing the four usual binary operations on the digits 1 to 9, in that
order.
"""

from __future__ import annotations

from fractions import Fraction
from typing import Dict, Set, Tuple


def solve() -> int:
    """Solve Problem 259."""
    B = 10
    dp: Dict[Tuple[int, int], Set[Fraction]] = {}

    # Initialize single digits
    for i in range(B - 1):
        dp[(i, i + 1)] = {Fraction(i + 1)}

    # Build up longer sequences
    for length in range(2, B):
        for start in range(B - length):
            end = start + length
            # Concatenated number
            num_str = "".join(str(d + 1) for d in range(start, end))
            nums: Set[Fraction] = {Fraction(int(num_str))}

            # Try all splits
            for left in range(1, length):
                mid = start + left
                for num1 in dp.get((start, mid), set()):
                    for num2 in dp.get((mid, end), set()):
                        nums.add(num1 + num2)
                        nums.add(num1 - num2)
                        nums.add(num1 * num2)
                        if num2 != 0:
                            nums.add(num1 / num2)

            dp[(start, end)] = nums

    # Sum all positive integers
    ans = 0
    for num in dp.get((0, B - 1), set()):
        if num.denominator == 1 and num.numerator > 0:
            ans += num.numerator

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
