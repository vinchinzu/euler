"""Project Euler Problem 776: Digit Sum Division.

Find Σ_{n=1}^N n / d(n), where d(n) is the sum of the digits of n.

We sum the terms for each distinct d(n) separately. This requires determining
the sum of all numbers up to n that have digit sum k, which we denote f(n, k).

If we let g(n, k) be the count of all such numbers, then g(n, k) =
Σ_{d=0}^9 g(n', k-d), where n' = ⌊n/10⌋ if d≤n%10 (the rest of the number can
be equal to n%10) and n' = ⌊n/10⌋-1 otherwise (the rest of the number must be
strictly smaller).

Then f(n, k) = Σ_{d=0}^9 10*f(n', k-d) + d*g(n', k-d); the digit d is counted
g times, and the remaining numbers are shifted left by 1 digit, i.e. the sum
is multiplied by 10.
"""

from __future__ import annotations

from functools import lru_cache
from typing import Tuple


@lru_cache(maxsize=None)
def f(n: int, sum_digits: int, B: int = 10) -> Tuple[int, float]:
    """Return (count, sum) of numbers ≤ n with digit sum = sum_digits."""
    if n == 0 and sum_digits == 0:
        return (1, 0.0)
    if n <= 0 or sum_digits < 0:
        return (0, 0.0)

    count = 0
    total_sum = 0.0

    for d in range(B):
        n_prime = n // B - (1 if d > n % B else 0)
        value = f(n_prime, sum_digits - d, B)
        count += value[0]
        total_sum += value[1] * B + d * value[0]

    return (count, total_sum)


def solve() -> float:
    """Solve Problem 776."""
    N = 1234567890123456789
    B = 10
    max_digit_sum = B * len(str(N))

    ans = 0.0
    for sum_digits in range(1, max_digit_sum):
        value = f(N, sum_digits, B)
        if value[0] > 0:
            ans += value[1] / sum_digits

    return ans


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.12e}")
    return result


if __name__ == "__main__":
    main()
