"""
Project Euler Problem 862
https://projecteuler.net/problem=862

For a positive integer n define T(n) to be the number of strictly larger
integers which can be formed by permuting the digits of n.

Leading zeros are not allowed and so for n = 2302 the total list of
permutations would be:
2023,2032,2203,2230,2302,2320,3022,3202,3220
giving T(2302)=4.

Further define S(k) to be the sum of T(n) for all k-digit numbers n.
You are given S(3) = 1701.

Find S(12).
"""

from typing import List
import sys


def precompute_factorials(k: int) -> List[int]:
    """Precompute factorials up to k."""
    fact: List[int] = [1] * (k + 1)
    for i in range(1, k + 1):
        fact[i] = fact[i - 1] * i
    return fact


def dfs(pos: int, rem: int, counts: List[int], fact: List[int],
        k: int, total: List[int]) -> None:
    """
    Recursively enumerate frequencies for digits pos to 9.
    pos=0..8: digits 0-8, pos=9: sets counts[9]=rem.
    """
    if pos == 9:
        counts[9] = rem
        denom: int = 1
        for c in counts:
            denom *= fact[c]
        total_perms: int = fact[k] // denom

        c0: int = counts[0]
        if c0 == 0:
            num_valid: int = total_perms
        else:
            # denom2 = prod(fact[counts[i]] for i!=0) * fact[c0-1]
            denom2: int = (denom // fact[c0]) * fact[c0 - 1]
            zero_first: int = fact[k - 1] // denom2
            num_valid = total_perms - zero_first

        # C(num_valid, 2) since each unordered pair contributes once
        if num_valid > 1:
            total[0] += num_valid * (num_valid - 1) // 2
        return

    # Try freq 0 to rem for this digit
    for v in range(rem + 1):
        counts[pos] = v
        dfs(pos + 1, rem - v, counts, fact, k, total)


def solve(k: int = 12) -> int:
    """Compute S(k)."""
    fact = precompute_factorials(k)
    total = [0]
    counts = [0] * 10
    dfs(0, k, counts, fact, k, total)
    return total[0]


# Verification helpers (disabled for production run)
# def compute_t(n: int) -> int:
#     """Compute T(n) directly for small n."""
#     from itertools import permutations
#     digits = [int(d) for d in str(n)]
#     perms_str = {''.join(p) for p in permutations(digits)}
#     valid_perms = [int(p) for p in perms_str if p[0] != '0']
#     return sum(p > n for p in valid_perms)
#
# # Tests:
# # assert compute_t(2302) == 4
# # assert solve(1) == 0
# # assert solve(2) == 36  # Manual: C(9,2)=36 for distinct non-zero pairs
# # assert solve(3) == 1701
# # Investigate further: Verify S(2)==36 by manual count or small enum.


if __name__ == "__main__":
    k = int(sys.argv[1]) if len(sys.argv) > 1 else 12
    print(solve(k))