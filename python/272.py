"""Project Euler Problem 272: Modular Cubes, part 2.

Find the sum of all positive numbers r up to N such that 1 has exactly 3^K
cube roots (mod r).

In mod 2, mod 3, and mod p^e for p ≡ 2 (mod 3), 1 has one cube root. In mod
3^e for e > 1 and mod p^e for p ≡ 1 (mod 3), 1 has three cube roots. This
means that 1 has 3^K cube roots (mod r) if r has exactly five distinct "good"
prime factors, where a good prime factor is either equal to 1 (mod 3) or
equal to 3 if there are at least two of them.
"""

from __future__ import annotations

from typing import List

from sympy import primerange


def primes_mod(n: int, a: int, m: int) -> List[int]:
    """Return primes p < n such that p ≡ a (mod m)."""
    return [p for p in primerange(2, n) if p % m == a]


def solve() -> int:
    """Solve Problem 272."""
    N = 10**11
    K = 5

    small_ps = primes_mod(100, 1, 3)
    max_p = N // 9
    max_q = N // 9
    for i in range(K - 2):
        max_p //= small_ps[i]
    for i in range(K - 1):
        max_q //= small_ps[i]

    ps = primes_mod(int(max_p), 1, 3)

    # Build prod_qs: numbers not divisible by primes ≡ 1 (mod 3)
    prod_qs = [True] * (int(max_q) + 1)
    for p in primes_mod(int(max_q), 1, 3):
        for i in range(p, int(max_q) + 1, p):
            prod_qs[i] = False

    ans = 0

    def helper(index: int, prod: int, num_ps: int, ps_list: List[int]) -> None:
        """Recursive helper."""
        nonlocal ans
        if num_ps >= K:
            for i in range(int(max_q) + 1):
                if i * prod <= N and prod_qs[i]:
                    ans += i * prod
            return
        for new_index in range(index, len(ps_list)):
            if new_index + K - num_ps > len(ps_list):
                break
            min_final_prod = prod
            for i in range(K - num_ps):
                min_final_prod *= ps_list[new_index + i]
            if min_final_prod > N:
                break
            new_prod = prod * ps_list[new_index]
            while new_prod <= N:
                helper(new_index + 1, new_prod, num_ps + 1, ps_list)
                new_prod *= ps_list[new_index]

    # Case 1: with two factors of 3
    for i in range(9, int(max_q) + 1, 9):
        prod_qs[i] = False
    helper(0, 9, 2, ps)

    # Case 2: without factors of 3
    prod_qs_orig = prod_qs[:]
    helper(0, 1, 0, ps)

    return ans


def main() -> None:
    """Main entry point."""
    result = solve()
    print(result)


if __name__ == "__main__":
    main()
