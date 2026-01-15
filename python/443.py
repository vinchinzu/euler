"""Project Euler Problem 443: GCD sequence.

Find g(N), where g(4) = 13 and g(n) = g(n-1) + GCD(n, g(n-1)).

Given g(n), the consecutive differences between subsequent terms must be
1, until some term g(n') satisfies GCD(g(n'), n'+1) > 1. Since the GCD
must divide the difference, g(n') - (n'+1), but this equals g(n) - (n+1)
because the difference is constant until then, we can figure out n' by
simply trying all divisors of g(n) - (n+1). This allows us to compute
g(n'), and then g(n'+1), and then we can repeat.

As an optimization, instead of factoring g(n) - (n+1) first, we initially
try g(n+d) for small values of d. This allows us to avoid an expensive
factorization in most cases.
"""

from __future__ import annotations

from math import gcd

from sympy import factorint


def solve() -> int:
    """Solve Problem 443."""
    N = 10**15
    L = 1000

    ans = 13
    n = 4

    while n < N:
        # Try small increments first
        found = False
        for d in range(L):
            if gcd(ans + d, n + d + 1) > 1:
                ans = ans + d
                n += d + 1
                ans += gcd(n, ans)
                found = True
                break

        if not found:
            # Factor and find next jump
            diff = ans - (n + 1)
            if diff == 0:
                n += 1
                ans += 1
            else:
                factors = factorint(abs(diff))
                next_val = ans + N - n - 1
                for p in factors:
                    # Find smallest multiple of p >= ans such that gcd(ans, n+1) > 1
                    candidate = ((ans // p) + 1) * p
                    if candidate < next_val:
                        next_val = candidate
                jump = next_val - ans
                n += jump + 1
                ans = next_val + gcd(n, next_val)

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
