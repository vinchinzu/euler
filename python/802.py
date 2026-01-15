"""Project Euler Problem 802: Iterated Quadratic Map.

Let f(x, y) = (x²-x-y², 2xy-y+π) and let the period of (x,y) be the smallest
n such that f^n(x,y) = (x,y). Find the sum of the x-coordinates of all
points with period up to N.

The sum of the x-coordinates of all points (x,y) where f^n(x,y) = (x,y) is
max(2^{n-1}, 2). This means the sum of the x-coordinates for points with
period n can be computed using PIE, Σ_d µ(d) max(2^{n/d-1}, 2). We have

S = Σ_{n=1}^N Σ_{d|n} µ(d) max(2^{n/d-1}, 2)
  = Σ_{d=1}^N µ(d) Σ_{k=1}^⌊N/d⌋ max(2^{i-1}, 2)
  = Σ_{d=1}^N µ(d) 2^⌊N/d⌋.
"""

from __future__ import annotations

from typing import List


def sieve_mobius(limit: int) -> List[int]:
    """Compute Mobius function µ(n) for n from 0 to limit."""
    mu = [1] * (limit + 1)
    mu[0] = 0
    mu[1] = 1
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False

    for i in range(2, limit + 1):
        if is_prime[i]:
            for j in range(i, limit + 1, i):
                is_prime[j] = False
                if j % (i * i) == 0:
                    mu[j] = 0
                else:
                    mu[j] = -mu[j]

    return mu


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
    """Solve Problem 802."""
    N = 10**7
    M = 1020340567

    mu = sieve_mobius(N)
    pow2s = [pow_mod(2, N // i, M) for i in range(N + 1)]

    ans = 0
    for i in range(1, N + 1):
        ans = (ans + mu[i] * pow2s[i]) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
