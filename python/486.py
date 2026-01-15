"""Project Euler Problem 486: Palindromic substrings.

Let F_5(n) be the number of binary strings with length at most n and a
palindromic substring of length at least 5. Find the number of integers 5≤n≤L
such that F_5(n) is divisible by K.
"""

from __future__ import annotations


def order(base: int, mod: int) -> int:
    """Multiplicative order of base modulo mod."""
    if gcd(base, mod) != 1:
        return 0
    result = 1
    power = base % mod
    while power != 1:
        power = (power * base) % mod
        result += 1
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


def gcd(a: int, b: int) -> int:
    """Greatest common divisor."""
    while b:
        a, b = b, a % b
    return a


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


def solve() -> int:
    """Solve Problem 486."""
    N = 10**18
    K = 87_654_321
    L = N

    ord_val = order(2, K)
    inv = mod_inv(100 * ord_val // 6, K)
    C = [57, 41, 25, 9, -8, -26]

    ans = 0
    for k in range(ord_val):
        term = pow_mod(2, k, K) - 100 * (k // 6) + C[k % 6]
        term %= K
        if term < 0:
            term += K
        t = (term * inv) % K
        n_mod = ord_val * t + k
        if 5 <= n_mod <= L:
            count = (L - n_mod) // (ord_val * K) + 1
            ans += count

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
