"""Project Euler Problem 528: Constrained Sums.

Let S(n, k, b) be the number of solutions to x_1 + x_2 + ... + x_k ≤ n,
where 0 ≤ x_m ≤ b^m for all m. Find Σ_{L≤k≤H} S(10^k, k, k).
"""

from __future__ import annotations


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Fast exponentiation."""
    result = 1
    base %= mod
    while exp > 0:
        if exp % 2 == 1:
            result = (result * base) % mod
        exp >>= 1
        base = (base * base) % mod
    return result


def ncr_mod(n: int, k: int, mod: int) -> int:
    """nCr modulo mod."""
    if k < 0 or k > n:
        return 0
    if k == 0 or k == n:
        return 1

    result = 1
    for i in range(min(k, n - k)):
        result = (result * (n - i)) % mod
        # Modular inverse of (i+1)
        inv = pow(i + 1, mod - 2, mod)
        result = (result * inv) % mod

    return result


def parity(n: int) -> int:
    """Return 1 if n is even, -1 if odd."""
    return 1 if n % 2 == 0 else -1


def S(n: int, k: int, b: int, mod: int) -> int:
    """Compute S(n, k, b) using inclusion-exclusion."""
    result = 0
    for subset in range(1 << k):
        d = 0
        for i in range(k):
            if (subset & (1 << i)) != 0:
                d += pow_mod(b, i + 1, mod) + 1
        if n - d + k >= 0:
            bit_count = bin(subset).count("1")
            result = (
                result + parity(bit_count) * ncr_mod(n - d + k, k, mod)
            ) % mod
    return result


def solve() -> int:
    """Solve Problem 528."""
    L = 10
    H = 15
    M = 10**9 + 7
    B = 10

    ans = 0
    for k in range(L, H + 1):
        n = pow_mod(B, k, M)
        ans = (ans + S(n, k, k, M)) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
