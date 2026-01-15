"""Project Euler Problem 266: Pseudo Square Root.

Let n be the product of all primes less than N. Find the largest factor
of n that does not exceed √n.
"""

from __future__ import annotations

from math import isqrt, prod as math_prod
from typing import List


def sieve(limit: int) -> List[int]:
    """Generate all primes up to limit."""
    if limit < 2:
        return []
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, isqrt(limit) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(limit + 1) if is_prime[i]]


def solve() -> int:
    """Solve Problem 266."""
    N = 190
    M = 10**16

    primes_list = sieve(N)
    mid = len(primes_list) // 2
    A = primes_list[:mid]
    B = primes_list[mid:]

    def prod_subset(nums: List[int], subset: int) -> float:
        """Product of subset."""
        result = 1.0
        for i, num in enumerate(nums):
            if subset & (1 << i):
                result *= num
        return result

    # Build PA map
    PA: dict[float, int] = {}
    for subset in range(1 << len(A)):
        PA[prod_subset(A, subset)] = subset

    # Compute sqrt of full product
    full_prod = prod_subset(primes_list, (1 << len(primes_list)) - 1)
    sqrt_val = full_prod**0.5

    best = 0.0
    ans = 1

    # Search through B subsets
    for subset_b in range(1 << len(B)):
        prod_b = prod_subset(B, subset_b)
        target = sqrt_val / prod_b

        # Find largest key < target
        best_match = None
        for key in PA:
            if key < target:
                if best_match is None or key > best_match:
                    best_match = key

        if best_match is not None:
            cand = prod_b * best_match
            if cand > best:
                best = cand
                filter_val = PA[best_match] + (subset_b << len(A))
                result = 1
                for i, p in enumerate(primes_list):
                    if filter_val & (1 << i):
                        result = (result * p) % M
                ans = result

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
