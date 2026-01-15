"""Project Euler Problem 721: High Powers of Irrational Numbers.

Find Σ_{a=1}^N f(a,a²), where f(a,n) = ⌊ (⌈√a⌉+√a)^n ⌋.

Note that S = (⌈√a⌉+√a)^n + (⌈√a⌉-√a)^n is an integer and the second term
is less than 1. So if a is a perfect square, than f(a,n) = S, otherwise
f(a,n) = S-1.

S can be computed directly in Z[√a]. We encode x+y√a as the vector [x,y],
and multiplication in Z[√a] corresponds to matrix multiplication. We can
compute (⌈√a⌉+√a) and then add its conjugate.
"""

from __future__ import annotations

from math import ceil, isqrt


def is_sq(n: int) -> bool:
    """Check if n is a perfect square."""
    root = isqrt(n)
    return root * root == n


def pow2x2(matrix: list[int], exp: int, mod: int) -> list[int]:
    """Raise 2x2 matrix to power exp modulo mod.

    Matrix is represented as [a, b, c, d] for [[a, b], [c, d]].
    """
    a, b, c, d = matrix
    result = [1, 0, 0, 1]  # Identity matrix

    base = [a % mod, b % mod, c % mod, d % mod]
    while exp > 0:
        if exp & 1:
            # Multiply result by base
            new_result = [
                (result[0] * base[0] + result[1] * base[2]) % mod,
                (result[0] * base[1] + result[1] * base[3]) % mod,
                (result[2] * base[0] + result[3] * base[2]) % mod,
                (result[2] * base[1] + result[3] * base[3]) % mod,
            ]
            result = new_result
        # Square base
        new_base = [
            (base[0] * base[0] + base[1] * base[2]) % mod,
            (base[0] * base[1] + base[1] * base[3]) % mod,
            (base[2] * base[0] + base[3] * base[2]) % mod,
            (base[2] * base[1] + base[3] * base[3]) % mod,
        ]
        base = new_base
        exp >>= 1

    return result


def f(a: int, n: int, mod: int) -> int:
    """Compute f(a, n)."""
    c = ceil(a**0.5)
    # Matrix representation of c + √a: [c, a, 1, c]
    matrix = [c, a, 1, c]
    result_matrix = pow2x2(matrix, n, mod)
    # The first element is the integer part
    s = 2 * result_matrix[0]
    if not is_sq(a):
        s -= 1
    return s % mod


def solve() -> int:
    """Solve Problem 721."""
    n = 5 * 10**6
    m = 999999937
    ans = 0
    for a in range(1, n + 1):
        ans = (ans + f(a, a * a, m)) % m
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
