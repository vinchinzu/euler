"""Project Euler Problem 137: Fibonacci golden nuggets.

The problem asks for the 15th "golden nugget".
A golden nugget is a positive integer N such that A_F(x) = N for some rational x.
A_F(x) is the generating function for Fibonacci numbers, A_F(x) = x / (1 - x - x^2).
"""


def lucas_number(n: int) -> int:
    """Compute n-th Lucas number."""
    if n == 0:
        return 2
    if n == 1:
        return 1
    a, b = 2, 1
    for _ in range(2, n + 1):
        a, b = b, a + b
    return b


def main() -> int:
    """Main function."""
    # For the 15th nugget, we use m=15.
    # The index for the Lucas number is 4*15 + 1 = 60 + 1 = 61.
    # The nugget N_15 is (L_61 - 1) / 5.
    l_target_index = 61
    l_val = lucas_number(l_target_index)
    n_15 = (l_val - 1) // 5
    return n_15


if __name__ == "__main__":
    print(main())
