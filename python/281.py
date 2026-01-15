"""Project Euler Problem 281: Pizza Toppings.

Let f(m, n) be the number of ways to put m different toppings onto a
pizza with m*n slices, with each topping on exactly n slices and each
slice having exactly one topping, and rotations not considered distinct.
Find the sum of all f(m, n) ≤ N where m≥2 and n≥1.
"""

from __future__ import annotations

from math import gcd, factorial


def nCr_multinomial(g: int, counts: list[int]) -> int:
    """Multinomial coefficient."""
    result = factorial(g)
    for count in counts:
        result //= factorial(count)
    return result


def solve() -> int:
    """Solve Problem 281."""
    N = 10**15
    ans = 0

    m = 2
    while True:
        n = 1
        while True:
            f_val = f(m, n)
            if f_val > N:
                break
            ans += f_val
            n += 1
        if f(m, 1) > N:
            break
        m += 1

    return ans


def f(m: int, n: int) -> int:
    """Compute f(m, n)."""
    total = 0
    for k in range(m * n):
        g = gcd(k, m * n)
        if g % m == 0:
            count_per_topping = g // m
            counts = [count_per_topping] * m
            total += nCr_multinomial(g, counts)
    return total // (m * n)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
