"""Project Euler Problem 140: Modified Fibonacci golden nuggets.

A_G(x) = x(1+3x)/(1-x-x^2) = N
For rational x, we need 5N^2 + 14N + 1 = k^2 for some integer k
This transforms to the Pell equation X^2 - 5Y^2 = 44 where X = 5N + 7, Y = k
So N = (X - 7)/5, and we need X ≡ 2 (mod 5) for N to be a positive integer
"""

from typing import List, Tuple


def main() -> int:
    """Main function."""
    # All base solutions to X^2 - 5Y^2 = 44
    base_solutions: List[Tuple[int, int]] = [
        (7, 1), (8, 2), (13, 5), (17, 7), (32, 14), (43, 19), (83, 37)
    ]

    all_nuggets: List[int] = []

    for x0, y0 in base_solutions:
        # For the recurrence X_{n+1} = 18*X_n - X_{n-1}, we need X_{-1}
        # We find X_{-1} by working backwards: X_{-1} = 18*X_0 - X_1
        # To find X_1, we use the transformation with fundamental solution (9,4):
        # X_1 = 9*X_0 + 20*Y_0, Y_1 = 4*X_0 + 9*Y_0
        x1 = 9 * x0 + 20 * y0
        y1 = 4 * x0 + 9 * y0
        x_minus1 = 18 * x0 - x1
        y_minus1 = 18 * y0 - y1

        # Check if initial solution gives a nugget
        if x0 > 7 and (x0 - 7) % 5 == 0:
            n = (x0 - 7) // 5
            all_nuggets.append(n)

        # Generate sequence using recurrence
        x_prev, x_curr = x_minus1, x0
        y_prev, y_curr = y_minus1, y0

        # Generate enough terms to ensure we get at least 30 nuggets total
        for _ in range(15):
            x_next = 18 * x_curr - x_prev
            y_next = 18 * y_curr - y_prev

            if x_next > 7 and (x_next - 7) % 5 == 0:
                n = (x_next - 7) // 5
                all_nuggets.append(n)

            x_prev, x_curr = x_curr, x_next
            y_prev, y_curr = y_curr, y_next

    # Remove duplicates, sort and take first 30
    all_nuggets = sorted(set(all_nuggets))
    return sum(all_nuggets[:30])


if __name__ == "__main__":
    print(main())
