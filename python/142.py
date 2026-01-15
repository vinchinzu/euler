"""Project Euler Problem 142."""

import math
from typing import Optional


def sqrt_if_square(n: int) -> Optional[int]:
    """Check if a number is a perfect square and return its integer square root if it is, otherwise None."""
    if n < 0:  # Ensure non-negative numbers
        return None
    # Optimization: last digit check for squares (0, 1, 4, 5, 6, 9)
    last_digit = n % 10
    if last_digit not in [0, 1, 4, 5, 6, 9]:
        return None

    sqrt_n = int(math.sqrt(n))
    if sqrt_n * sqrt_n == n:
        return sqrt_n
    return None


def main() -> int:
    """Main function."""
    min_sum_val = float('inf')

    # k5, k6 must be even
    # Based on analysis, k1 (approx sqrt(sum)) for sum=1006193 is 925.
    # k5 for that solution is 756.
    # A limit of 1000 for k5 should be sufficient.
    limit_k5 = 1000

    for k5 in range(2, limit_k5 + 1, 2):  # k5 must be even
        for k6 in range(2, k5, 2):  # k6 must be even, and k6 < k5

            k5_sq = k5 * k5
            k6_sq = k6 * k6

            y_val = (k5_sq + k6_sq) // 2
            z_val = (k5_sq - k6_sq) // 2

            # Find k3, k4 from z_val = (k3^2 - k4^2)/2 => 2*z_val = k3^2 - k4^2
            z2_val = 2 * z_val  # This is k3^2 - k4^2

            p_limit = int(math.sqrt(z2_val))
            for p_factor in range(2, p_limit + 1, 2):  # P must be even
                if z2_val % p_factor != 0:  # P must be a factor of z2_val
                    continue

                q_factor = z2_val // p_factor
                if q_factor % 2 == 1:  # Q must also be even
                    continue
                if p_factor >= q_factor:  # Ensures k4 > 0, as Q > P
                    continue

                k3 = (p_factor + q_factor) // 2
                k4 = (q_factor - p_factor) // 2

                x_val = (k3 * k3 + k4 * k4) // 2

                k1_sq_val = x_val + y_val  # k1_sq_val is k1^2
                k2_sq_val = x_val - y_val  # k2_sq_val is k2^2

                if k2_sq_val <= 0:  # This means x <= y, problem requires x > y
                    continue

                k1 = sqrt_if_square(k1_sq_val)
                if k1 is None:
                    continue

                k2 = sqrt_if_square(k2_sq_val)
                if k2 is None:
                    continue

                # Parity check: k1 and k2 must have the same parity
                if (k1 % 2) != (k2 % 2):
                    continue

                # Check if their common parity matches x_val's parity
                if (k1 % 2) != (x_val % 2):
                    continue

                # All conditions met for x,y,z
                sum_val = x_val + y_val + z_val
                if sum_val < min_sum_val:
                    min_sum_val = sum_val

    return int(min_sum_val)


if __name__ == "__main__":
    print(main())
