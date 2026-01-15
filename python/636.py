"""Project Euler Problem 636: Restricted Factorisations.

Find the number of ways that N! can be written as the product of one natural
number, two squares, three cubes, and four fourth powers, such that all bases
are distinct.

If N! = Π (p_i)^(e_i), then the number of ways to assign the exponents of
each p_i is the number of solutions to a+2b+3c+4d = e_i, and we multiply the
number of solutions over all p_i.
"""

from __future__ import annotations

from collections import Counter
from itertools import product

from sympy import primerange


def num_factors_in_factorial(n: int, p: int) -> int:
    """Count how many times p divides n!."""
    count = 0
    power = p
    while power <= n:
        count += n // power
        power *= p
    return count


def mod_inverse(a: int, m: int) -> int:
    """Modular inverse using extended Euclidean algorithm."""
    if m == 1:
        return 0
    t, new_t = 0, 1
    r, new_r = m, a % m
    while new_r != 0:
        q = r // new_r
        t, new_t = new_t, t - q * new_t
        r, new_r = new_r, r - q * new_r
    if r != 1:
        raise ValueError("Modular inverse does not exist")
    if t < 0:
        t += m
    return t


def factorial(n: int, mod: int) -> int:
    """Compute n! modulo mod."""
    result = 1
    for i in range(1, n + 1):
        result = (result * i) % mod
    return result


def triangular(n: int) -> int:
    """Triangular number."""
    return n * (n + 1) // 2


def parity(n: int) -> int:
    """Return 1 if even, -1 if odd."""
    return 1 if n % 2 == 0 else -1


def solve() -> int:
    """Solve Problem 636."""
    N = 10**6
    K = 4
    M = 10**9 + 7

    primes = list(primerange(2, N + 1))
    es = [num_factors_in_factorial(N, p) for p in primes]

    cache = {}

    def helper(max_group: int, pattern: list[int]) -> int:
        """Recursive helper."""
        if len(pattern) == triangular(K):
            mult = 1
            jumps = Counter()
            groups = Counter(pattern)

            for group in range(max_group + 1):
                group_size = groups[group]
                jump = 0
                for j in range(1, K + 1):
                    for k in range(1, j + 1):
                        idx = triangular(j - 1) + k - 1
                        if idx < len(pattern) and pattern[idx] == group:
                            jump += j
                mult = (
                    mult * parity(group_size) * factorial(group_size - 1, M)
                ) % M
                jumps[jump] += 1

            key = tuple(sorted(jumps.items()))
            if key in cache:
                return cache[key]

            total = es[0] if es else 0
            dp = [0] * (total + 1)
            dp[0] = 1

            for jump in jumps:
                for i in range(jump, total + 1):
                    dp[i] = (dp[i] + dp[i - jump]) % M

            result = 1
            for e in es:
                result = (result * dp[e]) % M

            cache[key] = result
            return (mult * result) % M

        result = 0
        for group in range(max_group + 2):
            new_pattern = pattern + [group]
            result = (result + helper(max(max_group, group), new_pattern)) % M
        return result

    ans = helper(-1, []) % M
    for i in range(1, K + 1):
        ans = (ans * mod_inverse(factorial(i, M), M)) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
