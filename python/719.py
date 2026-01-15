"""Project Euler Problem 719: Number Splitting.

An S-number n is a perfect square whose decimal representation can be split
into 2 or more numbers that add up to √n. Find the sum of all S-numbers up to
N.

For each n, we brute force all possible ways to split up n. As an
optimization, we can exit if the current sum is too large or too small. As a
further optimization, the sum of the split numbers is invariant (mod 9), so
an S-number must be congruent to its square root (mod 9).
"""

from __future__ import annotations

from math import isqrt


def sq(n: int) -> int:
    """Square of n."""
    return n * n


def sq_mod(n: int, mod: int) -> int:
    """Square of n modulo mod."""
    return (n * n) % mod


def can_make(target: int, digits: int, b: int = 10) -> bool:
    """Check if target can be made by splitting digits."""
    if target < 0 or digits < target:
        return False
    if digits == 0:
        return target == 0
    can_make_result = False
    pow_val = 1
    while pow_val <= digits:
        can_make_result = can_make_result or can_make(
            target - digits // pow_val, digits % pow_val, b
        )
        pow_val *= b
    return can_make_result


def solve() -> int:
    """Solve Problem 719."""
    n = 10**12
    b = 10
    ans = 0

    max_i = isqrt(n)
    for i in range(2, max_i + 1):
        i_sq = sq(i)
        if i_sq > n:
            break
        # Check mod 9 condition
        if i % (b - 1) == sq_mod(i, b - 1):
            if can_make(i, i_sq, b):
                ans += i_sq

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
