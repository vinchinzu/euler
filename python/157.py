"""Project Euler Problem 157: Solving the diophantine equation."""

from typing import Dict
import math


def prime_factors(num: int) -> Dict[int, int]:
    """Compute prime factors of a number."""
    factors: Dict[int, int] = {}
    while num % 2 == 0:
        factors[2] = factors.get(2, 0) + 1
        num //= 2
    i = 3
    while i * i <= num:
        while num % i == 0:
            factors[i] = factors.get(i, 0) + 1
            num //= i
        i += 2
    if num > 1:
        factors[num] = factors.get(num, 0) + 1
    return factors


def num_divisors(num: int) -> int:
    """Count number of divisors of a number."""
    if num == 1:
        return 1
    factors = prime_factors(num)
    result = 1
    for exp in factors.values():
        result *= exp + 1
    return result


def main() -> int:
    """Main function."""
    total = 0
    for n in range(1, 10):
        count = 0
        # Case 1: m=1, k=2**a * 5**b
        for a in range(n + 1):
            for b in range(n + 1):
                k = 2 ** a * 5 ** b
                s = 2 ** (n - a) * 5 ** (n - b)
                mk_sum = 1 + k
                s_val = s * mk_sum
                count += num_divisors(s_val)
        # Case 2: m=2**alpha, k=5**beta
        for alpha in range(1, n + 1):
            for beta in range(1, n + 1):
                m = 2 ** alpha
                k = 5 ** beta
                if m > k:
                    continue
                s = 2 ** (n - alpha) * 5 ** (n - beta)
                mk_sum = m + k
                s_val = s * mk_sum
                count += num_divisors(s_val)
        # Case 3: m=5**beta, k=2**alpha
        for beta in range(1, n + 1):
            for alpha in range(1, n + 1):
                m = 5 ** beta
                k = 2 ** alpha
                if m > k:
                    continue
                s = 2 ** (n - alpha) * 5 ** (n - beta)
                mk_sum = m + k
                s_val = s * mk_sum
                count += num_divisors(s_val)
        total += count
    return total


if __name__ == "__main__":
    print(main())
