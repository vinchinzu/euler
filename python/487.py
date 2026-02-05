"""Project Euler Problem 487: Sums of power sums.

Let f_k(n) be the sum of the k'th powers of the first n positive integers, and
let S_k(n) = Σ_{i=1}^n f_k(i). Find Σ S_K(N) (mod p) for all primes p with
L ≤ p ≤ H.
"""

from __future__ import annotations

from math import isqrt
from typing import List


def is_probable_prime(n: int) -> bool:
    """Check if n is prime."""
    if n < 2:
        return False
    for i in range(2, isqrt(n) + 1):
        if n % i == 0:
            return False
    return True


def sum_powers(n: int, k: int, mod: int) -> int:
    """Sum of k-th powers from 1 to n modulo mod via Lagrange interpolation."""
    if n <= 0:
        return 0

    m = k + 1
    x = n % mod

    # Precompute y[i] = sum_{t=1}^i t^k mod mod for i=0..m
    y = [0] * (m + 1)
    pow_vals = [0] * (m + 1)
    for i in range(1, m + 1):
        pow_vals[i] = pow(i, k, mod)
        y[i] = (y[i - 1] + pow_vals[i]) % mod

    if n <= m:
        return y[n]

    # Precompute factorials and inverse factorials up to m
    fact = [1] * (m + 1)
    for i in range(1, m + 1):
        fact[i] = (fact[i - 1] * i) % mod
    inv_fact = [1] * (m + 1)
    inv_fact[m] = pow(fact[m], mod - 2, mod)
    for i in range(m, 0, -1):
        inv_fact[i - 1] = (inv_fact[i] * i) % mod

    # Prefix and suffix products of (x - i)
    pre = [1] * (m + 2)
    for i in range(0, m + 1):
        pre[i + 1] = (pre[i] * (x - i)) % mod
    suf = [1] * (m + 2)
    for i in range(m, -1, -1):
        suf[i] = (suf[i + 1] * (x - i)) % mod

    # Lagrange interpolation at x
    result = 0
    for i in range(0, m + 1):
        num = pre[i] * suf[i + 1] % mod
        den = inv_fact[i] * inv_fact[m - i] % mod
        term = y[i] * num % mod * den % mod
        if (m - i) & 1:
            result = (result - term) % mod
        else:
            result = (result + term) % mod

    return result % mod


def solve() -> int:
    """Solve Problem 487."""
    N = 10**12
    K = 10_000
    L = 2 * 10**9
    H = 2 * 10**9 + 2000

    ans = 0
    for p in range(L, H):
        if is_probable_prime(p):
            s_k = sum_powers(N, K, p)
            s_k1 = sum_powers(N, K + 1, p)
            term = (((N + 1) % p) * s_k - s_k1) % p
            ans += term

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
