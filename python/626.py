"""Project Euler Problem 626: Counting Binary Matrices.

Find the number of equivalence classes of binary NxN matrices, where two
matrices are equivalent if one can be obtained from the other by swapping two
rows, swapping two columns, flipping all elements in a single row, or
flipping all elements in a single column.

We use Burnside's Lemma. There are N!² 2^{2N-1} group elements: N! for each
permutation of the rows and columns, and 2^{2N} to flip some subset of the
rows and columns, but flipping all rows and columns is equivalent to flipping
none.
"""

from __future__ import annotations

from collections import Counter
from itertools import product
from math import gcd

from sympy import factorial


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


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    result = 1
    base %= mod
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def generate_partitions(n: int, max_len: int) -> list[Counter]:
    """Generate all partitions of n."""
    result = []

    def helper(remaining: int, max_val: int, current: list[int]) -> None:
        if remaining == 0:
            result.append(Counter(current))
            return
        for i in range(1, min(max_val + 1, remaining + 1)):
            helper(remaining - i, i, current + [i])

    helper(n, max_len, [])
    return result


def num_arrangements(permutation: Counter, n: int, mod: int) -> int:
    """Number of ways to arrange permutation."""
    result = factorial(n) % mod
    for size, count in permutation.items():
        inv_size = mod_inverse(size, mod)
        for _ in range(count):
            result = (result * inv_size) % mod
        inv_fact_count = mod_inverse(int(factorial(count)), mod)
        result = (result * inv_fact_count) % mod
    return result


def num_restricted_rows(
    permutation: Counter, other_permutation: Counter
) -> int:
    """Count restricted rows."""
    count = 0
    for size in permutation:
        for other_size in other_permutation:
            if (other_size // gcd(size, other_size)) % 2 == 1:
                count += permutation[size]
                break
    return count


def solve() -> int:
    """Solve Problem 626."""
    N = 20
    M = 1001001011

    permutations = generate_partitions(N, N)
    ans = 0

    for perm1 in permutations:
        for perm2 in permutations:
            num_grid_cycles = 0
            for p in perm1:
                for q in perm2:
                    num_grid_cycles += (
                        gcd(p, q) * perm1[p] * perm2[q]
                    )

            num_restricted_rows1 = num_restricted_rows(perm1, perm2)
            num_restricted_rows2 = num_restricted_rows(perm2, perm1)
            all_restricted = (
                num_restricted_rows1 == sum(perm1.values())
                and num_restricted_rows2 == sum(perm2.values())
            )

            term = (
                num_arrangements(perm1, N, M)
                * num_arrangements(perm2, N, M)
                % M
                * pow_mod(2, num_grid_cycles, M)
                % M
                * pow_mod(
                    2,
                    2 * N
                    - num_restricted_rows1
                    - num_restricted_rows2
                    - (0 if all_restricted else 1),
                    M,
                )
                % M
            )
            ans = (ans + term) % M

    inv_fact_n = mod_inverse(int(factorial(N)), M)
    inv_2_pow = mod_inverse(pow_mod(2, 2 * N - 1, M), M)
    ans = (ans * inv_fact_n * inv_fact_n % M * inv_2_pow) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
