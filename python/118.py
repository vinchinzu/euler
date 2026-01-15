"""Project Euler Problem 118.

Counts sets of prime numbers formed by using each digit 1–9 exactly once.
"""

from itertools import permutations
from typing import Dict, List, Tuple


DIGITS = ['1', '2', '3', '4', '5', '6', '7', '8', '9']
ALL_MASK = (1 << 9) - 1


def is_prime(n: int) -> bool:
    """Check if a number is prime."""
    if n < 2:
        return False
    if n < 4:
        return True
    if n % 2 == 0 or n % 3 == 0:
        return False
    i = 5
    step = 2
    while i * i <= n:
        if n % i == 0:
            return False
        i += step
        step = 6 - step
    return True


def main() -> int:
    """Main function."""
    primes_by_mask: List[List[int]] = [[] for _ in range(1 << 9)]

    for mask in range(1, ALL_MASK + 1):
        digits = []
        for i in range(9):
            if mask & (1 << i) != 0:
                digits.append(DIGITS[i])
        for perm in permutations(digits):
            # multi-digit primes can't end in an even digit or 5
            if len(perm) > 1 and (int(perm[-1]) % 2 == 0 or perm[-1] == '5'):
                continue
            num = int(''.join(perm))
            if is_prime(num):
                primes_by_mask[mask].append(num)
        primes_by_mask[mask].sort()

    memo: Dict[Tuple[int, int], int] = {}

    def dfs(mask: int, last: int) -> int:
        """Depth-first search to count sets."""
        if mask == 0:
            return 1
        key = (mask, last)
        if key in memo:
            return memo[key]
        total = 0
        sub = mask
        while sub > 0:
            for p in primes_by_mask[sub]:
                if p > last:
                    total += dfs(mask ^ sub, p)
            sub = (sub - 1) & mask
        memo[key] = total
        return total

    return dfs(ALL_MASK, 0)


if __name__ == "__main__":
    print(main())
