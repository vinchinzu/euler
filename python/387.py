"""Project Euler Problem 387: Strong right truncatable Harshad primes.

This module provides utilities to compute the sum of strong, right truncatable
Harshad primes below a given limit. It is an idiomatic Python 3.12 translation
of the given Ruby implementation, with type hints and small, focused functions.

The known answer for limit=10**14 is 696067597313468.
"""

from __future__ import annotations

from collections import deque
from typing import Deque, List, Set, Tuple

LIMIT: int = 10**14
MAX_M_DIGITS: int = 13


def is_prime(n: int) -> bool:
    """Return True if n is a prime number using a 6k ± 1 trial division test."""

    if n < 2:
        return False
    if n in (2, 3):
        return True
    if n % 2 == 0 or n % 3 == 0:
        return False

    i: int = 5
    step: int = 2
    while i * i <= n:
        if n % i == 0:
            return False
        i += step
        step = 6 - step
    return True


def generate_strong_right_truncatable_harshad_prime_sum(limit: int = LIMIT) -> int:
    """Compute sum of strong right truncatable Harshad primes below ``limit``.

    A strong right truncatable Harshad prime p satisfies:
    - p is prime.
    - Let m be p without its last digit; m is a right truncatable Harshad number.
    - m is strong: (m / digit_sum(m)) is prime.
    """

    total_sum: int = 0

    # Queue holds tuples of (current_number, digit_sum(current_number)). We start
    # from 1..9 as base Harshad candidates.
    queue: Deque[Tuple[int, int]] = deque((d, d) for d in range(1, 10))
    visited: Set[int] = set()

    while queue:
        num, digit_sum_num = queue.popleft()

        if num in visited:
            continue
        visited.add(num)

        # If num is a strong Harshad number, try appending one digit and check
        # if the resulting number is a prime below the limit.
        if (
            digit_sum_num > 0
            and num % digit_sum_num == 0
            and is_prime(num // digit_sum_num)
        ):
            for d in range(10):
                candidate: int = num * 10 + d
                if candidate < limit and is_prime(candidate):
                    total_sum += candidate

        # Extend right truncatable Harshad numbers, ensuring room for one digit
        # to remain below the limit (hence MAX_M_DIGITS usage).
        if num < 10**MAX_M_DIGITS:
            for d in range(10):
                new_num: int = num * 10 + d
                new_digit_sum: int = digit_sum_num + d

                if new_digit_sum > 0 and new_num % new_digit_sum == 0:
                    queue.append((new_num, new_digit_sum))

    return total_sum


def verify_small_case() -> int:
    """Verify implementation for limit < 10000 (known sum is 90619).

    This replicates the Ruby verification procedure and should return 90619.
    """

    verification_primes: List[int] = []

    for n in range(10, 10_000):
        if not is_prime(n):
            continue

        m: int = n // 10
        digit_sum_m: int = sum(int(ch) for ch in str(m))

        if digit_sum_m == 0:
            continue
        if m % digit_sum_m != 0:
            continue
        if not is_prime(m // digit_sum_m):
            continue

        # Check right-truncatable Harshad property for m.
        current: int = m
        is_right_truncatable: bool = True

        while current > 0:
            digit_sum_current: int = sum(int(ch) for ch in str(current))
            if digit_sum_current == 0 or current % digit_sum_current != 0:
                is_right_truncatable = False
                break
            current //= 10

        if is_right_truncatable:
            verification_primes.append(n)

    return sum(verification_primes)


def main() -> None:
    """Run verification and compute the result up to LIMIT.

    Prints the verification sum (must be 90619) and the final result.
    """

    verification_sum = verify_small_case()
    if verification_sum != 90_619:
        raise RuntimeError("Verification failed!")

    result = generate_strong_right_truncatable_harshad_prime_sum(LIMIT)
    print(result)


if __name__ == "__main__":
    main()
