"""Project Euler Problem 402: Integer-valued polynomials.

Let M(a,b,c) be the largest integer that divides n⁴ + a*n³ + b*n² + c*n for
all integers n, and let S(i) = sum_{1 ≤ a,b,c ≤ N} M(a,b,c). Find
sum_{k=2}^N S(F_k), where F_k is the kth Fibonacci number.

We can determine that S(i) depends only on i (mod K! = 24), and the function
must be a polynomial with degree K - 1. This means we can extrapolate any
value of S(i) with only the first K * K! values of S(i).

Since sum S(F_k) is a cubic in F_k, and F_k is a linear combination of
F_{k - K!} and F_{k - 2K!}, the values of sum(F_k) for all k with the same
remainder r (mod K!) must satisfy a linear recurrence. This means that for
each possible remainder, we can compute sum S(F_k) for all F_k with that
remainder using extrapolation, and then sum all the values together.
"""

from __future__ import annotations

from functools import lru_cache
from itertools import product
from math import gcd
from typing import Callable, List


def factorial(n: int) -> int:
    """Compute factorial of n."""
    result = 1
    for i in range(2, n + 1):
        result *= i
    return result


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


def gcds(limit: int) -> List[List[int]]:
    """Precompute GCD table."""
    result = [[0] * (limit + 1) for _ in range(limit + 1)]
    for i in range(limit + 1):
        for j in range(limit + 1):
            result[i][j] = gcd(i, j)
    return result


@lru_cache(maxsize=None)
def fibonacci(n: int) -> int:
    """Compute nth Fibonacci number."""
    if n <= 1:
        return n
    a, b = 0, 1
    for _ in range(2, n + 1):
        a, b = b, a + b
    return b


def fibonaccis(count: int, mod: int) -> List[int]:
    """Generate first count Fibonacci numbers modulo mod."""
    result = [0, 1]
    for _ in range(2, count):
        result.append((result[-1] + result[-2]) % mod)
    return result


def polynomial_extrapolation(
    f: Callable[[int], int], degree: int, mod: int
) -> Callable[[int], int]:
    """Extrapolate polynomial function using Lagrange interpolation.

    Args:
        f: Function to extrapolate
        degree: Degree of polynomial (need degree+1 points)
        mod: Modulus

    Returns:
        Extrapolated function
    """
    # Generate degree+1 points
    n_points = degree + 1
    x_vals = list(range(n_points))
    y_vals = [f(x) % mod for x in x_vals]

    def interpolate(x: int) -> int:
        """Interpolate at point x."""
        result = 0
        for i in range(n_points):
            term = y_vals[i]
            for j in range(n_points):
                if i != j:
                    denom = (x_vals[i] - x_vals[j]) % mod
                    if denom == 0:
                        continue
                    inv = pow_mod(denom, mod - 2, mod)
                    term = (term * (x - x_vals[j]) * inv) % mod
            result = (result + term) % mod
        return result

    return interpolate


def extrapolation(
    f: Callable[[int], int], order: int, mod: int
) -> Callable[[int], int]:
    """Extrapolate function using linear recurrence of given order.

    Args:
        f: Function to extrapolate
        order: Order of recurrence
        mod: Modulus

    Returns:
        Extrapolated function
    """
    # Compute initial values
    values = [f(i) % mod for i in range(order * 2)]

    # Find recurrence coefficients (simplified - assumes periodicity)
    # For this problem, we use polynomial extrapolation
    poly_extrap = polynomial_extrapolation(f, order, mod)

    def apply(x: int) -> int:
        """Apply extrapolation at x."""
        if x < len(values):
            return values[x] % mod
        return poly_extrap(x) % mod

    return apply


def solve() -> int:
    """Solve Problem 402."""
    N = 1234567890123
    K = 4
    L = factorial(K)
    M = 10**9

    # Precompute GCDs
    gcd_limit = K**K * (1 + K * L // (K - 1))
    gcds_table = gcds(int(gcd_limit))

    # Compute S[i] for i in range(K * L)
    S = [0] * (K * L)
    for i in range(K * L):
        # Generate all combinations of coefficients
        for coeffs in product(range(1, i + 1), repeat=K - 1):
            g = 0
            for n in range(1, K + 1):
                val = n
                for coeff in coeffs:
                    val = (val + coeff) * n
                g = gcds_table[g][val]
            S[i] = (S[i] + g) % M

    # Generate Fibonacci numbers
    fibs = fibonaccis(2 * L * K, L * M)

    ans = 0
    for i in range(L):
        r = fibonacci(i + 2) % L
        # Create extrapolation function for S[L * n + r]
        def S_func(n: int) -> int:
            return S[L * n + r]

        extrap = polynomial_extrapolation(S_func, K - 1, M)

        # Create function for sum
        def sum_func(n: int) -> int:
            res = 0
            for j in range(n + 1):
                fib_idx = L * j + i + 2
                if fib_idx < len(fibs):
                    fib_val = fibs[fib_idx]
                    arg = (fib_val // L) % M
                    res = (res + extrap(arg)) % M
            return res % M

        # Extrapolate sum_func
        sum_extrap = extrapolation(sum_func, K, M)
        n_val = (N - 2 - i) // L
        ans = (ans + sum_extrap(n_val)) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
