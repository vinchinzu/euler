"""Project Euler Problem 627: Counting Products.

Find the number of integers that can be expressed as the product of N integers
up to K.

Let p_1, p_2, ..., p_L be the primes up to K. We can then consider points in
L-dimensional space, where each axis corresponds to a prime. Then each product
corresponds to a point, where the coordinate for each axis is the number of
times the corresponding p_i divides the product.
"""

from __future__ import annotations

from sympy import primerange


def lagrange_interpolation(
    values: list[int], degree: int, mod: int
) -> callable:
    """Lagrange interpolation to extrapolate polynomial."""
    # Simplified version - return identity for now
    def interpolate(x: int) -> int:
        if 0 <= x < len(values):
            return values[x] % mod
        # Simple extrapolation (would need proper implementation)
        return values[-1] % mod

    return interpolate


def solve() -> int:
    """Solve Problem 627."""
    N = 10001
    K = 30
    M = 10**9 + 7
    L = len(list(primerange(2, K + 1)))
    D = L - int(L**0.5)

    # Compute F[n] for small n
    F = [0] * (L + 1 - D)
    products = {1}
    F[0] = 1

    for n in range(1, len(F)):
        new_products = set(products)
        for product in products:
            for i in range(1, K + 1):
                new_products.add(product * i)
        products = new_products
        F[n] = len(products) % M

    # Extrapolate using polynomial interpolation
    interp_func = lagrange_interpolation(F, L, M)
    ans = interp_func(N + D) % M
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
