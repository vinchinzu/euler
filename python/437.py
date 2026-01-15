"""Project Euler Problem 437: Fibonacci primitive roots.

An integer x is a Fibonacci primitive root (mod p) if its powers cycle
through all values from 1 to p-1 (mod p), and x^n + x^{n+1} ≡ x^{n+2}.
Find the sum of all primes p ≤ N that have a Fibonacci primitive root.

From Lemmas 5 and 6 in this paper:
https://www.mathstat.dal.ca/FQ/Scanned/15-4/deleon.pdf, an equivalent
condition is that either p = 5, or p = 1 or 9 (mod 10) and the Fibonacci
numbers (mod p) have period p-1. The first condition is equivalent to p² ≢
4 (mod 5). For the second condition, we need to check the pairs of
Fibonacci numbers F_{(p-1)/d} and F_{(p-1)/d + 1} for all divisors d of
p-1. If none of the pairs are congruent to 0 and 1 respectively, then the
prime p has a Fibonacci primitive root.
"""

from __future__ import annotations

from sympy import primerange


def sq_mod(n: int, mod: int) -> int:
    """Return n squared modulo mod."""
    return (n * n) % mod


def fibonacci_mod(n: int, mod: int) -> int:
    """Compute nth Fibonacci number modulo mod."""
    if n == 0:
        return 0
    if n == 1:
        return 1

    def mat_mult(
        a: list[list[int]], b: list[list[int]]
    ) -> list[list[int]]:
        return [
            [
                (a[0][0] * b[0][0] + a[0][1] * b[1][0]) % mod,
                (a[0][0] * b[0][1] + a[0][1] * b[1][1]) % mod,
            ],
            [
                (a[1][0] * b[0][0] + a[1][1] * b[1][0]) % mod,
                (a[1][0] * b[0][1] + a[1][1] * b[1][1]) % mod,
            ],
        ]

    def mat_pow(mat: list[list[int]], exp: int) -> list[list[int]]:
        result = [[1, 0], [0, 1]]
        base = mat
        while exp:
            if exp & 1:
                result = mat_mult(result, base)
            base = mat_mult(base, base)
            exp >>= 1
        return result

    fib_mat = [[1, 1], [1, 0]]
    result_mat = mat_pow(fib_mat, n - 1)
    return result_mat[0][0]


def prime_factors(n: int) -> set[int]:
    """Get prime factors of n."""
    factors = set()
    d = 2
    while d * d <= n:
        while n % d == 0:
            factors.add(d)
            n //= d
        d += 1
    if n > 1:
        factors.add(n)
    return factors


def has_fibonacci_primitive_root(p: int) -> bool:
    """Check if prime p has a Fibonacci primitive root."""
    if sq_mod(p, 5) == 4:
        return False
    if p == 5:
        return True

    factors = prime_factors(p - 1)
    for d in factors:
        fib_val = fibonacci_mod((p - 1) // d, p)
        fib_next = fibonacci_mod((p - 1) // d + 1, p)
        if fib_val == 0 and fib_next == 1:
            return False
    return True


def solve() -> int:
    """Solve Problem 437."""
    N = 10**8
    ans = 0
    for p in primerange(3, N + 1):
        if has_fibonacci_primitive_root(p):
            ans += p
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
