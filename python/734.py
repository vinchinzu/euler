"""Project Euler Problem 734: A Bit of Prime.

Find the number of K-tuples (x_1, x_2, ... x_K) such that each element is a
prime ≤ N and the bitwise OR of all x_i is also a prime ≤ N.

Let c_i = 1 if i is prime, and 0 otherwise, and let X = Σ_i c_i x^i. If we
define multiplication by (x^i)(x^j) = x^{i|j} (| is the bitwise OR), then we
can see that each coefficient c_i of X^K represents the number of ways that the
bitwise OR of K of these primes equals i. The answer is the sum of all c_p for
p prime ≤ N.

To efficiently compute X^K, we use a variation of nimPow, using a different
Hadamard algorithm that works for bitwise OR instead of bitwise XOR.
"""

from __future__ import annotations

from math import ceil, log2

from sympy import primerange


def iceil_pow(n: int, base: int) -> int:
    """Smallest power of base >= n."""
    if n <= 1:
        return 1
    exp = ceil(log2(n) / log2(base))
    return base**exp


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    if mod <= 0:
        msg = "mod must be positive"
        raise ValueError(msg)
    if base == 0:
        return 0
    base %= mod
    result = 1
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def solve() -> int:
    """Solve Problem 734."""
    N = 10**6
    K = 999983
    M = 10**9 + 7
    L = iceil_pow(N, 2)

    # Initialize array with primes
    primes_list = list(primerange(2, N + 1))
    A = [0] * L
    for p in primes_list:
        if p < L:
            A[p] = 1

    # Forward Hadamard transform for bitwise OR
    u = 1
    while u < L:
        for x in range(0, L):
            if x & u == 0:  # Only process if bit is not set
                A[x | u] = (A[x | u] + A[x]) % M
        u <<= 1

    # Raise to K-th power
    for i in range(L):
        A[i] = pow_mod(A[i], K, M)

    # Inverse Hadamard transform
    u = 1
    while u < L:
        for x in range(0, L):
            if x & u == 0:
                A[x | u] = (A[x | u] - A[x]) % M
                if A[x | u] < 0:
                    A[x | u] += M
        u <<= 1

    # Sum over primes
    ans = 0
    for p in primes_list:
        if p < L:
            ans = (ans + A[p]) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
