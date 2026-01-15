"""Project Euler Problem 168: Number Rotations."""

from typing import Set
import math

MAX_DIGITS = 100
MODULO = 100_000


def numbers_with_rotation_property() -> Set[int]:
    """Compute integers n (10 < n < 10^100) with rotation property."""
    results: Set[int] = set()
    pow10: list[int] = [1] * (MAX_DIGITS + 1)
    for i in range(1, MAX_DIGITS + 1):
        pow10[i] = pow10[i - 1] * 10

    for k in range(1, 10):
        multiplier_mod = 10 * k - 1

        for digits in range(2, MAX_DIGITS + 1):
            base_power = pow10[digits - 1]
            difference = base_power - k
            if difference <= 0:
                continue

            gcd_val = math.gcd(multiplier_mod, difference)
            reduced_mod = multiplier_mod // gcd_val
            reduced_diff = difference // gcd_val

            for last_digit in range(1, 10):
                if (last_digit * reduced_diff) % reduced_mod != 0:
                    continue

                prefix_val = last_digit * difference // multiplier_mod
                prefix_str = str(prefix_val)
                if len(prefix_str) != digits - 1:
                    continue

                number_str = str(last_digit) + prefix_str
                number = int(number_str)
                rotated_str = prefix_str + str(last_digit)
                rotated = int(rotated_str)
                if rotated % number != 0:
                    continue

                results.add(number)

    return results


def main() -> int:
    """Main function."""
    numbers = numbers_with_rotation_property()
    # Sum with modulo at each step to avoid overflow
    sum_mod = 0
    for num in numbers:
        sum_mod = (sum_mod + num) % MODULO
    return sum_mod


if __name__ == "__main__":
    print(main())
