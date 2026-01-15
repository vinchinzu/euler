"""Project Euler Problem 422: Sequence of points on a hyperbola.

Let X = (7, 1) and define a sequence of points P1 = (13, 61/4), P2 =
(-43/6, -4), and P_k as the unique point on the hyperbola 12x² + 7x*y -
12y² = 625 such that the lines X P_{k-2} and P_{k-1} P_k are parallel.
Find P_N = (a/b, c/d) in lowest terms.

We can define an affine transformation ( (4x-3y)/25, (3x+4y)/25 ) →
(x', y'), which transforms the hyperbola (factored as (4x-3y)(3x+4y) = 625)
to x'y' = 1. This transforms X to X' = (1, 1), P1 to P1' = (1/4, 4), and
P2 to P2' = (-2/3, -3/2).

Define a_k to be the transformed x-coordinate Px_k' if k is odd, or the
y-coordinate Py_k' otherwise. If k is odd, then the parallel line condition
can be written as a cross product, which gives a_k = a_{k-1} a_{k-2}.

This condition also holds for even k, using similar logic. That gives the
closed form a_k = a_1 ^ F_{k-2} * a_2 ^ F_{k-1}, where F_k are the
Fibonacci numbers. For k=N odd, this is the x-coordinate, and the
reciprocal is the y-coordinate. Plugging in a_1 = 1/4 and a_2 = -3/2, we
get:
a_N = Px_N' = (-3)^F_{N-1} / 2^(2F_{N-2}+F_{N-1})
      Py_N' = 2^(2F_{N-2}+F_{N-1}) / (-3)^F_{N-1}

We can now apply the inverse affine transformation to get the original
coordinates of P_N.
"""

from __future__ import annotations


def fibonacci(n: int, mod: int) -> int:
    """Compute nth Fibonacci number modulo mod."""
    if n == 0:
        return 0
    if n == 1:
        return 1

    def mat_mult(a: list[list[int]], b: list[list[int]]) -> list[list[int]]:
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


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Compute base^exp mod mod."""
    result = 1
    base %= mod
    while exp:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def solve() -> int:
    """Solve Problem 422."""
    N = 11**14
    M = 10**9 + 7

    F_N1 = fibonacci(N - 1, M - 1)
    F_N2 = fibonacci(N - 2, M - 1)

    a = -(pow_mod(-3, 2 * F_N1 - 1, M) - pow_mod(2, 4 * F_N2 + 2 * F_N1 - 2, M))
    b = -pow_mod(2, 2 * F_N2 + F_N1 - 2, M) * pow_mod(-3, F_N1 - 1, M) % M
    c = pow_mod(-3, 2 * F_N1 + 1, M) + pow_mod(2, 4 * F_N2 + 2 * F_N1 + 2, M)
    d = pow_mod(2, 2 * F_N2 + F_N1, M) * pow_mod(-3, F_N1, M) % M

    ans = (a + b + c + d) % M
    return (ans + M) % M


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
