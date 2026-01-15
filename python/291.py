"""Project Euler Problem 291: Panaitopol Primes.

Find the number of primes less than N which can be expressed as (x⁴ - y⁴) /
(x³ + y³) for some integers x, y.

Let g = gcd(x, y) and let X = x/g, Y = y/g. We have (X, Y) = 1, and p = g(X⁴
- Y⁴) / (X³ + Y³) = g(X - Y)(X² + Y²) / (X² - XY + Y²).

Since X² - XY + Y² and X² + Y² do not share any common factors, we must have
g(X - Y) = X² - XY + Y² and X² + Y² a prime. But if X - Y is divisible by a
prime q, then it divides X² - XY + Y² and therefore (X² - XY + Y²) - (X -
Y)² = XY. However, since q | X - Y, both X and Y must be divisible by q, a
contradiction. Consequently, X - Y = 1, and all primes p must be of the form
X² + Y² = (Y + 1)² + Y².

To efficiently compute all primes of this form, we maintain a sieve f[Y] = (Y
+ 1)² + Y². An entry is prime if it has not been touched by the time we get
to it. If so, we divide out factors of the prime p from every entry Y + p, Y +
2p, etc. Since the solutions of (y + 1)² + y² = 0 (mod p) have solutions Y and
-(Y + 1), we also divide out factors of p from -(Y + 1) + p, -(Y + 1) + 2p,
etc.
"""

from __future__ import annotations


def sq(n: int) -> int:
    """Square of n."""
    return n * n


def f(y: int) -> int:
    """Compute (y + 1)² + y²."""
    return sq(y + 1) + sq(y)


def solve() -> int:
    """Solve Problem 291."""
    N = 5 * (10**15)

    # Find limit such that f(limit) < N
    limit = 0
    while f(limit) < N:
        limit += 1

    # Initialize array
    arr = [f(y) for y in range(limit)]

    ans = 0
    for y in range(1, limit):
        p = arr[y]
        if p > 1:
            if p == f(y):
                ans += 1
            # Sieve: divide out factors of p
            yy = y + p
            while yy < limit:
                while arr[yy] % p == 0:
                    arr[yy] //= p
                yy += p
            # Also handle -(y + 1) mod p
            yy = p - y - 1
            while yy < limit:
                while arr[yy] % p == 0:
                    arr[yy] //= p
                yy += p

    return ans


def main() -> None:
    """Main entry point."""
    result = solve()
    print(result)


if __name__ == "__main__":
    main()
