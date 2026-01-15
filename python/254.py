"""Project Euler Problem 254: Sums of Digit Factorials.

Define g(i) to be the smallest positive integer n such that the sum of the
digits of the sum of the factorials of the digits of n equals i. Find the sum
of the digits of all g(i) from 1 ≤ i ≤ N.

For all i ≤ 63, g(i) can be computed by enumerating over all possible sums of
factorials up to 9999999. For every sum f, there is exactly one optimum g such
that the sum of the factorials of the digits of g equals f; this can be
computed by first using as many 9!s as possible, then 8!s, etc., and then
writing those digits in increasing order 122333...999. We can check whether
this g is smaller than the current best value of g for the digit sum of f, and
update it if so.

For i > 63, the optimum sum of factorials with digit sum i must be a single
digit followed by 9s: [i % 9]99999... and again it is possible to find the
optimum g. This is the only candidate that we need to check for i > 63.
"""

from __future__ import annotations


def ipow(base: int, exp: int) -> int:
    """Integer power."""
    return base**exp


def sum_digits(n: int) -> int:
    """Sum of digits of n."""
    return sum(int(d) for d in str(n))


def factorial(n: int) -> int:
    """Factorial of n."""
    if n <= 1:
        return 1
    result = 1
    for i in range(2, n + 1):
        result *= i
    return result


def is_better(g: str, best_g: str) -> bool:
    """Check if g is lexicographically smaller than best_g."""
    if len(g) != len(best_g):
        return len(g) < len(best_g)
    for i in range(len(g) - 1, -1, -1):
        if g[i] != best_g[i]:
            return g[i] < best_g[i]
    return False


def solve() -> int:
    """Solve Problem 254."""
    N = 150
    B = 10
    L = ipow(10, 7)

    C = sum_digits(L - 1)
    sgs: list[int] = [0] * (C + 1)
    gs: list[str | None] = [None] * (C + 1)

    sf = 0
    sg = 0
    g = ""

    for f in range(1, L):
        sf += 1
        n = f
        while n % B == 0:
            sf -= B - 1
            n //= B

        g += "1"
        sg += 1

        n = f
        d = 2
        while d < B and n % d == 0:
            g = g[: len(g) - d] + str(d)
            sg -= (d - 2) * d
            n //= d
            d += 1

        if gs[sf] is None or is_better(g, gs[sf]):
            gs[sf] = g
            sgs[sf] = sg

    ans = sum(sgs)

    for sf in range(C + 1, N + 1):
        f = (sf % (B - 1) + 1) * (B ** (sf // (B - 1))) - 1
        sg = 0
        for d in range(B - 1, 0, -1):
            fact_d = factorial(d)
            sg += (f // fact_d) * d
            f %= fact_d
        ans += sg

    return ans


def main() -> None:
    """Main entry point."""
    result = solve()
    print(result)


if __name__ == "__main__":
    main()
