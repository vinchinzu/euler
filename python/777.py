"""Project Euler Problem 777: Lissajous Curves.

Let the Lissajous curve C_{a,b} be a curve parameterized by x = cos(at) and
y = cos(b (t-π/10)). Find Σ(x²+y²) for all (x,y) at which C_{a,b} crosses
itself, summed over all relatively prime a,b with 2≤a,b≤N.

For most curves C_{a,b}, t ranges from 0 to 2π to trace the curve once. We need
to find t1, t2 such that x1 = x2 and y1 = y2. For x1 = x2, we need t1 and t2
to either differ by a multiple of 2π/a, or sum to a multiple of 2π/a. For y1
= y2, we need t1 and t2 to either differ by a multiple of 2π/b, or sum to
2π/10 plus a multiple of 2π/b. For each possible pair of a difference and
sum, we can solve the pair of equations to get a pair (t1, t2) and
consequently a point (x,y). From small values of a,b, we can find the formula:

Σ(x²+y²) = (4ab-3a-3b)/2

For some a,b, it is possible for 2π/10 plus a multiple of 2π/b to equal a
multiple of 2π/a. This only happens if 10|ab, in which case t only ranges
from half of the range (0,2π), and from small values of a,b, we instead have
the formula:

Σ(x²+y²) = (2ab-3a-3b+4)/4

Now we need only sum these formulas over relatively prime 2≤a,b≤N. The
relatively prime condition is handled in the usual way. If we sum only the
first formula, we get simple arithmetic series, which can be summed in
constant time for a given g. Then to handle a,b where 10|ab, we use
inclusion-exclusion, adding the difference of the two formulas.
"""

from __future__ import annotations

from math import gcd, isqrt
from typing import List


def pre_mobius(limit: int) -> List[int]:
    """Precompute Möbius function."""
    mu = [1] * (limit + 1)
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


def pre_ff(limit: int) -> List[int]:
    """Precompute smallest prime factor."""
    ff = [0] * (limit + 1)
    for i in range(2, limit + 1):
        if ff[i] == 0:
            ff[i] = i
            for j in range(2 * i, limit + 1, i):
                if ff[j] == 0:
                    ff[j] = i
    return ff


def sq(n: int) -> int:
    """Square of n."""
    return n * n


def tr(n: int) -> int:
    """Triangular number."""
    return n * (n + 1) // 2


def fsq(n: int) -> int:
    """Sum of squares: 1² + 2² + ... + n²."""
    return n * (n + 1) * (2 * n + 1) // 6


def all_divisors(n: int) -> List[int]:
    """Return all divisors of n."""
    divisors = []
    for i in range(1, int(n**0.5) + 1):
        if n % i == 0:
            divisors.append(i)
            if i != n // i:
                divisors.append(n // i)
    return sorted(divisors)


def num_divisors(n: int) -> int:
    """Count number of divisors."""
    count = 0
    for i in range(1, int(n**0.5) + 1):
        if n % i == 0:
            count += 2
            if i * i == n:
                count -= 1
    return count


def solve() -> float:
    """Solve Problem 777."""
    N = 1000000
    ff = pre_ff(N)
    mobius = pre_mobius(N)
    num_divs_10 = num_divisors(10)

    ans = 0.0

    for g in range(1, N + 1):
        n = N // g
        t = gcd(10, g)
        res = 2.0 * fsq(g * tr(n)) - 3.0 * n * g * tr(n)

        for d in all_divisors(10 // t):
            e = (10 // t) // d
            res += (
                num_divs_10
                * (
                    -6.0 * sq(g) * d * tr(n // d) * e * tr(n // e)
                    + 3.0 * (n // d) * g * e * tr(n // e)
                    + 3.0 * (n // e) * g * d * tr(n // d)
                    + 4.0 * (n // d) * (n // e)
                )
                / 4.0
            )

        ans += mobius[g] * res

    return ans


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.9e}")
    return result


if __name__ == "__main__":
    main()
