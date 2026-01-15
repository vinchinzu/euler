"""Project Euler Problem 735: Divisors of 2n².

Let f(n) be the number of divisors of 2n² no greater than n. Find Σ_{k=1}^N f(k).

Each divisor can be represented as the first factor of the product
(n-x)(2n+y)=2n², where 0≤x≤n. This can be rearranged to n=xy/(y-2x). Let
g = GCD(x,y), x = g*x', and y = g*y'. Then for any relatively prime (x',y'),
first we must have y'>2x' in order for n to be positive. Further, we must have
y'-2x'|2g, so we need to choose g such that n=g²x'y'/(g(y'-2x')) = k*x'y' ≤ N,
where k is an integer or half-integer if y'-2x' is even. This yields
⌊N/(x'y')⌋ or ⌊2N/(x'y')⌋ solutions for a given (x',y').

We remove the GCD constraint in the usual way. For the half-integer constraint,
we can encode the parity constraint by subtracting solutions with N replaced
with N/2, then adding back it with N/4, etc. The problem then becomes
computing Σ_{(x',y')} ⌊N/(x'y')⌋, for both y'>x' and y'>2x', which is equal to
the number of solutions to x*y*z ≤ N with each of these constraints. To
compute this efficiently in O(N^{2/3}) time, we iterate over the smaller two
of (x,y,z), and count the number of solutions for the third variable.
"""

from __future__ import annotations

from math import gcd, isqrt


def pre_mobius(limit: int) -> list[int]:
    """Precompute Mobius function."""
    mobius = [1] * (limit + 1)
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False

    for i in range(2, limit + 1):
        if is_prime[i]:
            for j in range(i, limit + 1, i):
                is_prime[j] = False
                mobius[j] *= -1
            for j in range(i * i, limit + 1, i * i):
                mobius[j] = 0

    return mobius


def sq(n: int) -> int:
    """Square."""
    return n * n


def cb(n: int) -> int:
    """Cube."""
    return n * n * n


def parity(n: int) -> int:
    """Return 1 if n is even, -1 if odd."""
    return 1 if n % 2 == 0 else -1


def solve() -> int:
    """Solve Problem 735."""
    N = 10**12
    L = isqrt(N)

    mobius = pre_mobius(L)
    ans = N

    for g in range(1, L + 1):
        if sq(g) >= N:
            break
        t = 0
        while sq(g) << t <= N:
            n_val = N // sq(g) >> t

            res = 0

            # x*y*z≤n, y>x
            for x in range(1, int(n_val ** (1 / 3)) + 1):
                if cb(x) > n_val:
                    break
                for y in range(x + 1, int((n_val / x) ** 0.5) + 1):
                    if x * sq(y) > n_val:
                        break
                    res += n_val // (x * y) - y

            for x in range(1, int(n_val ** (1 / 3)) + 1):
                if cb(x) > n_val:
                    break
                for z in range(x + 1, int((n_val / x) ** 0.5) + 1):
                    if x * sq(z) > n_val:
                        break
                    res += n_val // (x * z) - (z - 1)

            for z in range(1, int(n_val ** (1 / 3)) + 1):
                if cb(z) > n_val:
                    break
                for x in range(z, int((n_val / z) ** 0.5) + 1):
                    if z * sq(x) > n_val:
                        break
                    res += n_val // (x * z) - x

            # x*y*z≤n, y>2x
            for x in range(1, int(n_val ** (1 / 3)) + 1):
                if cb(x) > n_val:
                    break
                for y in range(2 * x + 1, int((n_val / x) ** 0.5) + 1):
                    if x * sq(y) > n_val:
                        break
                    res += n_val // (x * y) - y

            for x in range(1, int(n_val ** (1 / 3)) + 1):
                if cb(x) > n_val:
                    break
                for z in range(x + 1, int((n_val / x) ** 0.5) + 1):
                    if x * sq(z) > n_val or 2 * z * sq(x) > n_val:
                        break
                    res += n_val // (x * z) - max(2 * x, z - 1)

            for z in range(1, int(n_val ** (1 / 3)) + 1):
                if cb(z) > n_val:
                    break
                for x in range(z, int((n_val / (2 * z)) ** 0.5) + 1):
                    if 2 * z * sq(x) > n_val:
                        break
                    res += n_val // (x * z) - 2 * x

            ans += res * parity(t) * mobius[g]
            t += 1

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
