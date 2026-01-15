"""Project Euler Problem 749: Near Power Sums.

Find the sum of the near power sums (the sum of the kth powers of its digits
is 1 away from the number itself, for some k) with up to N digits.

We brute force over all combinations of digits, with the following two
optimizations:
- k must be even. Otherwise, Σ (d_i)^k ≡ Σ d_i ≡ Σ (10^i) d_i (mod 3), so
  they cannot be 1 apart.
- The total number of digits cannot be more than 1 greater than k, because
  then it is not possible for even a number with all 9s to add up to a large
  enough sum.
"""

from __future__ import annotations


def imod(a: int, m: int) -> int:
    """Modulo operation: a mod m."""
    return a % m


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Fast exponentiation modulo mod."""
    result = 1
    base = base % mod
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def nth_pows(base: int, k: int, max_val: int) -> list[int]:
    """Precompute kth powers of digits."""
    result = []
    for d in range(base):
        pow_val = d**k
        if pow_val > max_val:
            break
        result.append(pow_val)
    return result


def solve() -> int:
    """Solve Problem 740."""
    n = 16
    b = 10
    nums: set[int] = set()

    def helper(d: int, num_digits: int, sum_powers: int, k: int, pows: list[int]) -> None:
        """Recursive helper."""
        for candidate in [sum_powers - 1, sum_powers + 1]:
            if candidate > 0:
                actual_sum_powers = 0
                num = candidate
                while num > 0:
                    actual_sum_powers += pows[imod(num, b)]
                    num //= b
                if actual_sum_powers == sum_powers:
                    nums.add(candidate)

        if num_digits < n and num_digits <= k + 1:
            for new_d in range(d, b):
                new_sum = sum_powers + pows[new_d]
                if new_sum < b**n:
                    helper(new_d, num_digits + 1, new_sum, k, pows)

    for k in range(2, n + 2, 2):
        pows = nth_pows(b, k, b**n)
        helper(1, 0, 0, k, pows)

    return sum(nums)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
