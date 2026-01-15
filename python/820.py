"""Project Euler Problem 820: Nth digit of reciprocal.

Let d_n(x) be the nth decimal digit of the fractional part of x. Find
Σ_{k=1}^N d_n(1/k).

Given x, the fractional part of 10^{N-1} x has our desired digit right
after the decimal point. To find it, we multiply by 10 and take the floor.

As a quick optimization, instead of computing 10^{N-1} (mod k) for all k,
we can just take 10^{N-1} (mod 2k) (mod k) if k is small enough.
"""

from __future__ import annotations


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    result = 1
    base = base % mod
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def solve() -> int:
    """Solve Problem 820."""
    N = 10**7
    B = 10

    pows = [0] * (N + 1)
    for k in range(N, 0, -1):
        if 2 * k <= N:
            pows[k] = pows[2 * k] % k
        else:
            pows[k] = pow_mod(B, N - 1, k)

    ans = 0
    for k in range(1, N + 1):
        ans += (pows[k] * B) // k

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
