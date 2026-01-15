"""Project Euler Problem 710: One Million Members.

Let t(n) be the number of ways to write n as a sum of a palindromic series of
integers containing at least one 2. Find the smallest N > 42 for which t(N)
is a multiple of a million.

Let u(n) be the number of ways to write n without the condition of having at
least one 2. We can have a single term n, or the first and last terms can be
k and we recurse on the remaining n-2k: u(n) = 1 + Σ_{k=1}^⌊n/2⌋ u(n-2k).
It is easy to see by induction that u(n) = 2^⌊n/2⌋.

The recurrence for t(n) is similar: t(n) = Σ_{k=1}^⌊n/2⌋ t(n-2k), except we
cannot use a single term when n>2, and for the k=2 term we replace t(n-2k)
with u(n-2k) (since we already have a 2). We can then simplify this to:

t(n) = t(n-2) + u(n-4) + Σ_{k=3}^⌊n/2⌋ t(n-2k)
     = t(n-2) + 2^{n/2-2} + Σ_{k=3}^⌊n/2⌋ t(n-2k)
     = 2t(n-2) - t(n-4) + t(n-6) + 2^{n/2-3}.
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


def imod(a: int, m: int) -> int:
    """Modulo operation: a mod m."""
    return a % m


def solve() -> int:
    """Solve Problem 710."""
    m = 1000000
    ts = [0, 0, 1, 0, 2, 1]

    while ts[-1] > 0:
        n = len(ts)
        term = (
            2 * ts[n - 2]
            - ts[n - 4]
            + ts[n - 6]
            + pow_mod(2, n // 2 - 3, m)
        )
        ts.append(imod(term, m))

    return len(ts) - 1


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
