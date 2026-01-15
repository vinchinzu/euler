"""Project Euler Problem 731: A Stoneham Number.

Starting at the Nth digit after the decimal point of the decimal expansion of
Σ_{i=1}^∞ 1/(3^i 10^(3^i)), find the next K digits.

For each i, we need to find the K digits of 1 / 3^i starting from digit
N - 3^i. This is just the last K digits of 10^{N+K-3^i} / 3^i, but we need
to subtract the fractional part, which is 10^{N+K-3^i} (mod 3^i) / 3^i. To
protect against carries from the next few digits, we fetch K+2 digits instead
of K, and only take the first K digits at the end.
"""

from __future__ import annotations


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Fast exponentiation modulo mod."""
    result = 1
    base = base % mod
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def mod_inv(a: int, m: int) -> int:
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


def solve() -> str:
    """Solve Problem 731."""
    n = 10**16
    k = 10
    t = 3
    b = 10
    m = b ** (k + 2)

    ans = 0
    i = 1
    while t**i < n:
        three_power_i = t**i
        exp = n + k + 1 - three_power_i
        if exp > 0:
            term1 = pow_mod(b, exp, m)
            term2 = pow_mod(b, exp, three_power_i)
            inv_three_i = mod_inv(three_power_i, m)
            ans = (ans + (term1 - term2) * inv_three_i) % m
        i += 1

    return str(ans % m)[:k]


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return int(result)


if __name__ == "__main__":
    main()
