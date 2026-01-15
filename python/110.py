"""Project Euler Problem 110.

Find the smallest n with more than 4,000,000 solutions to 1/x + 1/y = 1/n.
"""

import sys
from typing import List


PRIMES = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47]
MAX_EXPONENT = 20


def main(threshold: int = 4_000_000) -> int:
    """Find the smallest n with more than threshold solutions."""
    if threshold <= 0:
        raise ValueError('Threshold must be positive')

    target_divisors = 2 * threshold - 1
    best = [float('inf')]

    def search(idx: int, limit_exp: int, current_n: int, divisor_count: int) -> None:
        """Recursive search for optimal n."""
        if divisor_count > target_divisors:
            if current_n < best[0]:
                best[0] = current_n
            return

        if idx >= len(PRIMES):
            return

        prime = PRIMES[idx]
        value = current_n * prime
        exp = 1

        while exp <= limit_exp:
            if value >= best[0]:
                break
            new_divisors = divisor_count * (2 * exp + 1)
            search(idx + 1, exp, value, new_divisors)
            exp += 1
            value *= prime

    search(0, MAX_EXPONENT, 1, 1)
    return int(best[0])


if __name__ == "__main__":
    threshold = int(sys.argv[1]) if len(sys.argv) > 1 else 4_000_000
    print(main(threshold))
