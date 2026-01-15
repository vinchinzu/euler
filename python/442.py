"""Project Euler Problem 442: Eleven-free integers.

Find the Nth positive "eleven-free" integer, i.e. an integer that does not
contain any power of 11 (except possibly 1) as a substring.

We compute the answer digit-by-digit, repeatedly appending the first digit that
would prevent the total number of eleven-free integers from exceeding N. The
only optimization is that when computing the number of eleven-free integers of
a particular length that start with a given prefix, only the end of that prefix
that matches any power of 11 is relevant. Memoizing the number for prefixes
that have this same "relevant prefix" allows us to quickly compute the answer.
"""

from __future__ import annotations

from functools import lru_cache
from math import log
from typing import Dict, Tuple


def num_eleven_frees(
    prefix: str, num_remaining_digits: int, K: int, B: int, N: int, cache: Dict[Tuple[str, int], int]
) -> int:
    """Count eleven-free integers with given prefix and remaining digits."""
    # Find relevant prefix (longest suffix of prefix that matches a power of K)
    relevant_prefix = ""
    pow_k = 1
    while pow_k <= N:
        pow_k_str = str(pow_k)
        if pow_k != 1 and prefix.endswith(pow_k_str):
            return 0  # Contains a power of K as substring
        # Check all suffixes of pow_k_str
        temp = pow_k_str
        while len(temp) > len(relevant_prefix):
            temp = temp[:-1]
            if prefix.endswith(temp):
                relevant_prefix = temp
        pow_k *= K
    
    if num_remaining_digits == 0:
        return 1
    
    key = (relevant_prefix, num_remaining_digits)
    if key in cache:
        return cache[key]
    
    res = 0
    for d in range(B):
        new_prefix = prefix + str(d)
        res += num_eleven_frees(new_prefix, num_remaining_digits - 1, K, B, N, cache)
    
    cache[key] = res
    return res


def solve() -> int:
    """Solve Problem 442."""
    N = 10**18
    K = 11
    B = 10
    L = int(log(2**63 - 1) / log(B))  # Maximum digits
    
    ans = 0
    n = N
    cache: Dict[Tuple[str, int], int] = {}
    
    for num_remaining_digits in range(L, -1, -1):
        for d in range(B):
            prefix = str(ans * B + d)
            num_eleven_frees_count = num_eleven_frees(
                prefix, num_remaining_digits, K, B, N, cache
            )
            if num_eleven_frees_count > n:
                ans = ans * B + d
                break
            n -= num_eleven_frees_count
    
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
