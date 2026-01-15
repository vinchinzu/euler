"""Project Euler Problem 648: Skipping Squares.

Let f(ρ) be the expected number of perfect squares skipped over if we start
at 0 and repeatedly increment by 1 with probability ρ and increment by 2 with
probability 1-ρ. If f(ρ) is expressed as a power series in ρ, find the sum
of the coefficients f_0, f_1, ... f_N.

Let f^s(ρ) be the expected number of perfect squares if we start at s, so
f(ρ) = f^0(ρ). Then if s is a perfect square, then f^s(ρ) = (√s - 1);
otherwise, f^s(ρ) = ρ f^{s+1}(ρ) + (1-ρ) f^{s+2}(ρ).
"""

from __future__ import annotations

from math import isqrt


def is_square(n: int) -> bool:
    """Check if n is a perfect square."""
    root = isqrt(n)
    return root * root == n


def solve() -> int:
    """Solve Problem 648."""
    N = 1000
    M = 10**9

    jump1 = [0] * (N + 1)
    jump2 = [0] * (N + 1)

    max_s = (N // 2) ** 2
    for s in range(max_s, -1, -1):
        f = [0] * (N + 1)
        if s > 0 and is_square(s):
            f[0] = isqrt(s) - 1
        else:
            f[0] = jump2[0]
            for k in range(1, N + 1):
                f[k] = (jump2[k] + jump1[k - 1] - jump2[k - 1]) % M

        jump2 = jump1[:]
        jump1 = f[:]

    ans = sum(jump1) % M
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
