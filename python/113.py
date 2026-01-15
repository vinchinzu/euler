"""Project Euler Problem 113: Non-bouncy numbers.

A number is non-bouncy if it is increasing or decreasing.
We need to find the count of such numbers below 10^100 (i.e., up to 100 digits).
"""


def combinations(n: int, k: int) -> int:
    """Calculate combinations C(n, k)."""
    if k == 0 or k == n:  # Base cases: C(n,0)=1, C(n,n)=1
        return 1
    if k < 0 or k > n:  # Invalid k
        return 0

    # C(n, k) == C(n, n - k). Choose smaller k for efficiency.
    if k > n // 2:
        k = n - k

    res = 1
    for i in range(1, k + 1):
        # (res * (n - i + 1)) / i ensures intermediate products are divisible
        # and maintains integer precision.
        res = res * (n - i + 1) // i
    return res


def main() -> int:
    """Main function."""
    # N is the maximum number of digits (100 for numbers below 10^100).
    N = 100

    # The formula for the number of positive non-bouncy integers with up to N digits is:
    # Count = C(N+10, 10) + C(N+9, 9) - 10*N - 2

    # Calculate C(N+10, 10), which is C(110, 10) for N=100
    term1 = combinations(N + 10, 10)  # Same as C(N+10, N)

    # Calculate C(N+9, 9), which is C(109, 9) for N=100
    term2 = combinations(N + 9, 9)  # Same as C(N+9, N)

    # Constant term related to numbers with all same digits and adjustments
    term3 = 10 * N

    term4 = 2

    # Total non-bouncy numbers
    count = term1 + term2 - term3 - term4

    return count


if __name__ == "__main__":
    print(main())
