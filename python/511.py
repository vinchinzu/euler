"""Project Euler Problem 511: Sequences with Divisibility Constraints.

Find the number of sequences of N integers such that each term is divisible by
N, and the sum of all terms and N is divisible by K.

Let T_n[i] be the number of sequences of n terms, all divisible by N, that sum
to i (mod K). We can compute T_n by taking T_{n-1} and adding the transitions
for each divisor of N.

To efficiently compute T_n for large n, we note that T_{2n} is the convolution
of T_n with itself, and can use repeated doubling. Convolution can be
implemented with polynomial multiplication, for which there are fast
algorithms, by concatenating T_n with itself to handle the terms in the
convolution that are wrapped around.

The final answer is T_N[-N].
"""

from __future__ import annotations

from math import isqrt
from typing import List


def all_divisors(n: int) -> List[int]:
    """Get all divisors of n."""
    divisors = []
    limit = isqrt(n)
    for i in range(1, limit + 1):
        if n % i == 0:
            divisors.append(i)
            if i * i != n:
                divisors.append(n // i)
    return sorted(divisors)


def imod(a: int, m: int) -> int:
    """Integer modulo (handles negative)."""
    return ((a % m) + m) % m


def polynomial_multiply(p1: List[int], p2: List[int], mod: int) -> List[int]:
    """Multiply two polynomials modulo mod."""
    deg1 = len(p1)
    deg2 = len(p2)
    result = [0] * (deg1 + deg2 - 1)
    for i in range(deg1):
        for j in range(deg2):
            result[i + j] = (result[i + j] + p1[i] * p2[j]) % mod
    return result


def combine(
    num_transitions1: List[int], num_transitions2: List[int], K: int, M: int
) -> List[int]:
    """Combine two transition arrays via convolution."""
    # Concatenate each array with itself to handle wrap-around
    p1 = num_transitions1 + num_transitions1
    p2 = num_transitions2 + num_transitions2
    
    # Multiply polynomials
    p = polynomial_multiply(p1, p2, M)
    
    # Extract wrapped-around convolution
    num_transitions = [0] * K
    for i in range(K):
        num_transitions[i] = (p[i + K] - p[i]) % M
    
    return num_transitions


def num_transitions(n: int, all_divisors: List[int], K: int, M: int) -> List[int]:
    """Compute transition array for n sequences."""
    if n == 1:
        num_transitions_arr = [0] * K
        for d in all_divisors:
            num_transitions_arr[imod(d, K)] = (num_transitions_arr[imod(d, K)] + 1) % M
        return num_transitions_arr
    
    res = num_transitions(n // 2, all_divisors, K, M)
    res = combine(res, res, K, M)
    if n % 2 == 1:
        res = combine(res, num_transitions(1, all_divisors, K, M), K, M)
    return res


def solve() -> int:
    """Solve Problem 511."""
    N = 1234567898765
    K = 4321
    M = 10**9
    
    all_divisors_list = all_divisors(N)
    transitions = num_transitions(N, all_divisors_list, K, M)
    ans = transitions[imod(-N, K)]
    
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
