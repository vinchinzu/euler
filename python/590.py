"""Project Euler Problem 590: Sets with LCM.

Find the number of sets of positive integers whose LCM is equal to the
LCM of the numbers from 1 to n, mod M.

Given a number n = prod_k (p_k)^(e_k), a set whose LCM is a divisor of
n can only contain numbers prod_k (p^k)^(f_k) where 0 ≤ f_k ≤ e_k.
There are prod_k (e_k + 1) such numbers, so the total number of sets is
f(n) = 2^(prod_k (e_k + 1)), assuming that the empty set has LCM of 1.
By PIE, the number of sets whose LCM is exactly n is
sum_{d|n} μ(n/d) f(d).
"""

from __future__ import annotations

from math import comb, gcd, log
from typing import List

from sympy import primerange


def euler_totient_mod(n: int, mod: int) -> int:
    """Compute Euler's totient modulo mod."""
    result = n
    temp = n
    p = 2
    while p * p <= temp:
        if temp % p == 0:
            result = result // p * (p - 1)
            while temp % p == 0:
                temp //= p
        p += 1
    if temp > 1:
        result = result // temp * (temp - 1)
    return result % mod


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Fast exponentiation modulo mod."""
    result = 1
    base = base % mod
    while exp > 0:
        if exp % 2 == 1:
            result = (result * base) % mod
        exp //= 2
        base = (base * base) % mod
    return result


def solve() -> int:
    """Solve Problem 590."""
    N = 50_000
    M = 10**9
    phi_M = euler_totient_mod(M, M)

    # Compute exponents for N!
    es: List[int] = []
    for p in primerange(2, N + 1):
        e = 0
        n = N
        while n > 0:
            n //= p
            e += n
        es.append(e)

    # Precompute nCr table
    max_e = max(es) if es else 0
    n_crs: List[List[int]] = []
    for i in range(max_e + 1):
        row = [1]
        for j in range(1, i + 1):
            row.append((row[-1] * (i - j + 1) * pow_mod(j, phi_M - 1, M)) % M)
        n_crs.append(row)

    ans = 0

    def helper(
        es: List[int],
        fs: List[int],
        index: int,
        sum_fs: int,
        num_ds: int,
        f_val: int,
    ) -> None:
        """Recursive helper."""
        nonlocal ans
        if index == 0:
            parity = 1 if sum_fs % 2 == 0 else -1
            ans = (ans + parity * num_ds * f_val) % M
            return

        # Update f_val
        if index + 1 < len(fs):
            exp = fs[index + 1]
            f_val = pow_mod(f_val, pow_mod(index + 1, exp, phi_M), M)

        # Try all values of fs[index]
        for fs_val in range(es[index], -1, -1):
            fs[index] = fs_val
            new_num_ds = (num_ds * n_crs[es[index]][fs_val]) % M
            new_f_val = f_val
            if fs_val < es[index]:
                new_f_val = (new_f_val * pow_mod(index + 1, 1, M)) % M
            helper(es, fs, index - 1, sum_fs + fs_val, new_num_ds, new_f_val)

    fs = [0] * len(es)
    helper(es, fs, len(es) - 2, 0, 1, 2)

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
