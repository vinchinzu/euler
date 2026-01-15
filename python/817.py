"""Project Euler Problem 817: Digits in Squares.

Let M(n,d) be the smallest integer m such that m² in base n contains the
digit d. Find Σ_{d=1}^N M(P, P-D).

If d is a quadratic residue where d=s², then M(p,d) is clearly s or p-s,
whichever is smaller. Otherwise, it will almost certainly appear in the
"tens" digit first. So we iterate over possible "hundreds" digits h
starting from 0, determining whether d appears by checking if
√(h + d*p) ≤ ⌊√(h + (d+1)p - 1)⌋.
"""

from __future__ import annotations

from math import isqrt


def is_square_mod(n: int, mod: int) -> bool:
    """Check if n is a quadratic residue modulo mod."""
    for i in range(mod):
        if (i * i) % mod == n % mod:
            return True
    return False


def sqrt_mod(n: int, mod: int) -> int:
    """Find square root modulo mod (if exists)."""
    for i in range(mod):
        if (i * i) % mod == n % mod:
            return i
    return -1


def M(p: int, d: int) -> int:
    """Compute M(p, d)."""
    if is_square_mod(d, p):
        sqrt_val = sqrt_mod(d, p)
        return min(sqrt_val, p - sqrt_val)

    h = 0
    while True:
        # Check if there's a square in [h + d*p, h + (d+1)*p - 1]
        low_bound = h + d * p
        high_bound = h + (d + 1) * p - 1
        sqrt_high = isqrt(high_bound)
        if sqrt_high * sqrt_high >= low_bound:
            return sqrt_high
        h += p * p


def solve() -> int:
    """Solve Problem 817."""
    N = 100_000
    P = 10**9 + 7

    ans = 0
    for d in range(1, N + 1):
        ans += M(P, P - d)

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
