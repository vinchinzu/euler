"""Project Euler Problem 284: Steady Squares.

Find the sum of the digits of all n-digit steady squares in base 14 for
1 <= n <= 10000, where a steady square is a positive integer (no leading zeros)
whose square ends with the same digits as the number itself in base 14.

A steady square k with n digits satisfies k^2 ≡ k (mod 14^n), equivalently
k(k-1) ≡ 0 (mod 14^n). Since 14^n = 2^n * 7^n with gcd(2^n, 7^n) = 1,
by CRT the solutions mod 14^n are:
  - 0 (trivial, excluded since no leading zeros unless n=0)
  - 1 (trivial, 1-digit steady square)
  - a_n ≡ 0 (mod 2^n), ≡ 1 (mod 7^n)
  - b_n ≡ 1 (mod 2^n), ≡ 0 (mod 7^n)

Key insight: a_n = a_N mod 14^n (suffix property). So the base-14 digits of a_N
contain all shorter steady squares as suffixes. An n-digit suffix is a valid
steady square iff its leading digit (d[n-1]) is non-zero.

For each digit d[i], it contributes to all valid n-digit steady squares with
n >= i+1. The total contribution is d[i] * |{j >= i : d[j] != 0}|, which
can be computed efficiently with a suffix count of non-zero digits.
"""

from __future__ import annotations


def solve() -> str:
    """Solve Problem 284 and return the answer in base 14."""
    N = 10000
    p2 = 2 ** N
    p7 = 7 ** N
    mod = p2 * p7  # 14^N

    # Two non-trivial steady square sequences via CRT
    a = p2 * pow(p2, -1, p7) % mod
    b = p7 * pow(p7, -1, p2) % mod

    def get_digits(k: int) -> list[int]:
        """Extract base-14 digits, least significant first."""
        digits = []
        for _ in range(N):
            digits.append(k % 14)
            k //= 14
        return digits

    def digit_sum_contribution(digits: list[int]) -> int:
        """Compute the total digit-sum contribution across all valid suffixes.

        For each valid n-digit suffix (where d[n-1] != 0), we add its digit sum.
        Rearranging, d[i] contributes d[i] * (count of non-zero d[j] for j >= i).
        """
        nonzero_suffix = [0] * (N + 1)
        for i in range(N - 1, -1, -1):
            nonzero_suffix[i] = nonzero_suffix[i + 1] + (1 if digits[i] != 0 else 0)

        total = 0
        for i in range(N):
            total += digits[i] * nonzero_suffix[i]
        return total

    da = get_digits(a)
    db = get_digits(b)

    result = 1  # Trivial steady square: 1
    result += digit_sum_contribution(da)
    result += digit_sum_contribution(db)

    return to_base14(result)


def to_base14(num: int) -> str:
    """Convert a non-negative integer to its base-14 string representation."""
    if num == 0:
        return "0"
    chars = "0123456789abcd"
    parts: list[str] = []
    while num > 0:
        parts.append(chars[num % 14])
        num //= 14
    return "".join(reversed(parts))


def main() -> None:
    """Main entry point."""
    print(solve())


if __name__ == "__main__":
    main()
