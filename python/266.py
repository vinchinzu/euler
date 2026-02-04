"""Project Euler Problem 266: Pseudo Square Root.

Let n be the product of all primes less than N. Find the largest factor
of n that does not exceed sqrt(n).
"""

from __future__ import annotations

import math
from bisect import bisect_left
from math import isqrt
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


def build_log_products(primes: List[int]) -> list[tuple[float, int]]:
    """Build all subset log-products using Gray code-like incremental approach."""
    n = len(primes)
    size = 1 << n
    logs = [math.log(p) for p in primes]
    # Use dynamic programming: products[subset] = log of product of elements in subset
    result = [0.0] * size
    for i in range(n):
        for s in range(1 << i):
            result[s | (1 << i)] = result[s] + logs[i]
    return [(result[s], s) for s in range(size)]


def solve() -> int:
    """Solve Problem 266."""
    N = 190
    M = 10**16

    primes_list = sieve(N)
    mid = len(primes_list) // 2
    A = primes_list[:mid]
    B = primes_list[mid:]

    # Build sorted (log_product, bitmask) for A subsets
    pa_list = build_log_products(A)
    pa_list.sort()
    pa_keys = [x[0] for x in pa_list]
    pa_vals = [x[1] for x in pa_list]

    # Build log products for B subsets
    nb = len(B)
    b_logs = [0.0] * (1 << nb)
    blog = [math.log(p) for p in B]
    for i in range(nb):
        for s in range(1 << i):
            b_logs[s | (1 << i)] = b_logs[s] + blog[i]

    # Compute log(sqrt(n))
    log_sqrt = sum(math.log(p) for p in primes_list) / 2.0

    best_log = -1.0
    best_filter = 0
    na = len(A)

    # Search through B subsets with binary search into sorted A subsets
    for subset_b in range(1 << nb):
        target = log_sqrt - b_logs[subset_b]

        idx = bisect_left(pa_keys, target)
        if idx > 0:
            idx -= 1
            cand_log = b_logs[subset_b] + pa_keys[idx]
            if cand_log > best_log:
                best_log = cand_log
                best_filter = pa_vals[idx] | (subset_b << na)

    # Compute answer mod M
    ans = 1
    for i, p in enumerate(primes_list):
        if best_filter & (1 << i):
            ans = (ans * p) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
