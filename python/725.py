"""Project Euler Problem 725: Digit Sum Numbers.

Find the sum of all DS-numbers, numbers where one digit is the sum of all
other digits, with N digits or less.

Given a digit d at place value k, by balls and bins there are nCr(d+N-2,d)
ways to choose the remaining N-1 digits to add up to d. This means that the
sum of all digits at place value k for numbers with largest digit d is
2d*nCr(d+N-2,d).

However, we've double counted numbers that only contain two non-zero digits
which are the same. For each place value k and largest digit d, there are
N-1 of these.

So the total is, in base B,
S(N) = (B^N-1)/(B-1) (n-1)(2nCr(B+N-2,B-2) - nCr(B,2)).
"""

from __future__ import annotations

from math import comb


def solve() -> int:
    """Solve Problem 725."""
    n = 2020
    m = 10**16
    b = 10

    # Use Python's exact integer arithmetic, then take mod at the end
    term1 = (pow(b, n) - 1) // (b - 1)
    term2 = n - 1
    term3 = 2 * comb(b + n - 2, b - 2) - comb(b, 2)

    ans = (term1 * term2 * term3) % m
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
