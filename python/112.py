"""Project Euler Problem 112: Bouncy numbers."""

from math import gcd
from typing import List


class Euler112:
    """Solver for Problem 112."""

    def __init__(self, target_percentage: int) -> None:
        """Initialize with target percentage."""
        self.target_percentage = target_percentage  # e.g., 99

        # Calculate the denominator for the check optimization.
        # If target_percentage/100 = num/den (simplified), current_number must be a multiple of den.
        common_divisor = gcd(self.target_percentage, 100)
        self.check_denominator = 100 // common_divisor

    def get_digits(self, n: int) -> List[int]:
        """Efficiently get digits. For n=0, this would be [0].
        
        Problem deals with positive integers. Smallest is 1.
        """
        if n < 10:  # Single digit numbers handled by is_increasing/is_decreasing
            return [n]

        digits = []
        while n > 0:
            digits.insert(0, n % 10)
            n //= 10
        return digits

    def is_increasing(self, n_digits: List[int]) -> bool:
        """Check if digits are increasing."""
        if len(n_digits) <= 1:  # Single digit numbers are considered increasing
            return True
        for i in range(len(n_digits) - 1):
            if n_digits[i] > n_digits[i + 1]:
                return False
        return True

    def is_decreasing(self, n_digits: List[int]) -> bool:
        """Check if digits are decreasing."""
        if len(n_digits) <= 1:  # Single digit numbers are considered decreasing
            return True
        for i in range(len(n_digits) - 1):
            if n_digits[i] < n_digits[i + 1]:
                return False
        return True

    def is_bouncy(self, n: int) -> bool:
        """Check if a number is bouncy.
        
        Numbers less than 100 are never bouncy by problem's implication,
        but definitions handle them:
        e.g., 55 is both increasing and decreasing, so not bouncy.
        12 is increasing. 21 is decreasing.
        This method correctly identifies them as non-bouncy.
        """
        n_digits = self.get_digits(n)  # Use the math-based get_digits

        increasing = self.is_increasing(n_digits)
        decreasing = self.is_decreasing(n_digits)

        # A number is bouncy if it is NOT increasing AND NOT decreasing.
        return not increasing and not decreasing

    def solve(self) -> int:
        """Solve the problem."""
        bouncy_count = 0
        current_number = 0  # Start from 0, loop will increment to 1 first

        while True:
            current_number += 1

            if self.is_bouncy(current_number):
                bouncy_count += 1

            # Optimized check:
            # Only evaluate the proportion condition when current_number is a multiple of self.check_denominator.
            # For 99%, self.check_denominator is 100.
            # For 50%, self.check_denominator is 2.
            # For 90%, self.check_denominator is 10.
            if current_number % self.check_denominator == 0:
                # Proportion check: bouncy_count / current_number == self.target_percentage / 100
                # Use cross-multiplication to avoid floating point:
                # bouncy_count * 100 == self.target_percentage * current_number
                if bouncy_count * 100 == self.target_percentage * current_number:
                    # This condition implies that if self.target_percentage > 0, bouncy_count must be > 0.
                    # If self.target_percentage == 0, this means bouncy_count must be 0.

                    # Handle target_percentage = 0 explicitly if needed, though problem implies > 0.
                    if self.target_percentage == 0 and bouncy_count != 0:
                        continue  # Keep searching if we want 0% but have bouncy numbers

                    return current_number


def main() -> int:
    """Main function."""
    # Problem 112: Find the least number for which the proportion of bouncy numbers is exactly 99%.
    solver = Euler112(99)
    return solver.solve()


if __name__ == "__main__":
    print(main())
