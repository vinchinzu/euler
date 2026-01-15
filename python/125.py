"""Project Euler Problem 125: Palindromic Sums.

The palindromic number 595 is interesting because it can be written as the
sum of consecutive squares: 6^2 + 7^2 + 8^2 + 9^2 + 10^2 + 11^2 + 12^2.

There are exactly eleven palindromes below one-thousand that can be written
as consecutive square sums, and the sum of these palindromes is 4164. Note
that 1 = 0^2 + 1^2 has not been included as this problem is concerned with
the squares of positive integers.

Find the sum of all the numbers less than 10^8 that are both palindromic
and can be written as the sum of consecutive squares.
"""

import math
from typing import Set


class Problem125Solver:
    """Solver for Problem 125."""

    MAX_SUM_LIMIT = 10**8  # Numbers must be less than 10^8

    @staticmethod
    def is_palindrome(num: int) -> bool:
        """Check if a number is a palindrome."""
        num_str = str(num)
        return num_str == num_str[::-1]

    def solve(self) -> int:
        """Solve the problem."""
        # Use a Set to store unique palindromic sums found
        palindromic_sums_found: Set[int] = set()

        # Outer loop for the starting number 'i' of the sequence of squares.
        # The loop for 'i' can stop when i*i + (i+1)*(i+1) >= MAX_SUM_LIMIT,
        # as this is the smallest possible sum of at least two consecutive squares starting with i.
        # 2*i^2 + 2*i + 1 >= MAX_SUM_LIMIT. Approx. i_limit = sqrt(MAX_SUM_LIMIT / 2).
        i_upper_bound = int(math.sqrt(self.MAX_SUM_LIMIT / 2.0))
        # If i_upper_bound is such that i_upper_bound^2 + (i_upper_bound+1)^2 >= MAX_SUM_LIMIT,
        # then i_upper_bound might be one too high or just right.
        # For i = 1 up to this calculated bound.
        for i in range(1, i_upper_bound + 1):
            current_sum_of_squares = i * i  # Start sum with the first square i^2

            # Inner loop for the subsequent number 'j' in the sequence of squares.
            # The sum must include at least two squares, so j starts from i+1.
            j = i + 1
            while True:  # Upper bound for j is loose; break condition is key
                current_sum_of_squares += j * j  # Add the next square j^2

                # If the current sum exceeds or equals MAX_SUM_LIMIT, break this inner loop.
                # No further sums starting with i^2 and ending beyond j^2 will be valid.
                if current_sum_of_squares >= self.MAX_SUM_LIMIT:
                    break

                # Check if the current sum is a palindrome.
                if self.is_palindrome(current_sum_of_squares):
                    palindromic_sums_found.add(current_sum_of_squares)

                j += 1

        # Calculate the sum of all unique palindromic numbers found.
        return sum(palindromic_sums_found)


def main() -> int:
    """Main function."""
    solver = Problem125Solver()
    return solver.solve()


if __name__ == "__main__":
    print(main())
