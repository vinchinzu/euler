"""Project Euler Problem 138: Isosceles triangles.

We are looking for isosceles triangles with base b, side L, such that h = b +/- 1.
"""


def fibonacci(n: int) -> int:
    """Compute F_n (n-th Fibonacci number)."""
    if n == 0:
        return 0
    if n == 1:
        return 1
    a, b = 0, 1
    for _ in range(2, n + 1):
        a, b = b, a + b
    return b


def main() -> int:
    """Main function."""
    # The problem statement's derivation is L = F_{6j+3}/2.
    # We need the first 12 values of L.
    # j ranges from 1 to 12.
    total = 0
    for j in range(1, 13):
        m = 6 * j + 3
        L = fibonacci(m) // 2
        total += L
    return total


if __name__ == "__main__":
    print(main())
