"""Project Euler Problem 479: Roots on the Rise.

Let a_k, b_k, c_k be the three solutions to the equation 1/x = (k/x)² (k+x²) - k*x.
Find Σ_{p=1}^N Σ_{k=1}^N (a_k + b_k)^p (b_k + c_k)^p (c_k + a_k)^p.
"""

from __future__ import annotations


def sq(n: int) -> int:
    """Square."""
    return n * n


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    result = 1
    base %= mod
    while exp > 0:
        if exp % 2 == 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp //= 2
    return result


def mod_inv(a: int, m: int) -> int:
    """Modular inverse."""
    t, new_t = 0, 1
    r, new_r = m, a
    while new_r != 0:
        q = r // new_r
        t, new_t = new_t, t - q * new_t
        r, new_r = new_r, r - q * new_r
    if r != 1:
        raise ValueError("Inverse does not exist")
    if t < 0:
        t += m
    return t


def solve() -> int:
    """Solve Problem 479."""
    N = 10**6
    M = 10**9 + 7
    ans = 0

    for k in range(1, N + 1):
        k_sq = sq(k) % M
        term = (1 - k_sq) % M
        term = term * (1 - pow_mod(1 - k_sq, N, M)) % M
        term = term * mod_inv(k_sq, M) % M
        ans = (ans + term) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
