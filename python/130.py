"""Project Euler Problem 130.

Sum the first 25 composite values n (coprime to 10) such that A(n) | (n - 1),
where A(n) is the least k with the repunit R(k) divisible by n.
"""

import math

TARGET_COUNT = 25


def repunit_period(n: int) -> int:
    """Calculate repunit period."""
    remainder = 1 % n
    length = 1
    while remainder != 0:
        remainder = (remainder * 10 + 1) % n
        length += 1
    return length


def is_composite(n: int) -> bool:
    """Check if n is composite."""
    if n < 4 or n % 2 == 0 or (n % 5) == 0:
        return False
    limit = int(math.sqrt(n))
    i = 3
    while i <= limit:
        if (n % i) == 0:
            return True
        i += 2
    return False


def main() -> int:
    """Main function."""
    found = 0
    total_sum = 0
    n = 1

    while found < TARGET_COUNT:
        n += 1
        if n % 2 == 0 or (n % 5) == 0:
            continue
        if not is_composite(n):
            continue

        period = repunit_period(n)
        if ((n - 1) % period) != 0:
            continue

        total_sum += n
        found += 1

    return total_sum


if __name__ == "__main__":
    print(main())
