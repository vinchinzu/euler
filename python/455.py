"""Project Euler Problem 455: Powers with trailing digits.

Let f(n) be the largest x ≤ K such that n^x ≡ x (mod K). Find Σ_{n=2}^N f(n).
"""

from __future__ import annotations


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
    """Solve Problem 455."""
    N = 1_000_000
    K = 10**9
    ans = 0

    def f(n: int) -> int:
        """Compute f(n)."""
        if n % 10 == 0:
            return 0
        f_val = 2
        while True:
            new_f = pow_mod(n, f_val, K)
            if f_val == new_f:
                return f_val
            f_val = new_f

    for n in range(2, N + 1):
        ans += f(n)

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
