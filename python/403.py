"""Project Euler Problem 403: Lattice points enclosed by parabola and line.

Let L(a, b) be the number of lattice points inside or on the border of
D(a, b), the region between y = x² and y = a*x + b. Find sum_{a,b} L(a, b)
for all integer |a|,|b| ≤ N where the area of D(a, b) is rational.

Suppose that y = x² and y = a*x + b intersect at x = r and x = s where r ≤ s.
Then by Vieta's formulas we can compute that r+s = a and r*s = -b. We also
have s-r = √( (r+s)² - 4 (r*s) ) = √(a²+4b).

The area of D(a, b) is ∫_{r}^{s} (a*x + b - x²) dx = (a²/2 + b - (a²+b)/3)
(s-r). This means that (s-r) must be an integer, and since their sum and
product is an integer, both r and s must be integers.

The number of lattice points in D(a, b) is:
L(a, b) = sum_{r}^{s} (a*x + b - x² + 1)
        = a (sum_{r}^{s} x) - (sum_{r}^{s} x²) + (sum_{r}^{s} b+1)
        = a ( s(s+1)/2 - (r-1)r/2 ) - ( s(s+1)(2s+1)/6 - (r-1)r(2r-1)/6 )
          + (b+1)(s-r+1)
        = ((s-r)³ + 5(s-r)) / 6 + 1

We can compute this in O(√N) time as follows. Iterate over all -√N ≤ r ≤ √N.
For each r, we can choose any s that satisfies s ≥ r, s ≤ N-r, and s ≤ N/|r|.
This means that s-r can range from 0 to some maximum value, and sum L(a, b)
can be computed by the formulas for the sum of consecutive integers and sum
of consecutive integer cubes.

However, it is also possible for r < √N. Fortunately, those values are
identical to the values where s > √N, so we need only double-count the
values for -√N ≤ r ≤ √N where s > √N. Equivalently, we double-count for
all s, and then subtract the values where s ≤ √N.
"""

from __future__ import annotations

from math import isqrt


def sum_powers(n: int, exp: int, mod: int | None = None) -> int:
    """Return sum_{k=1}^n k^exp, optionally modulo mod."""
    if exp == 1:
        result = n * (n + 1) // 2
    elif exp == 2:
        result = n * (n + 1) * (2 * n + 1) // 6
    elif exp == 3:
        result = (n * (n + 1) // 2) ** 2
    else:
        result = sum(k**exp for k in range(1, n + 1))
    return result % mod if mod else result


def solve() -> int:
    """Solve Problem 403."""
    N = 10**12
    L = isqrt(N)
    M = 10**8

    ans = 0
    for r in range(-L, L + 1):
        max_s = N if r == 0 else min(N // abs(r), N - r)
        ans += (
            2
            * (
                (sum_powers(max_s - r, 3, 6 * M) + 5 * sum_powers(max_s - r, 1, 6 * M))
                // 6
                + (max_s - r + 1)
            )
            - (
                (sum_powers(L - r, 3, 6 * M) + 5 * sum_powers(L - r, 1, 6 * M))
                // 6
                + (L - r + 1)
            )
        )
    ans %= M
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
